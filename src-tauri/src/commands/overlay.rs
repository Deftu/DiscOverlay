use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn open_overlay(app: AppHandle) {
    if app.get_window("overlay").is_none() {
        tauri::WindowBuilder::new(&app, "overlay", tauri::WindowUrl::App("overlay".into()))
            .title("Overlay")
            .transparent(true)
            .decorations(false)
            .resizable(false)
            .inner_size(800.0, 600.0)
            .center()
            .build()
            .unwrap();
    }
}

#[tauri::command]
pub async fn close_overlay(app: AppHandle) {
    if let Some(window) = app.get_window("overlay") {
        window.close().unwrap();
    }
}

#[tauri::command]
pub async fn open_custom_css_path(app: AppHandle) {
    let path = crate::custom_css::file(&app);
    open::that(path).unwrap();
}
