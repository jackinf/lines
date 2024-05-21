use std::collections::{HashMap, HashSet};

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Camera, Entity, EventReader, EventWriter, MouseButton, Query, ResMut, Transform, Vec2, With,
};
use bevy::utils::petgraph::algo::astar;
use bevy::window::Window;
use petgraph::Graph;

use crate::actions::{is_sprite_clicked_vec3, screen_pos_to_world_pos};
use crate::components::{Piece, Tile};
use crate::constants::{Coord, BALL_SIZE_SCALED, GRID_HEIGHT, GRID_WIDTH, TILE_SIZE_SCALED};
use crate::events::CenterPieceToTileEvent;
use crate::resources::SelectionInfo;

pub fn select_piece(
    q_windows: Query<&Window>,
    q_camera: Query<&Transform, With<Camera>>,
    q_pieces: Query<(Entity, &Transform, &Piece), With<Piece>>,
    q_tiles: Query<(Entity, &Transform, &Tile), With<Tile>>,
    mut selection_info: ResMut<SelectionInfo>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut center_piece_to_tile_event_writer: EventWriter<CenterPieceToTileEvent>,
) {
    if !selection_info.is_choosing() {
        return;
    }

    let window = q_windows.single();
    let camera = q_camera.single();

    for event in mouse_button_input_events.read() {
        let world_pos = screen_pos_to_world_pos(window, camera);
        if world_pos.is_none() {
            continue;
        }
        let world_pos = world_pos.unwrap();

        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            let clicked_piece = q_pieces
                .iter()
                .find(|(_, transform, _)| {
                    is_sprite_clicked_vec3(transform.translation, world_pos, BALL_SIZE_SCALED)
                })
                .map(|(entity, _, _)| entity);

            let tile_clicked = q_tiles.iter().find(|(_, transform, _)| {
                is_sprite_clicked_vec3(transform.translation, world_pos, TILE_SIZE_SCALED)
            });
            if tile_clicked.is_none() {
                continue;
            }
            let (tile_id, tile_transform, tile) = tile_clicked.unwrap();

            match (clicked_piece, selection_info.selected()) {
                (Some(clicked_piece_id), Some(selected_piece_id))
                    if clicked_piece_id == selected_piece_id =>
                {
                    let piece_id = selected_piece_id;
                    selection_info.deselect();
                    center_piece_to_tile_event_writer
                        .send(CenterPieceToTileEvent::new(tile_id, piece_id));
                }
                (Some(clicked_piece_id), _) => {
                    selection_info.select(clicked_piece_id);
                }
                (None, Some(selected_piece_id)) => {
                    let selected_piece = q_pieces.get(selected_piece_id);
                    if selected_piece.is_err() {
                        continue;
                    }
                    let (_, _, piece) = selected_piece.unwrap(); // safe to unwrap here
                    let from = piece.coord();
                    let to = tile.coord();

                    let path = compute_path(&q_pieces, from, to);
                    if path.is_empty() {
                        continue;
                    }

                    let world_path = convert_path(path, &q_tiles);

                    selection_info.set_dest_coord(to); // deferring setting the coord until the end
                    selection_info.set_path(world_path);
                    selection_info.start_moving();
                }
                _ => {}
            }
        }
    }
}

fn compute_path(
    q_pieces: &Query<(Entity, &Transform, &Piece), With<Piece>>,
    start: Coord,
    goal: Coord,
) -> Vec<Coord> {
    // Define the grid size
    let grid_width = GRID_WIDTH;
    let grid_height = GRID_HEIGHT;

    // Set of non-traversable cells
    let pieces_set: HashSet<Coord> = q_pieces
        .iter()
        .map(|(_, _, piece)| piece.coord())
        .filter(|&coord| coord != start && coord != goal)
        .collect();

    // Create a graph
    let mut graph = Graph::<Coord, ()>::new();

    // Create a 2D vector to store node indices
    let mut node_indices = vec![vec![None; grid_width]; grid_height];

    // Add nodes to the graph
    for y in 0..grid_height {
        for x in 0..grid_width {
            let coord: Coord = (x, y);
            if !pieces_set.contains(&coord) {
                let node = graph.add_node(coord);
                node_indices[y][x] = Some(node);
            }
        }
    }

    // Add edges between adjacent nodes
    for y in 0..grid_height {
        for x in 0..grid_width {
            if let Some(node) = node_indices[y][x] {
                let neighbors = [
                    (x.wrapping_sub(1), y), // left
                    (x + 1, y),             // right
                    (x, y.wrapping_sub(1)), // up
                    (x, y + 1),             // down
                ];
                for &(nx, ny) in &neighbors {
                    if nx < grid_width && ny < grid_height {
                        if let Some(neighbor_node) = node_indices[ny][nx] {
                            graph.add_edge(node, neighbor_node, ());
                        }
                    }
                }
            }
        }
    }

    // Example: Running A* algorithm from start to goal
    if let (Some(start_node), Some(goal_node)) =
        (node_indices[start.1][start.0], node_indices[goal.1][goal.0])
    {
        let result = astar(
            &graph,
            start_node,
            |finish| finish == goal_node,
            |_| 1,
            |_| 0,
        );

        let path_coords = result.map(|(_cost, path)| {
            path.into_iter()
                .map(|node| *graph.node_weight(node).unwrap())
                .collect::<Vec<Coord>>()
        });

        if let Some(coords) = path_coords {
            return coords;
        } else {
            return vec![];
        }
    } else {
        println!("Invalid start or goal");
        return vec![];
    }
}

fn convert_path(
    path: Vec<Coord>,
    q_tiles: &Query<(Entity, &Transform, &Tile), With<Tile>>,
) -> Vec<Vec2> {
    let sub_result: HashMap<Coord, Vec2> = q_tiles
        .iter()
        .filter_map(|(_, transform, tile)| {
            if path.contains(&tile.coord()) {
                let coord = tile.coord();
                let world_pos = transform.translation.truncate();
                Some((coord, world_pos))
            } else {
                None
            }
        })
        .collect();

    let world_path = path
        .iter()
        // TODO: if there's at least one None, return an empty Vec
        .map(|coord| sub_result.get(coord).unwrap().clone())
        .collect();

    world_path
}
