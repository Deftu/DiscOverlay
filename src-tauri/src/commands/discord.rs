use rpc_discord::{
    models::{rpc_command::RPCCommand, rpc_event::RPCEvent, shared::User},
    DiscordIpcClient,
};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn obtain_own_id(app: AppHandle) -> String {
    let user = app.state::<User>();

    user.id.clone()
}

#[tauri::command]
pub async fn request_voice_channel(app: AppHandle) {
    let client_mutex = app.state::<Mutex<DiscordIpcClient>>();
    let mut client = client_mutex.lock().await;

    client
        .emit_command(&RPCCommand::GetSelectedVoiceChannel)
        .await
        .expect("Failed to get voice channel");
}

#[tauri::command]
pub async fn subscribe_speaking_state(app: AppHandle, channel_id: String) {
    let client_mutex = app.state::<Mutex<DiscordIpcClient>>();
    let mut client = client_mutex.lock().await;

    client
        .emit_command(&RPCCommand::Subscribe(RPCEvent::SpeakingStart {
            channel_id: channel_id.clone(),
        }))
        .await
        .expect("Failed to subscribe to speaking state");

    client
        .emit_command(&RPCCommand::Subscribe(RPCEvent::SpeakingStop {
            channel_id: channel_id.clone(),
        }))
        .await
        .expect("Failed to subscribe to speaking state");

    client
        .emit_command(&RPCCommand::Subscribe(RPCEvent::VoiceStateUpdate {
            channel_id: channel_id.clone(),
        }))
        .await
        .expect("Failed to subscribe to speaking state");
}

#[tauri::command]
pub async fn unsubscribe_speaking_state(app: AppHandle, channel_id: String) {
    let client_mutex = app.state::<Mutex<DiscordIpcClient>>();
    let mut client = client_mutex.lock().await;

    client
        .emit_command(&RPCCommand::Unsubscribe(RPCEvent::SpeakingStart {
            channel_id: channel_id.clone(),
        }))
        .await
        .expect("Failed to unsubscribe to speaking state");

    client
        .emit_command(&RPCCommand::Unsubscribe(RPCEvent::SpeakingStop {
            channel_id: channel_id.clone(),
        }))
        .await
        .expect("Failed to unsubscribe to speaking state");

    client
        .emit_command(&RPCCommand::Unsubscribe(RPCEvent::VoiceStateUpdate {
            channel_id: channel_id.clone(),
        }))
        .await
        .expect("Failed to unsubscribe to speaking state");
}
