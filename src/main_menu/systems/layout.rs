use bevy::{
    prelude::{
        default, AssetServer, BuildChildren, ButtonBundle, Commands, DespawnRecursiveExt, Entity,
        ImageBundle, NodeBundle, Query, Res, TextBundle, With,
    },
    text::{Text, TextAlignment, TextSection},
};
use bevy_ui_styled::styled;

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
use crate::main_menu::components::QuitButton;

use crate::main_menu::{
    components::{MainMenu, PlayButton},
    styles::{get_button_text_style, get_title_text_style, NORMAL_BUTTON_COLOR},
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: styled!("flex-col justify-center items-center w-full h-full gap-16"),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: styled!("flex-row justify-center items-center"),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: styled!("grow-0 shrink-0 m-8"),
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });
                    parent
                        .spawn(NodeBundle {
                            style: styled!(
                                "grow shrink-0 basis-200 flex-row justify-center items-center"
                            ),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Bevy Ball Game",
                                        get_title_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    parent.spawn(ImageBundle {
                        style: styled!("grow-0 shrink-0 m-8"),
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: styled!("flex-row justify-center items-center w-200 h-80"),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            #[cfg(not(target_arch = "wasm32"))]
            parent
                .spawn((
                    ButtonBundle {
                        style: styled!("flex-row justify-center items-center w-200 h-80"),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
