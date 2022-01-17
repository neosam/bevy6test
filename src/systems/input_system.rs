use bevy::prelude::*;

use crate::events;
use crate::resources;

pub fn input_system(
    mut input: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut output: EventWriter<events::InputEvent>,
    mut input_store: ResMut<resources::InputStore>
) {
    for event in input.iter() {
        if event.state.is_pressed() {
            if let Some(input_event) = match event.key_code {
                Some(KeyCode::W) => Some(events::InputEvent::MoveNorth),
                Some(KeyCode::S) => Some(events::InputEvent::MoveSouth),
                Some(KeyCode::A) => Some(events::InputEvent::MoveWest),
                Some(KeyCode::D) => Some(events::InputEvent::MoveEast),
                _ => None
            } {
                match input_event {
                    events::InputEvent::MoveNorth => input_store.player_move_north = true,
                    events::InputEvent::MoveSouth => input_store.player_move_south = true,
                    events::InputEvent::MoveEast => input_store.player_move_east = true,
                    events::InputEvent::MoveWest => input_store.player_move_west = true,
                    _ => ()
                }
                output.send(input_event);
            }
        } else {
            if let Some(input_event) = match event.key_code {
                Some(KeyCode::W) => Some(events::InputEvent::MoveNorth),
                Some(KeyCode::S) => Some(events::InputEvent::MoveSouth),
                Some(KeyCode::A) => Some(events::InputEvent::MoveWest),
                Some(KeyCode::D) => Some(events::InputEvent::MoveEast),
                _ => None
            } {
                match input_event {
                    events::InputEvent::MoveNorth => input_store.player_move_north = false,
                    events::InputEvent::MoveSouth => input_store.player_move_south = false,
                    events::InputEvent::MoveEast => input_store.player_move_east = false,
                    events::InputEvent::MoveWest => input_store.player_move_west = false,
                    _ => ()
                }
                if !input_store.player_move_north && !input_store.player_move_south
                    && !input_store.player_move_east && !input_store.player_move_west {
                        output.send(events::InputEvent::Stop);
                    }
            }
        }
    }
}