use tauri::AppHandle;

use crate::overlays::OverlayStorage;

#[tauri::command]
pub async fn open_overlay(app: AppHandle, id: String) {
    if let Some(mut overlay) = OverlayStorage::get_overlay_by_id(&app, &id.to_string()).await {
        overlay.maybe_open().unwrap();
    }
}

#[tauri::command]
pub async fn close_overlay(app: AppHandle, id: String) {
    if let Some(mut overlay) = OverlayStorage::get_overlay_by_id(&app, &id.to_string()).await {
        overlay.maybe_close().unwrap();
    }
}

#[tauri::command]
pub async fn toggle_overlay(app: AppHandle, id: String) {
    if let Some(mut overlay) = OverlayStorage::get_overlay_by_id(&app, &id.to_string()).await {
        overlay.toggle().unwrap();
    }
}

#[tauri::command]
pub async fn focus_overlay(app: AppHandle, id: String) {
    if let Some(overlay) = OverlayStorage::get_overlay_by_id(&app, &id.to_string()).await {
        overlay.focus().unwrap();
    }
}

#[tauri::command]
pub async fn toggle_overlay_devtools(app: AppHandle, id: String) {
    if let Some(overlay) = OverlayStorage::get_overlay_by_id(&app, &id.to_string()).await {
        overlay.toggle_devtools();
    }
}

#[tauri::command]
pub async fn open_custom_css_path(app: AppHandle) {
    let path = crate::custom_css::path(&app);
    open::that(path).unwrap();
}
