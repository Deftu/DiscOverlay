use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

#[derive(Debug)]
pub enum UpdaterError {
    PluginError,
}

impl From<tauri_plugin_updater::Error> for UpdaterError {
    fn from(_e: tauri_plugin_updater::Error) -> Self {
        UpdaterError::PluginError
    }
}

pub fn setup_updater(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        update(app).await.unwrap();
    });
}

async fn update(app: AppHandle) -> Result<(), UpdaterError> {
    log::info!("Checking for updates...");
    if let Some(update) = app.updater()?.check().await? {
        log::info!(
            "Update found: {:#?} to {:#?}",
            update.current_version,
            update.version
        );

        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    log::info!("Downloading update... {downloaded} bytes of {content_length:?}");
                },
                || {
                    log::info!("Update downloaded!");
                },
            )
            .await?;

        log::info!("Update installed!");
        app.restart();
    } else {
        log::info!("No updates found.");
    }

    Ok(())
}
