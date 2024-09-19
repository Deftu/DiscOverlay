use std::fs::File;

use tauri::{AppHandle, Emitter, Manager};

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub voice: VoiceOverlaySettings,
    pub message: MessageOverlaySettings,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceOverlaySettings {
    pub show_voice_channel_name: bool,
    pub show_speaking_users_only: bool,
    pub show_own_user_first: bool,
    pub show_muted_users: bool,
    pub show_usernames: bool,
}

impl Default for VoiceOverlaySettings {
    fn default() -> Self {
        VoiceOverlaySettings {
            show_voice_channel_name: true,
            show_speaking_users_only: false,
            show_own_user_first: false,
            show_muted_users: true,
            show_usernames: true,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageOverlaySettings {
    pub channel_id: String,
    pub show_text_channel_name: bool,
}

impl Default for MessageOverlaySettings {
    fn default() -> Self {
        MessageOverlaySettings {
            channel_id: String::new(),
            show_text_channel_name: true,
        }
    }
}

pub fn read(app: &AppHandle) -> Result<Settings, Box<dyn std::error::Error>> {
    let file = file(app);
    let settings = serde_json::from_reader(file)?;
    Ok(settings)
}

pub fn write(app: &AppHandle, settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let file = file(app);
    serde_json::to_writer(file, &settings)?;
    app.emit("settings-updated", Some(settings))?;
    Ok(())
}

fn file(app: &AppHandle) -> File {
    let mut path = app
        .path()
        .app_config_dir()
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
