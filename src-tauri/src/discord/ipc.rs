use discord_ipc_rust::{
    models::receive::{
        commands::ReturnedCommand,
        events::{ReturnedEvent, VoiceConnectionState},
    },
    DiscordIpcClient, ReceivedItem,
};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;

use crate::commands::auth::IpcState;

use super::http::UserToken;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceResponseUser {
    id: String,
    name: String,
    avatar: Option<String>,
    mute: bool,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceResponse {
    id: String,
    channel_name: String,
    users: Vec<VoiceResponseUser>,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceStateResponse {
    id: String,
    server_mute: bool,
    server_deaf: bool,
    self_mute: bool,
    self_deaf: bool,
}

pub fn setup_ipc(app: &AppHandle) {
    let ipc_state = app.state::<IpcState>();
    let token = app.state::<UserToken>();

    let client_id = ipc_state.client_id.clone();
    let client_secret = ipc_state.client_secret.clone();
    let access_token = token.0.clone();

    let app = app.clone();
    tokio::spawn(async move {
        println!("Starting Discord IPC client");

        // Create our IPC client
        let construction = DiscordIpcClient::create(&client_id, &access_token)
            .await
            .expect("Failed to connect to Discord IPC");

        let mut client = construction.0;

        let app_handle = app.clone();
        client
            .setup_event_handler(move |event| {
                ipc_message_handler(&app_handle, event);
            })
            .await;

        // We were sucessful, so let's save these settings
        let settings = crate::discord::config::DiscordConfig {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
        };
        crate::discord::config::save_config(&app, &settings);

        println!("Passing Discord IPC client to app");
        app.manage(Mutex::new(client));

        println!("Passing self user to app");
        app.manage(construction.1);

        println!("Emitting authenticated event");
        app.emit("authenticated", ()).unwrap();
    });
}

fn ipc_message_handler(app: &AppHandle, event: ReceivedItem) {
    if let ReceivedItem::Command(ReturnedCommand::GetSelectedVoiceChannel(data)) = event {
        app.emit(
            "voice-channel",
            if let Some(data) = data {
                Some(VoiceResponse {
                    id: data.id,
                    channel_name: data.name,
                    users: data
                        .voice_states
                        .iter()
                        .filter(|state| state.user.is_some())
                        .map(|state| {
                            let mut name = state.nick.clone();
                            name = if name.is_empty() {
                                state.user.as_ref().unwrap().username.clone()
                            } else {
                                name
                            };

                            VoiceResponseUser {
                                id: state.user.as_ref().unwrap().id.clone(),
                                name,
                                avatar: state.user.as_ref().unwrap().avatar.clone(),
                                mute: state.state.mute,
                            }
                        })
                        .collect(),
                })
            } else {
                None
            },
        )
        .unwrap();
    } else if let ReceivedItem::Event(event_type) = event {
        match event_type.as_ref() {
            ReturnedEvent::VoiceConnectionStatus(data) => {
                // If the user disconnected, send an event to the frontend
                if let VoiceConnectionState::Disconnected = data.state {
                    app.emit("voice-connection-disconnected", ()).unwrap();
                }
            }

            ReturnedEvent::SpeakingStart(data) => {
                app.emit("speaking-start", data.user_id.clone()).unwrap();
            }

            ReturnedEvent::SpeakingStop(data) => {
                app.emit("speaking-stop", data.user_id.clone()).unwrap();
            }

            ReturnedEvent::VoiceStateCreate(data) => {
                app.emit(
                    "voice-state-create",
                    VoiceStateResponse {
                        id: data.user.as_ref().unwrap().id.clone(),
                        server_mute: data.state.mute,
                        server_deaf: data.state.deaf,
                        self_mute: data.state.self_mute,
                        self_deaf: data.state.self_deaf,
                    },
                )
                .unwrap();
            }

            ReturnedEvent::VoiceStateUpdate(data) => {
                app.emit(
                    "voice-state-update",
                    VoiceStateResponse {
                        id: data.user.as_ref().unwrap().id.clone(),
                        server_mute: data.state.mute,
                        server_deaf: data.state.deaf,
                        self_mute: data.state.self_mute,
                        self_deaf: data.state.self_deaf,
                    },
                )
                .unwrap();
            }

            ReturnedEvent::VoiceStateDelete(data) => {
                app.emit("voice-state-delete", data.user.as_ref().unwrap().id.clone())
                    .unwrap();
            }

            _ => {}
        }
    }
}
