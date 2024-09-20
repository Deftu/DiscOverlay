use std::{fs::create_dir_all, path::PathBuf, time::Duration};

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, Watcher};
use tauri::{
    http::{response::Builder as ResponseBuilder, Response as HttpResponse},
    AppHandle, Emitter, Manager,
};

use crate::overlays::OverlayStorage;

pub async fn handle_uri_scheme_protocol(
    app: &AppHandle,
    request_uri: String,
) -> HttpResponse<Vec<u8>> {
    let requested_path = percent_encoding::percent_decode(request_uri[1..].as_bytes())
        .decode_utf8_lossy()
        .to_string();
    let path = path(app);

    let id = requested_path.replace(".css", "");
    let id = id.rsplit_once('/').unwrap().1;

    // If there are any query parameters, remove them.
    let id = id.split_once('?').unwrap_or((id, "")).0;

    if id == "global" {
        return create_response_from_path(path.join("global.css"));
    }

    // If there are no overlay at the ID (requested path), return a 404.
    let overlay = OverlayStorage::get_overlay_by_id(app, id).await;
    if overlay.is_none() {
        log::warn!(
            "Requested custom CSS @ {}, but that's not a valid overlay",
            id
        );

        return ResponseBuilder::new().status(404).body(vec![]).unwrap();
    }

    let overlay = overlay.unwrap();
    let mut filename = String::from(overlay.get_custom_css_filename());
    if !filename.ends_with(".css") {
        filename.push_str(".css");
    }

    let path = path.join(filename);
    create_response_from_path(path)
}

pub fn setup_path_watcher(app: &AppHandle) {
    let (watcher, mut rx) = create_async_watcher().expect("Failed to create watcher");

    let app_handle = app.clone();
    tokio::task::spawn(async move {
        let mut watcher = watcher;
        let path = path(&app_handle);

        watcher
            .watch(&path, notify::RecursiveMode::NonRecursive)
            .expect("Failed to watch path");

        while let Some(res) = rx.next().await {
            match res {
                Ok(event) => {
                    if let Some(path) = event.paths.first() {
                        if path.extension().map_or(false, |ext| ext == "css") {
                            app_handle
                                .emit("custom-css-update", path.to_str().unwrap())
                                .unwrap();
                        }
                    }
                }
                Err(e) => {
                    log::error!("watch error: {:?}", e);
                }
            }
        }
    });
}

/// Returns the path to the directories where custom CSS files are stored.
pub fn path(app: &AppHandle) -> PathBuf {
    let mut path = app
        .path()
        .app_config_dir()
        .expect("Failed to get app config dir");
    path.push("css");
    if !path.exists() {
        create_dir_all(&path).expect("Failed to create app config dir");
    }

    path
}

fn create_response_from_path(path: PathBuf) -> HttpResponse<Vec<u8>> {
    if !path.exists() {
        std::fs::File::create(&path).unwrap();
    }

    ResponseBuilder::new()
        .status(200)
        .header("content-type", "text/css")
        .body(std::fs::read(path).unwrap())
        .unwrap()
}

fn create_async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(true),
    )?;

    Ok((watcher, rx))
}
