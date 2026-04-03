use anyhow::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

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
    app.insert_resource(load_settings().unwrap_or_default());
    app.init_resource::<Settings>();

    app.add_systems(Update, save_settings.run_if(resource_changed::<Settings>));
}

fn save_settings(_commands: Commands, settings: Res<Settings>) {
    let serialized_settings = serde_json::to_string(&settings.into_inner()).unwrap();
    let settings_path = get_settings_dir();
    create_dir_all(&settings_path).ok();
    write(
        append_settings_filename(settings_path),
        &serialized_settings,
    )
    .expect("Error, could not write settings");
}

fn load_settings() -> Result<Settings> {
    let settings_string = read_to_string(append_settings_filename(get_settings_dir()))?;

    Ok(serde_json::from_str(&settings_string)?)
}

fn get_settings_dir() -> PathBuf {
    let mut path = std::env::home_dir().unwrap();
    path.push(".config/pong");

    path
}

fn append_settings_filename(mut path: PathBuf) -> PathBuf {
    path.push(SETTINGS_PATH);

    path
}
