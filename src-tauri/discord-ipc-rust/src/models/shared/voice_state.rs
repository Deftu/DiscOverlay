use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceState {
  pub nick: String,
  pub mute: bool,
  pub volume: f32,
  pub pan: VoicePan,
  #[serde(rename = "voice_state")]
  pub state: VoiceStateData,
  pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoicePan {
  pub left: u8,
  pub right: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceStateData {
  pub mute: bool,
  pub deaf: bool,
  pub self_mute: bool,
  pub self_deaf: bool,
  pub suppress: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: String,
  pub username: String,
  pub discriminator: String,
  pub avatar: Option<String>,
  pub bot: bool,
}
