use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::commands::SpeakingData;
use crate::models::shared::VoiceState;

use super::error::ErrorData;
use super::login::LoginData;
use super::ready::ReadyData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "evt")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// All events that come back from the discord RPC
pub enum EventReturn {
  Ready {
    data: ReadyData,
  },
  Login {
    data: LoginData,
  },
  Error {
    data: ErrorData,
  },

  /// speaking start
  SpeakingStart {
    data: SpeakingData,
  },
  /// speaking stop
  SpeakingStop {
    data: SpeakingData,
  },

  /// voice state update
  VoiceStateUpdate {
    data: VoiceState,
  },

  // TODO: type these payloads
  GetSelectedVoiceChannel {
    data: HashMap<String, Value>,
  },
  VoiceStateCreate {
    data: HashMap<String, Value>,
  },
  VoiceStateDelete {
    data: HashMap<String, Value>,
  },
  VoiceChannelSelect {
    data: HashMap<String, Value>,
  },
}
