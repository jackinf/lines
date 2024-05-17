use crate::resources::Board;
use crate::types::{PieceColor, PieceInfo};
use bevy::math::Vec2;
use bevy::prelude::{Color, ResMut};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::HashSet;

pub fn create_seed_pieces(mut board: ResMut<Board>) {
    let mut rng = rand::thread_rng();
    let mut positions = HashSet::new();

    // Generate 5 unique positions
    for _ in 0..5 {
        let (x, y) = generate_position(&mut rng, &mut positions);
        let piece_color = PieceColor::choose_piece_color();
        let piece = PieceInfo::new(Vec2::new(x as f32, y as f32), piece_color);
        board.push(piece);
    }
}

/// Function to generate a random position ensuring no overlap
fn generate_position(rng: &mut ThreadRng, positions: &mut HashSet<(i32, i32)>) -> (i32, i32) {
    loop {
        let x = rng.gen_range(0..9);
        let y = rng.gen_range(0..9);
        let position = (x, y);
        if positions.insert(position) {
            return position;
        }
    }
}
