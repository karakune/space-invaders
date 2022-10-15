mod events;
pub mod resources;

use bevy::prelude::*;
use bevy::ecs::schedule::StateData;

use crate::events::*;

pub struct BoardPlugin<T> {
    pub running_state: T
}

impl<T: StateData> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        
    }
}