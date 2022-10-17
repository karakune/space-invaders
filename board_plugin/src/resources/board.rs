use crate::bounds::Bounds2;
use crate::components::Coordinates;
use bevy::prelude::*;
// use bevy::utils::HashMap;
// use bevy::log;

#[derive(Debug)]
pub struct Board {
    pub bounds: Bounds2,
    pub entity: Entity,
}

impl Board {
    /// Translates a mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        if !self.bounds.in_bounds(position) {
            return None;    
        }
        // World space to board space
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: coordinates.x,
            y: coordinates.y,
        })
    }
}
