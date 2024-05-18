use crate::components::Piece;
use crate::events::spawn_new_pieces_event::SpawnNewPiecesEvent;
use crate::events::validate_move_event::{ValidateMoveEvent, ValidationType};
use crate::resources::{Score, SelectionInfo};
use bevy::prelude::{Commands, EventReader, EventWriter, Query, ResMut};

pub fn validate_move_event_handler(
    mut validate_move_event_reader: EventReader<ValidateMoveEvent>,
    mut spawn_new_pieces_event_writer: EventWriter<SpawnNewPiecesEvent>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    q_pieces: Query<&Piece>,
    mut selection_info: ResMut<SelectionInfo>,
) {
    for validate_move_event in validate_move_event_reader.read() {
        let validation_type = validate_move_event.validation_type();
        println!("VALIDATING!!!");

        match validation_type {
            ValidationType::PreSpawn => {
                spawn_new_pieces_event_writer.send(SpawnNewPiecesEvent::new(3));
            }
            ValidationType::PostSpawn => {
                selection_info.start_choosing();
            }
        }
    }
}
