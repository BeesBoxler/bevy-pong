use std::fs::{read_to_string, write};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "settings.json";

#[derive(Resource, Serialize, Deserialize)]
pub struct Settings {
    pub demo_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { demo_mode: true }
    }
}

pub fn settings_plugin(app: &mut App) {
    app.insert_resource(match load_settings() {
        Ok(settings) => settings,
        Err(_) => Default::default(),
    });
    app.init_resource::<Settings>();

    app.add_systems(Update, save_settings.run_if(resource_changed::<Settings>));
}

fn save_settings(_commands: Commands, settings: Res<Settings>) {
    let serialized_settings = serde_json::to_string(&settings.into_inner()).unwrap();

    write("settings.json", &serialized_settings).expect("Error, could not write settings");

    println!("{}", serialized_settings);
}

fn load_settings() -> Result<Settings> {
    match read_to_string(SETTINGS_PATH) {
        Err(e) => Result::Err(e.into()),
        Ok(settings) => match serde_json::from_str::<Settings>(&settings) {
            Err(e) => Result::Err(e.into()),
            Ok(settings) => Ok(settings),
        },
    }
}
