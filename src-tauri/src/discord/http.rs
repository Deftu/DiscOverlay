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
        let app = AppState { app };

        println!("Creating OAuth HTTP server");

        let server = Router::new()
            .route("/redirect", get(redirect))
            .with_state(app)
            .into_make_service();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3053")
            .await
            .unwrap();

        axum::serve(listener, server).await.unwrap();
    });
}

async fn redirect(
    Query(query): Query<RedirectQuery>,
    State(app): State<AppState>,
) -> impl IntoResponse {
    println!("Redirected with code: {}", query.code);

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
            eprintln!("Error exchanging code: {:?}", err);
            "Authentication failed".to_string()
        }
    }
}
