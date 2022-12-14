use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;

/// Material of a `Sprite` with a texture and color
#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}

/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone)]
pub struct BoardAssets {
    /// Label
    pub label: String,
    /// Board background
    pub board_material: SpriteMaterial,
    pub tank: SpriteMaterial,
    pub cover: SpriteMaterial,
    pub alien: SpriteMaterial,
}

impl BoardAssets {
    /// Default color set
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::PURPLE,
        ]
    }
}