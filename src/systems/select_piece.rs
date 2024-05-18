use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{
    Camera, Entity, EventReader, MouseButton, Query, ResMut, Transform, Vec2, Vec3Swizzles, With,
};
use bevy::utils::petgraph::algo::astar;
use bevy::window::Window;
use petgraph::Graph;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::actions::{is_sprite_clicked_vec3, screen_pos_to_world_pos};
use crate::components::{Piece, Tile};
use crate::constants::{
    Coord, BALL_SIZE_SCALED, GRID_HEIGHT, GRID_WIDTH, TILES_HORIZONTAL, TILES_VERTICAL,
    TILE_SIZE_SCALED,
};
use crate::resources::SelectionInfo;

pub fn select_piece(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    q_windows: Query<&Window>,
    q_camera: Query<&Transform, With<Camera>>,
    q_pieces: Query<(Entity, &Transform, &Piece), With<Piece>>,
    q_tiles: Query<(&Transform, &Tile), With<Tile>>,
    mut selection_info: ResMut<SelectionInfo>,
) {
    let window = q_windows.single();
    let camera = q_camera.single();

    for event in mouse_button_input_events.read() {
        let world_pos = screen_pos_to_world_pos(window, camera);
        if world_pos.is_none() {
            continue;
        }
        let world_pos = world_pos.unwrap();

        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            let piece_clicked = q_pieces
                .iter()
                .find(|(entity, transform, _)| {
                    is_sprite_clicked_vec3(transform.translation, world_pos, BALL_SIZE_SCALED)
                })
                .map(|(entity, _, _)| entity);

            let tile_clicked = q_tiles.iter().find(|(transform, _)| {
                is_sprite_clicked_vec3(transform.translation, world_pos, TILE_SIZE_SCALED)
            });

            match (tile_clicked, piece_clicked, selection_info.selected()) {
                (Some(_), Some(piece), Some(selected_piece)) if piece == selected_piece => {
                    selection_info.deselect();
                    println!("Piece deselected: {}", piece.index());
                }
                (Some(_), Some(piece), _) => {
                    selection_info.select(piece);
                    println!("Piece selected: {}", piece.index());
                }
                (Some(tile_clicked), None, Some(selected_piece)) => {
                    let (transform, tile) = tile_clicked;
                    // TODO: deal with unwrap in a safe manner
                    let (_, _, piece) = q_pieces.get(selected_piece).unwrap();
                    let from = piece.coord();
                    let to = tile.coord();

                    println!(
                        "Calculating path ({}.{}) -> ({}.{})",
                        from.0, from.1, to.0, to.1
                    );

                    let path = compute_path(&q_pieces, from, to);
                    println!("Found a path: {:?}", path);
                    // selection_info.set_path(path);
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
            // println!("Found a path: {:?}", coords);
            return coords;
        } else {
            return vec![];
        }
    } else {
        println!("Invalid start or goal");
        return vec![];
    }
}

fn convert_path(path: Vec<Coord>) -> Vec<Vec2> {
    todo!()
    // path.iter().map(|(x, y)| Vec2Wrapper::new(*x as f32, *y as f32)).collect()
}
