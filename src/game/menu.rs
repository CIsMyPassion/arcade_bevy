use bevy::prelude::*;
use bevy::app::AppExit;
use crate::game::{button_colors, GameState};

#[derive(Component)]
struct MenuItem;

#[derive(Component, Clone, Copy)]
enum MenuButton {
    Asteroids,
    Settings,
    Quit,
}

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(on_enter)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(on_exit)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(handle_buttons)
            );
    }
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font: Handle<Font> = asset_server.load("fonts/Regular.ttf");

    commands
        .spawn_bundle(UiCameraBundle::default())
            .insert(MenuItem);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(MenuItem)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                ..Default::default()
                            },
                            color: Color::rgb(0.5, 0.5, 0.5).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            spawn_button(parent, font.clone(), MenuButton::Asteroids);
                            spawn_button(parent, font.clone(), MenuButton::Settings);
                            spawn_button(parent, font.clone(), MenuButton::Quit);
                        });

                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::FlexEnd,
                        justify_content: JustifyContent::Center,
                        border: Rect::all(Val::Px(50.0)),
                        ..Default::default()
                    },
                    color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                   parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Arcade Games",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 80.0,
                                    color: Color::rgb(0.9, 0.9, 0.9)
                                },
                                Default::default()
                            ),
                            ..Default::default()
                        });
                });
        });
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

fn handle_buttons(
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<(&Interaction, &mut UiColor, &MenuButton)>,
) {

    query.for_each_mut(|(interaction, mut color, menu_button)| match interaction {
        Interaction::Clicked => {

            match menu_button {
                MenuButton::Asteroids =>
                    game_state.set(GameState::Asteroids).unwrap(),
                MenuButton::Settings =>
                    game_state.set(GameState::Settings).unwrap(),
                MenuButton::Quit =>
                    app_exit_events.send(AppExit),
            }

            *color = button_colors::NORMAL_BUTTON.into();
        }
        Interaction::Hovered => {
            *color = button_colors::HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = button_colors::NORMAL_BUTTON.into();
        }
    });
}

fn spawn_button(commands: &mut ChildBuilder, font: Handle<Font>, button_type: MenuButton) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(170.0), Val::Px(50.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: button_colors::NORMAL_BUTTON.into(),
        ..Default::default()
    })
    .insert(button_type)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                match button_type {
                    MenuButton::Asteroids => "Asteroids",
                    MenuButton::Settings => "Settings",
                    MenuButton::Quit => "Quit",
                },
                TextStyle {
                    font: font.clone(),
                    font_size: 35.0,
                    color: Color::rgb(0.9, 0.9, 0.9)
                },
                Default::default()
            ),
            ..Default::default()
        });
    });
}
