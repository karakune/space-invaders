use bevy::prelude::*;
use bevy::log;
use board_plugin::BoardPlugin;
use board_plugin::resources::{BoardAssets, SpriteMaterial};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out
}

fn main() {
    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Space invaders!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // Startup system (cameras)
    app

        .add_state(AppState::Out)
        .add_startup_system(setup_board)
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame
        })
        .add_system(state_handler);
    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board assets
    // commands.insert_resource(BoardAssets {
    //     label: "Default".to_string(),
    //     board_material: SpriteMaterial {
    //         color: Color::WHITE,
    //         ..Default::default()
    //     }
    // });
    // Plugin activation
    state.set(AppState::InGame).unwrap();
}