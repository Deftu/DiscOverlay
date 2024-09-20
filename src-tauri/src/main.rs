// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Emitter, Manager};

mod commands;
mod custom_css;
mod discord;
mod logging;
mod overlays;
mod settings;

#[derive(Clone, serde::Serialize)]
struct SingleInstancePayload {
    args: Vec<String>,
    cwd: String,
}

#[tokio::main]
async fn main() {
    let logging_handle = logging::setup();

    let builder = tauri::Builder::default()
        .setup(|app| {
            logging::setup_with_app(logging_handle, app.handle());
            custom_css::setup_path_watcher(app.handle());
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit("single-instance", SingleInstancePayload { args: argv, cwd })
                .unwrap();
        }));

    builder
        .invoke_handler(tauri::generate_handler![
            close_completely,
            commands::auth::setup_initial,
            commands::auth::start_discord_auth,
            commands::discord::obtain_own_id,
            commands::discord::request_voice_channel,
            commands::discord::subscribe_voice_disconnect,
            commands::discord::subscribe_speaking_state,
            commands::discord::unsubscribe_speaking_state,
            commands::overlay::open_overlay,
            commands::overlay::close_overlay,
            commands::overlay::toggle_overlay,
            commands::overlay::focus_overlay,
            commands::overlay::toggle_overlay_devtools,
            commands::overlay::open_custom_css_path,
            commands::settings::load_settings,
            commands::settings::save_settings,
            discord::config::load_config
        ])
        .register_uri_scheme_protocol("custom-css", |app, request| {
            // Clone the app handle to use inside the async block
            let app_handle = app.clone();
            let request_clone = request.uri().to_string();

            // Spawn an asynchronous task to handle the protocol
            futures::executor::block_on(async {
                custom_css::handle_uri_scheme_protocol(&app_handle, request_clone).await
            })
        })
        .on_window_event(|window, event| {
            // If this is the "main" window and the close event is triggered, exit the app
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                if window.label() == "main" {
                    window.app_handle().exit(0);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn close_completely(app: AppHandle) {
    app.exit(0);
}
