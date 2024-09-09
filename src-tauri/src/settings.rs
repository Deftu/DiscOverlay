use std::fs::File;

use tauri::{AppHandle, Manager};

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserSettings {
    pub visibility: UserVisibilitySettings,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UserVisibilitySettings {
    pub show_voice_channel_name: bool,
    pub show_speaking_users_only: bool,
    pub show_own_user_first: bool,
    pub show_muted_users: bool,
    pub show_usernames: bool,
}

impl Default for UserVisibilitySettings {
    fn default() -> Self {
        UserVisibilitySettings {
            show_voice_channel_name: true,
            show_speaking_users_only: false,
            show_own_user_first: false,
            show_muted_users: true,
            show_usernames: true,
        }
    }
}

pub fn read(app: &AppHandle) -> Result<UserSettings, Box<dyn std::error::Error>> {
    let file = file(app);
    let settings = serde_json::from_reader(file)?;
    Ok(settings)
}

pub fn write(app: &AppHandle, settings: UserSettings) -> Result<(), Box<dyn std::error::Error>> {
    let file = file(app);
    serde_json::to_writer(file, &settings)?;
    app.emit_all("settings-updated", Some(settings))?;
    Ok(())
}

fn file(app: &AppHandle) -> File {
    let mut path = tauri::api::path::app_config_dir(app.config().as_ref())
        .expect("Failed to get app config dir");
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create app config dir");
    }

    path.push("settings.json");
    File::options()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .unwrap()
}
