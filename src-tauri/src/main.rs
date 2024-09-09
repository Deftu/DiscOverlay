// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager};

mod commands;
mod custom_css;
mod discord;
mod settings;

#[derive(Clone, serde::Serialize)]
struct SingleInstancePayload {
    args: Vec<String>,
    cwd: String,
}

#[tokio::main]
async fn main() {
    let builder =
        tauri::Builder::default().plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            use tauri::Manager;

            app.emit_all("single-instance", SingleInstancePayload { args: argv, cwd })
                .unwrap();
        }));

    builder
        .invoke_handler(tauri::generate_handler![
            close_completely,
            commands::auth::setup_initial,
            commands::auth::start_discord_auth,
            commands::discord::obtain_own_id,
            commands::discord::request_voice_channel,
            commands::discord::subscribe_speaking_state,
            commands::discord::unsubscribe_speaking_state,
            commands::overlay::open_overlay,
            commands::overlay::close_overlay,
            commands::overlay::open_custom_css_path,
            commands::settings::load_settings,
            commands::settings::save_settings,
            discord::config::load_config
        ])
        .register_uri_scheme_protocol("discoverlay-custom-css", |app, _| {
            custom_css::handle_uri_scheme_protocol(app)
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                event.window().app_handle().exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn close_completely(app: AppHandle) {
    app.exit(0);
}
