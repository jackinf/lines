use std::collections::{HashMap, HashSet};

use bevy::math::Vec2;
use bevy::prelude::{EventReader, Query, ResMut, Transform, With};
use petgraph::algo::astar;
use petgraph::Graph;

use crate::components::{Piece, Tile};
use crate::constants::{Coord, GRID_HEIGHT, GRID_WIDTH};
use crate::events::CalculateMovementPathEvent;
use crate::resources::SelectionInfo;

pub fn calculate_movement_path_event_handler(
    q_pieces: Query<&Piece, With<Piece>>,
    q_tiles: Query<(&Transform, &Tile), With<Tile>>,
    mut selection_info: ResMut<SelectionInfo>,
    mut calculate_movement_path_events: EventReader<CalculateMovementPathEvent>,
) {
    for event in calculate_movement_path_events.read() {
        let piece_id = event.piece_id();
        let tile_id = event.target_tile_id();

        let selected_piece = q_pieces.get(piece_id);
        if selected_piece.is_err() {
            return;
        }
        let selected_piece = selected_piece.unwrap(); // safe to unwrap here

        let tile = q_tiles.get(tile_id);
        if tile.is_err() {
            return;
        }
        let (_, tile) = tile.unwrap(); // safe to unwrap here

        let from = selected_piece.coord();
        let to = tile.coord();

        let path = compute_path(&q_pieces, from, to);
        if path.is_empty() {
            return;
        }

        let world_path = convert_path_from_coord_to_world(path, &q_tiles);

        selection_info.set_dest_coord(to); // deferring setting the coord until the end
        selection_info.set_path(world_path);
        selection_info.start_moving();
    }
}

fn compute_path(q_pieces: &Query<&Piece, With<Piece>>, start: Coord, goal: Coord) -> Vec<Coord> {
    // Define the grid size
    let grid_width = GRID_WIDTH;
    let grid_height = GRID_HEIGHT;

    // Set of non-traversable cells
    let pieces_set: HashSet<Coord> = q_pieces
        .iter()
        .map(|piece| piece.coord())
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
                // neighbors
                let left = (x.wrapping_sub(1), y);
                let right = (x + 1, y);
                let up = (x, y.wrapping_sub(1));
                let down = (x, y + 1);

                for &(nx, ny) in &[left, right, up, down] {
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

fn convert_path_from_coord_to_world(
    path: Vec<Coord>,
    q_tiles: &Query<(&Transform, &Tile), With<Tile>>,
) -> Vec<Vec2> {
    let sub_result: HashMap<Coord, Vec2> = q_tiles
        .iter()
        .filter_map(|(transform, tile)| {
            if path.contains(&tile.coord()) {
                let coord = tile.coord();
                let world_pos = transform.translation.truncate();
                Some((coord, world_pos))
            } else {
                None
            }
        })
        .collect();

    let world_path: Vec<_> = path
        .iter()
        .map(|coord| sub_result.get(coord).cloned())
        .collect::<Option<Vec<_>>>()
        .unwrap_or_else(Vec::new);

    world_path
}
