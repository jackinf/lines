use crate::components::Piece;
use crate::constants::{Coord, GRID_HEIGHT, GRID_WIDTH};
use crate::events::spawn_new_pieces_event::SpawnNewPiecesEvent;
use crate::events::validate_move_event::{NextPlannedMove, ValidateMoveEvent};
use crate::resources::{Score, SelectionInfo};
use crate::types::PieceColor;
use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, ResMut, With};
use bevy::utils::HashMap;
use std::collections::HashSet;

pub fn validate_move_event_handler(
    mut validate_move_event_reader: EventReader<ValidateMoveEvent>,
    mut spawn_new_pieces_event_writer: EventWriter<SpawnNewPiecesEvent>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    q_pieces: Query<(Entity, &Piece), With<Piece>>,
    mut selection_info: ResMut<SelectionInfo>,
) {
    for validate_move_event in validate_move_event_reader.read() {
        let mut next_planned_move = validate_move_event.next_planned_move();

        let mut piece_map = q_pieces
            .iter()
            .map(|(_, piece)| (piece.coord(), piece.piece_color()))
            .collect::<HashMap<Coord, PieceColor>>();

        let (total_score, matched_pieces) = score_and_find_matched_pieces(&piece_map, 9);
        if matched_pieces.len() > 0 {
            next_planned_move = NextPlannedMove::Play;
        }

        matched_pieces
            .iter()
            .filter_map(|coord| q_pieces.iter().find(|(_, piece)| piece.coord() == *coord))
            .for_each(|(entity, _)| {
                commands.entity(entity).despawn();
            });

        println!("Total score: {}", total_score);
        println!("Matched pieces: {:?}", matched_pieces);

        match next_planned_move {
            NextPlannedMove::SpawnPieces => {
                spawn_new_pieces_event_writer.send(SpawnNewPiecesEvent::new(3));
            }
            NextPlannedMove::Play => {
                selection_info.start_choosing();
            }
        }
    }
}

fn score_and_find_matched_pieces(
    piece_map: &HashMap<Coord, PieceColor>,
    grid_size: usize,
) -> (i32, HashSet<Coord>) {
    let mut score = 0;
    let mut matched_pieces = HashSet::new();
    let mut visited = HashSet::new();

    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Some(&ref color) = piece_map.get(&(x, y)) {
                if !visited.contains(&(x, y)) {
                    // Check right
                    score += check_direction(
                        piece_map,
                        &mut matched_pieces,
                        &mut visited,
                        &color,
                        x,
                        y,
                        1,
                        0,
                        grid_size,
                    );
                    // Check down
                    score += check_direction(
                        piece_map,
                        &mut matched_pieces,
                        &mut visited,
                        &color,
                        x,
                        y,
                        0,
                        1,
                        grid_size,
                    );
                    // Check diagonal right-down
                    score += check_direction(
                        piece_map,
                        &mut matched_pieces,
                        &mut visited,
                        &color,
                        x,
                        y,
                        1,
                        1,
                        grid_size,
                    );
                    // Check diagonal right-up
                    score += check_direction(
                        piece_map,
                        &mut matched_pieces,
                        &mut visited,
                        &color,
                        x,
                        y,
                        1,
                        -1,
                        grid_size,
                    );
                }
            }
        }
    }

    (score, matched_pieces)
}

fn check_direction(
    piece_map: &HashMap<Coord, PieceColor>,
    matched_pieces: &mut HashSet<Coord>,
    visited: &mut HashSet<Coord>,
    color: &PieceColor,
    start_x: usize,
    start_y: usize,
    dir_x: isize,
    dir_y: isize,
    grid_size: usize,
) -> i32 {
    let mut length = 0;
    let mut score = 0;
    let mut temp_pieces = Vec::new();

    for i in 0..grid_size {
        let x = start_x as isize + i as isize * dir_x;
        let y = start_y as isize + i as isize * dir_y;

        if x >= 0 && x < grid_size as isize && y >= 0 && y < grid_size as isize {
            let coord = (x as usize, y as usize);
            if piece_map.get(&coord) == Some(&color) && !visited.contains(&coord) {
                length += 1;
                temp_pieces.push(coord);
            } else {
                if length >= 5 {
                    score += calculate_score(length);
                    matched_pieces.extend(&temp_pieces);
                    visited.extend(&temp_pieces);
                }
                length = 0;
                temp_pieces.clear();
            }
        } else {
            break;
        }
    }

    if length >= 5 {
        score += calculate_score(length);
        matched_pieces.extend(&temp_pieces);
        visited.extend(temp_pieces);
    }

    score
}
fn calculate_score(length: usize) -> i32 {
    match length {
        5 => 10,
        6 => 12,
        7 => 14,
        8 => 16,
        9 => 18,
        _ => 0,
    }
}
