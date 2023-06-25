mod game_over_menu;
mod hud;
mod pause_menu;

use bevy::prelude::{App, Plugin};
use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugin(HudPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(GameOverMenuPlugin);
    }
}
