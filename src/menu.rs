use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
enum MenuButton {
    StartGame,
}

pub fn main_menu(app: &mut App) {
    app.add_systems(Startup, (spawn_camera_2d, spawn_buttons));
    app.add_systems(Update, handle_buttons.run_if(in_state(GameState::Menu)));
}

fn spawn_camera_2d(mut commands: Commands) {
    commands.spawn((DespawnOnExit(GameState::Menu), Camera2d));
}

fn handle_buttons(
    button: Single<(&MenuButton, &Interaction), Changed<Interaction>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (button, interaction) = button.into_inner();

    match button {
        MenuButton::StartGame => match interaction {
            Interaction::Pressed => game_state.set(GameState::Game),
            _ => (),
        },
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
            ..default()
        },
        children![(
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
        )],
    ));
}
