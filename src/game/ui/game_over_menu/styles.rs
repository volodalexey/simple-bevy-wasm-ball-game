use bevy::{
    prelude::{AssetServer, Color, Res},
    text::TextStyle,
    ui::{AlignItems, Display, FlexDirection, JustifyContent, PositionType, Style, Val},
};

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn game_over_menu_style() -> Style {
    Style {
        position_type: PositionType::Absolute, // Needed to display separately from HUD.
        display: Display::Flex,                // Hidden by Default
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Style::DEFAULT
    }
}

pub fn game_over_menu_container_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(400.0),
        height: Val::Px(400.0),
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        ..Style::DEFAULT
    }
}

pub fn button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 48.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}
