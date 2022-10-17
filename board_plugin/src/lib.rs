mod bounds;
mod components;
mod events;
pub mod resources;
mod systems;

use bevy::{prelude::*, log};
use bevy::ecs::schedule::StateData;
use bevy::math::Vec3Swizzles;
use bounds::Bounds2;
use resources::*;
use systems::*;

use crate::events::*;


pub struct BoardPlugin<T> {
    pub running_state: T
}

impl<T: StateData> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        // When the running states comes into the stack we load a board
        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_board),
        )
        // We handle input and trigger events only if the state is active
        .add_system_set(
            SystemSet::on_update(self.running_state.clone())
                .with_system(systems::input::input_handling)
        );
        // // We handle uncovering even if the state is inactive
        // .add_system_set(
        //     SystemSet::on_in_stack_update(self.running_state.clone())
        //         .with_system(systems::uncover::uncover_tiles)
        //         .with_system(systems::mark::mark_tiles)
        // )
        // .add_system_set(
        //     SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_board)
        // )
        // .add_event::<TileTriggerEvent>()
        // .add_event::<TileMarkEvent>()
        // .add_event::<BombExplosionEvent>()
        // .add_event::<BoardCompletedEvent>();
    }
}

impl<T> BoardPlugin<T> {
    fn create_board(mut commands: Commands, board_assets: Res<BoardAssets>, options: Res<BoardOptions>) {
        let board_size = Vec2::new(
            500 as f32,
            800 as f32
        );
        log::info!("board size: {}", board_size);
        
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };
        
        let board_entity = commands.spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.board_material.color,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    texture: board_assets.board_material.texture.clone(),
                    transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    ..Default::default()
                })
                .insert(Name::new("Background"));

                Self::spawn_player(parent, &board_assets, 50., 25., board_size.x / 2., board_size.y / 5.);
                
                let covers_count = 4;
                let cover_width = 50.;
                let start_x = cover_width / -2. + board_size.x / covers_count as f32;
                Self::spawn_covers(parent, &board_assets, covers_count, cover_width, 25., start_x, board_size.y / 4.);
                
                let rows = 5;
                let aliens_per_row = 12;
                let alien_width = 25.;
                let start_x = alien_width / -2. + board_size.x / aliens_per_row as f32;
                let start_y = board_size.y / 3.;
                Self::spawn_aliens(parent, &board_assets, rows, aliens_per_row, alien_width, 25., start_x, start_y);
            })
            .id();

        commands.insert_resource(Board {
            bounds: Bounds2 { 
                position: board_position.xy(),
                size: board_size ,
            },
            entity: board_entity
        });
    }

    fn spawn_player(parent: &mut ChildBuilder, board_assets: &BoardAssets, width: f32, height: f32, start_x: f32, start_y: f32) {
        let mut cmd = parent.spawn();
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: board_assets.tank.color,
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            texture: board_assets.tank.texture.clone(),
            transform: Transform::from_xyz(start_x, start_y, 0.),
            ..Default::default()
        })
        .insert(Name::new("Tank"));
    }

    fn spawn_covers(parent: &mut ChildBuilder, board_assets: &BoardAssets, count: i32, width: f32, height: f32, start_x: f32, start_y: f32) {
        for i in 1..count+1 {
            let mut cmd = parent.spawn();
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: board_assets.cover.color,
                    custom_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
                texture: board_assets.cover.texture.clone(),
                transform: Transform::from_xyz(start_x * i as f32, start_y, 0.),
                ..Default::default()
            })
            .insert(Name::new(format!("Cover {}", i)));
        }
    }

    fn spawn_aliens(parent: &mut ChildBuilder, board_assets: &BoardAssets, rows: i32, count_per_row: i32, width: f32, height: f32, start_x: f32, start_y: f32) {
        for row in 1..rows+1 {
            for i in 1..count_per_row+1 {
                let mut cmd = parent.spawn();
                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.alien.color,
                        custom_size: Some(Vec2::new(width, height)),
                        ..Default::default()
                    },
                    texture: board_assets.alien.texture.clone(),
                    transform: Transform::from_xyz(start_x + (i * 34) as f32, start_y + (row * 30) as f32, 0.),
                    ..Default::default()
                })
                .insert(Name::new(format!("Alien ({}, {})", i, row)));
            }
        }
    }
}