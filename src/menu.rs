use bevy::app::AppExit;
use bevy::prelude::*;

use crate::GameState;
use crate::settings::Settings;

#[derive(Component)]
enum MenuButton {
    StartGame,
    Quit,
    ToggleDemo,
}

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), (spawn_camera_2d, spawn_buttons));
    app.add_systems(Update, handle_buttons.run_if(in_state(GameState::Menu)));
}

fn spawn_camera_2d(mut commands: Commands) {
    commands.spawn((DespawnOnExit(GameState::Menu), Camera2d));
}

fn handle_buttons(
    buttons: Query<(&MenuButton, &Interaction, &Children), Changed<Interaction>>,
    mut text_color: Query<&mut TextColor>,
    mut game_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<Settings>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (button, interaction, children) in buttons {
        match button {
            MenuButton::StartGame => if interaction == &Interaction::Pressed { game_state.set(GameState::Game) },
            MenuButton::Quit => if interaction == &Interaction::Pressed {
                app_exit.write(AppExit::Success);
            },
            MenuButton::ToggleDemo => if interaction == &Interaction::Pressed {
                settings.demo_mode = !settings.demo_mode;
                for child in children.iter() {
                    if let Ok(mut text_color_component) = text_color.get_mut(child) {
                        text_color_component.0 = get_demo_mode_style(settings.demo_mode)
                    }
                }
            },
        }
    }
}

pub fn spawn_buttons(mut commands: Commands, settings: Res<Settings>) {
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
            ),
            (
                Visibility::default(),
                Transform::default(),
                MenuButton::ToggleDemo,
                Button,
                children![(
                    Text::new("Demo Mode"),
                    TextColor(get_demo_mode_style(settings.demo_mode))
                )]
            )
        ],
    ));
}

fn get_demo_mode_style(demo_enabled: bool) -> Color {
    Color::Srgba(Srgba::hex(if demo_enabled { "#00FF00" } else { "#FF0000" }).unwrap())
}
