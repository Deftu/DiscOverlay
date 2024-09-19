use tauri::AppHandle;

#[tauri::command]
pub fn load_settings(app: AppHandle) -> crate::settings::Settings {
    crate::settings::read(&app).unwrap_or_default()
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: crate::settings::Settings) {
    crate::settings::write(&app, settings).unwrap();
}
