use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

/// Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    /// Centered board
    Centered { offset: Vec3 },
    /// Custom position
    Custom(Vec3),
}

/// Board generation options. Must be used as a resource
// We use serde to allow saving option presets and loading them at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardOptions {
    /// Board world position
    pub position: BoardPosition
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            position: Default::default()
        }
    }
}