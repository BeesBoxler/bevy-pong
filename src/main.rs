use bevy::{prelude::*, window::WindowResolution};

mod menu;
mod pong;
mod settings;

use menu::main_menu;
use pong::pong_game;
use settings::settings_plugin;

const W_WIDTH: f32 = 1080.;
const W_HEIGHT: f32 = 720.;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(W_WIDTH as u32, W_HEIGHT as u32),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.init_state::<GameState>();

    app.add_plugins(main_menu);
    app.add_plugins(pong_game);
    app.add_plugins(settings_plugin);

    app.run()
}
