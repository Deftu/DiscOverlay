use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use tauri::{AppHandle, Emitter, Manager};

const AUTH_URL: &str = "https://discord.com/api/oauth2/authorize?response_type=code";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";

#[derive(Clone)]
pub struct IpcState {
    pub client_id: String,
    pub client_secret: String,
    pub oauth_client: Option<BasicClient>,
}

#[tauri::command]
pub async fn setup_initial(app: AppHandle) {
    crate::discord::http::handle_setup(app);
}

#[tauri::command]
pub async fn start_discord_auth(app: AppHandle, client_id: String, client_secret: String) {
    tokio::spawn(async move {
        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new(AUTH_URL.to_string()).unwrap(),
            Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap()),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:3053/redirect".to_string().to_string()).unwrap(),
        );

        app.manage(IpcState {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            oauth_client: Some(client.clone()),
        });

        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("rpc".to_string()))
            .add_scope(Scope::new("rpc.voice.read".to_string()))
            .url();

        app.emit("auth-url", auth_url.as_ref()).unwrap();
        if let Err(e) = webbrowser::open(auth_url.as_ref()) {
            app.emit("failed-open-browser", e.to_string()).unwrap();
        }
    });
}
