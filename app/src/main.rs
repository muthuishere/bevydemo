use bevy::prelude::*;
use platform::config::AppConfig;
use gameplay::{app_states::AppState, level_plugin::LevelPlugin};
use ui::{hud_plugin::HudPlugin, menu_plugin::MenuPlugin, results_plugin::ResultsPlugin};
use audio_plugin::MathQuestAudioPlugin;
use physics_plugin::MathQuestPhysicsPlugin;

fn main() {
    let _config = AppConfig::from_env();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MathQuest".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(LevelPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(ResultsPlugin)
        .add_plugins(MathQuestAudioPlugin)
        .add_plugins(MathQuestPhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Boot), transition_to_menu)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn transition_to_menu(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}
