use std::fs::File;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[tauri::command]
pub fn load_config(app: AppHandle) -> DiscordConfig {
    let mut file = config_file(&app);
    let settings: DiscordConfig = serde_json::from_reader(&mut file).unwrap_or_default();
    settings
}

pub fn save_config(app: &AppHandle, config: &DiscordConfig) {
    let mut file = config_file(app);
    serde_json::to_writer(&mut file, config).unwrap();
}

fn config_file(app: &AppHandle) -> File {
    let path = tauri::api::path::app_config_dir(app.config().as_ref())
        .expect("Failed to get app config dir");
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create app config dir");
    }

    let config_file = path.join("discord_config.json");

    File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(config_file)
        .unwrap()
}
