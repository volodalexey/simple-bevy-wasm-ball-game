use bevy::app::AppExit;
use bevy::prelude::{
    default, info, Color, Commands, DirectionalLight, DirectionalLightBundle, EventReader,
    EventWriter, Input, KeyCode, NextState, Query, Res, ResMut, State, Transform, Vec3, With,
};
use bevy::window::{PrimaryWindow, Window};

use crate::events::GameOverEvent;
use crate::AppState;

pub const CAMERA_FAR: f32 = 1000.0;

pub fn spawn_lights(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 25000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, CAMERA_FAR)
            .looking_at(
                Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
                Vec3::Y,
            ),
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.eq(&AppState::Game) {
            app_state_next_state.set(AppState::Game);
            info!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.eq(&AppState::MainMenu) {
            app_state_next_state.set(AppState::MainMenu);
            info!("Entered AppState::MainMenu");
        }
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for _event in game_over_event_reader.iter() {
        app_state_next_state.set(AppState::GameOver);
        info!("Entered AppState::GameOver");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
