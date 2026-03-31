use crate::{GameState, W_HEIGHT, W_WIDTH, settings::Settings};
use bevy::{math::ops::abs, prelude::*};
use rand::{Rng, rng};
use std::ops::Range;

pub fn pong_game(app: &mut App) {
    app.init_resource::<Score>();
    app.add_systems(
        OnEnter(GameState::Game),
        (spawn_camera_2d, spawn_paddles, spawn_ball, draw_score),
    );
    app.add_systems(
        Update,
        (update_paddles, update_ball, update_score, handle_esc_press)
            .run_if(in_state(GameState::Game)),
    );
}

const PADDLE_SPEED: f32 = 100.;

#[derive(Debug, Component, Default)]
#[require(Sprite)]
struct Paddle;

#[derive(Component, PartialEq, Debug)]
enum Player {
    One,
    Two,
    Ai,
}

#[derive(Component, Debug)]
#[require(Sprite)]
struct Ball(Vec2);

#[derive(Resource, Debug, Default)]
struct Score {
    p1: usize,
    p2: usize,
}

impl ToString for Score {
    fn to_string(&self) -> String {
        format!("{} | {}", self.p1, self.p2)
    }
}

fn spawn_camera_2d(mut commands: Commands) {
    commands.spawn((DespawnOnExit(GameState::Game), Camera2d));
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Ball(Vec2 { x: 100., y: 0. }),
        Sprite {
            color: Color::default(),
            custom_size: Some(Vec2::new(30., 30.)),
            ..Default::default()
        },
    ));
}

fn spawn_paddles(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Paddle,
        Sprite {
            color: Color::Srgba(Srgba::hex("#0099FF").unwrap()),
            custom_size: Some(Vec2::new(10., 150.)),
            ..Default::default()
        },
        Transform {
            translation: Vec3 {
                x: W_WIDTH / 2. - 20.,
                y: 0.,
                z: 0.,
            },
            ..Default::default()
        },
        if settings.demo_mode {
            Player::Ai
        } else {
            Player::One
        },
    ));

    commands.spawn((
        DespawnOnExit(GameState::Game),
        Paddle,
        Sprite {
            color: Color::Srgba(Srgba::hex("#FF0099").unwrap()),
            custom_size: Some(Vec2::new(10., 150.)),
            ..Default::default()
        },
        Transform {
            translation: Vec3 {
                x: -W_WIDTH / 2. + 20.,
                y: 0.,
                z: 0.,
            },
            ..Default::default()
        },
        Player::Ai,
    ));
}

fn update_paddles(
    paddles: Query<(&mut Transform, &Player)>,
    ball: Single<&Transform, (With<Ball>, Without<Player>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let d_t = time.delta_secs();
    for (mut transform, player) in paddles {
        if player == &Player::One {
            if keyboard.pressed(KeyCode::KeyW) {
                transform.translation.y += PADDLE_SPEED * d_t;
            }
            if keyboard.pressed(KeyCode::KeyS) {
                transform.translation.y -= PADDLE_SPEED * d_t;
            }
        } else {
            if transform.translation.y > ball.translation.y {
                transform.translation.y -= PADDLE_SPEED * d_t;
            } else if transform.translation.y < ball.translation.y {
                transform.translation.y += PADDLE_SPEED * d_t;
            }
        }
        transform.translation.y = transform
            .translation
            .y
            .clamp(-W_HEIGHT / 2. + 75., W_HEIGHT / 2. - 75.);
    }
}

fn update_ball(
    paddles: Query<&mut Transform, (With<Player>, Without<Ball>)>,
    ball: Single<(&mut Transform, &mut Ball)>,
    mut score: ResMut<Score>,
    time: Res<Time>,
) {
    let d_t = time.delta_secs();

    let (mut transform, mut ball) = ball.into_inner();

    for paddle in paddles {
        if (transform.translation.x + 15. > paddle.translation.x - 5.
            && transform.translation.x + 15. < paddle.translation.x + 5.
            || transform.translation.x - 15. < paddle.translation.x + 5.
                && transform.translation.x - 15. > paddle.translation.x - 5.)
            && transform.translation.y + 15. > paddle.translation.y - 75.
            && transform.translation.y - 15. < paddle.translation.y + 75.
        {
            ball.0.x *= -1.1;
            ball.0.y += rng().random_range::<f32, Range<f32>>(-1.0..1.) * 50.;
            // ball.0.y *= 1.1;
        }
    }
    if transform.translation.x > W_WIDTH / 2. - 20. {
        score.p1 += 1;
        transform.translation = Vec3::ZERO;
        ball.0 = Vec2 { x: 100., y: 0. };

        return;
    } else if transform.translation.x < -W_WIDTH / 2. + 20. {
        score.p2 += 1;
        transform.translation = Vec3::ZERO;
        ball.0 = Vec2 { x: -100., y: 0. };

        return;
    }

    if abs(transform.translation.y) > W_HEIGHT / 2. - 20. {
        ball.0.y *= -1.;
    }

    transform.translation += ball.0.extend(0.) * d_t;
}

fn draw_score(mut commands: Commands, score: Res<Score>) {
    commands.spawn((DespawnOnExit(GameState::Game), Text2d(score.to_string())));
}

fn update_score(mut text_ui: Single<&mut Text2d>, score: Res<Score>) {
    text_ui.0 = score.to_string();
}

fn handle_esc_press(key: Res<ButtonInput<KeyCode>>, mut state: ResMut<NextState<GameState>>) {
    if key.pressed(KeyCode::Escape) {
        state.set(GameState::Menu);
    }
}
