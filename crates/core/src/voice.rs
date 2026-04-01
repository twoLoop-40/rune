// @spec Specs/Domain/Voice.lean ClaudeCode.Voice

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VoiceStatus {
    #[default]
    Idle,
    Recording,
    Processing,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoiceState {
    #[serde(default)]
    pub status: VoiceStatus,
    pub error: Option<String>,
    pub interim_transcript: Option<String>,
    #[serde(default)]
    pub audio_levels: Vec<u32>,
    #[serde(default)]
    pub warming_up: bool,
}
