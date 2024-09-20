use std::net::SocketAddr;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use tauri::{AppHandle, Manager};

use crate::commands::auth::IpcState;

#[derive(Clone)]
struct AppState {
    app: AppHandle,
}

#[derive(Debug, serde::Deserialize)]
struct RedirectQuery {
    code: String,
}

pub struct UserToken(pub String);

pub fn handle_setup(app: AppHandle) {
    tokio::spawn(async move {
        let app_state = app.clone();
        let app_state = AppState { app: app_state };

        if app.try_state::<SocketAddr>().is_some() {
            return;
        }

        log::info!("Creating OAuth HTTP server");

        let server = Router::new()
            .route("/redirect", get(redirect))
            .with_state(app_state)
            .into_make_service();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3053")
            .await
            .unwrap();

        app.manage(listener.local_addr().unwrap());

        axum::serve(listener, server).await.unwrap();
    });
}

async fn redirect(
    Query(query): Query<RedirectQuery>,
    State(app): State<AppState>,
) -> impl IntoResponse {
    let app = app.app;
    let ipc_state = app.state::<IpcState>();
    let client = ipc_state.oauth_client.clone();

    let token_result = client
        .expect("No client")
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            app.manage(UserToken(token.access_token().secret().to_string()));
            super::ipc::setup_ipc(&app);
            "Authenticated".to_string()
        }

        Err(err) => {
            log::error!("Error exchanging code: {:?}", err);
            "Authentication failed".to_string()
        }
    }
}
