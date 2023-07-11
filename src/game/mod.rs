mod actor;
mod audio;
mod enemy;
mod models;
mod physics;
mod player;
mod score;
mod star;
mod systems;
mod ui;
mod utils;

use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, States, Update};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use ui::GameUIPlugin;

use crate::{events::GameOverEvent, AppState};

use self::{
    audio::GameAudioPlugin,
    models::ModelsPlugin,
    physics::PhysicsPlugin,
    systems::{pause_simulation, resume_simulation, toggle_simulation},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_state::<SimulationState>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins((
                PhysicsPlugin,
                ModelsPlugin,
                GameAudioPlugin,
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
                GameUIPlugin,
            ))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
