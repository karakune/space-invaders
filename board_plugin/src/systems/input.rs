use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    mut button_evr: EventReader<KeyboardInput>,
) {
    for event in button_evr.iter() {
        if let ElementState::Pressed = event.state {
            match event.key_code {
                Some(keycode) => {
                    match keycode {
                        KeyCode::A => {
                            log::info!("Pressed A!");
                        }
                        KeyCode::D => {
                            log::info!("Pressed D!");
                            
                        }
                        KeyCode::Space => {
                            log::info!("Pressed Space!");
                            
                        }
                        _ => ()
                    }
                },
                None => ()
            }
        }
    }
}