use crate::events::SpawnNewPiecesEvent;
use bevy::prelude::EventWriter;

pub fn start(mut spawn_new_pieces_event_writer: EventWriter<SpawnNewPiecesEvent>) {
    spawn_new_pieces_event_writer.send(SpawnNewPiecesEvent::new(5));
}
