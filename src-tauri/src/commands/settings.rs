use tauri::AppHandle;

#[tauri::command]
pub fn load_settings(app: AppHandle) -> crate::settings::UserSettings {
    crate::settings::read(&app).unwrap_or_default()
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: crate::settings::UserSettings) {
    crate::settings::write(&app, settings).unwrap();
}
