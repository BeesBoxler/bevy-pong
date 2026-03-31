use bevy::app::AppExit;
use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
enum MenuButton {
    StartGame,
    Quit,
}

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), (spawn_camera_2d, spawn_buttons));
    app.add_systems(Update, handle_buttons.run_if(in_state(GameState::Menu)));
}

fn spawn_camera_2d(mut commands: Commands) {
    commands.spawn((DespawnOnExit(GameState::Menu), Camera2d));
}

fn handle_buttons(
    buttons: Query<(&MenuButton, &Interaction), Changed<Interaction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (button, interaction) in buttons {
        match button {
            MenuButton::StartGame => match interaction {
                Interaction::Pressed => game_state.set(GameState::Game),
                _ => (),
            },
            MenuButton::Quit => match interaction {
                Interaction::Pressed => {
                    app_exit.write(AppExit::Success);
                }
                _ => (),
            },
        }
    }
}

pub fn spawn_buttons(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Visibility::Visible,
        Transform::default(),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(25),
            ..default()
        },
        children![
            (
                Visibility::default(),
                Transform::default(),
                MenuButton::StartGame,
                Button,
                Node {
                    width: px(150),
                    height: px(50),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::MAX,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::BLACK),
                children![(Text::new("Start Game"))]
            ),
            (
                Visibility::default(),
                Transform::default(),
                MenuButton::Quit,
                Button,
                Node {
                    width: px(150),
                    height: px(50),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::MAX,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::BLACK),
                children![(Text::new("Exit"))]
            )
        ],
    ));
}
