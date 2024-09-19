use std::collections::HashMap;

use tauri::{AppHandle, Error, Manager, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Overlay {
    title: String,
    label: String,
    url: String,
    custom_css_filename: String,
    app: AppHandle,
}

impl Overlay {
    pub fn new(
        title: String,
        label: String,
        url: String,
        custom_css_filename: String,
        app: AppHandle,
    ) -> Self {
        Overlay {
            title,
            label,
            url,
            custom_css_filename,
            app,
        }
    }

    pub fn maybe_open(&mut self) -> Result<(), Error> {
        if self.app.get_webview_window(&self.label).is_none() {
            WebviewWindowBuilder::new(
                &self.app,
                self.label.clone(),
                WebviewUrl::App(self.url.clone().into()),
            )
            .title(self.title.clone())
            .transparent(true)
            .resizable(false)
            .inner_size(800.0, 600.0)
            .maximizable(false)
            .center()
            .build()?;
        }

        Ok(())
    }

    pub fn maybe_close(&mut self) -> Result<(), Error> {
        if let Some(window) = self.app.get_webview_window(&self.label) {
            window.close()?;
        }

        Ok(())
    }

    pub fn toggle(&mut self) -> Result<(), Error> {
        if self.app.get_webview_window(&self.label).is_some() {
            self.maybe_close()?;
        } else {
            self.maybe_open()?;
        }

        Ok(())
    }

    pub fn focus(&self) -> Result<(), Error> {
        if let Some(window) = self.app.get_webview_window(&self.label) {
            window.set_focus()?;
        }

        Ok(())
    }

    pub fn toggle_devtools(&self) {
        if let Some(window) = self.app.get_webview_window(&self.label) {
            if window.is_devtools_open() {
                window.close_devtools();
            } else {
                window.open_devtools();
            }
        }
    }

    pub fn get_custom_css_filename(&self) -> &str {
        &self.custom_css_filename
    }
}

pub struct OverlayStorage(pub Mutex<HashMap<String, Overlay>>);

impl OverlayStorage {
    pub fn get(app: &AppHandle) -> &Self {
        if let Some(overlays) = app.try_state::<Self>() {
            return overlays.clone().inner();
        }

        let overlays = OverlayStorage(Mutex::new(HashMap::new()));
        app.manage(overlays);
        Self::get(app)
    }

    pub async fn get_voice_overlay(app: &AppHandle) -> Overlay {
        let mut overlays = Self::get(app).0.lock().await;
        if let Some(overlay) = overlays.get("voice") {
            return overlay.clone();
        }

        let overlay = Overlay::new(
            "Voice Overlay".to_string(),
            "voice".to_string(),
            "voice-overlay".to_string(),
            "voice".to_string(),
            app.clone(),
        );

        overlays.insert("voice".to_string(), overlay.clone());
        overlay
    }

    pub async fn get_message_overlay(app: &AppHandle) -> Overlay {
        let mut overlays = Self::get(app).0.lock().await;
        if let Some(overlay) = overlays.get("message") {
            return overlay.clone();
        }

        let overlay = Overlay::new(
            "Message Overlay".to_string(),
            "message".to_string(),
            "message-overlay".to_string(),
            "message".to_string(),
            app.clone(),
        );

        overlays.insert("message".to_string(), overlay.clone());
        overlay
    }

    pub async fn get_overlay_by_id(app: &AppHandle, id: &str) -> Option<Overlay> {
        match id {
            "voice" => Some(Self::get_voice_overlay(app).await),
            "message" => Some(Self::get_message_overlay(app).await),
            _ => None,
        }
    }
}
