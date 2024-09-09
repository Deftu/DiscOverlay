use std::path::PathBuf;

use tauri::{
    http::{Response as HttpResponse, ResponseBuilder},
    AppHandle,
};

pub fn handle_uri_scheme_protocol(
    app: &AppHandle,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let path = file(app);
    let response = ResponseBuilder::new()
        .status(200)
        .header("content-type", "text/css")
        .body(std::fs::read(path).unwrap());
    Ok(response.unwrap())
}

pub fn file(app: &AppHandle) -> PathBuf {
    let config = app.config();
    let config = config.as_ref();
    let binding = tauri::api::path::app_config_dir(config);
    let mut path = binding.expect("Failed to get app config dir");
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create app config dir");
    }

    path.push("custom.css");

    // Ensure the file exists
    if !path.exists() {
        std::fs::write(&path, "").expect("Failed to create custom.css");
    }

    path
}
