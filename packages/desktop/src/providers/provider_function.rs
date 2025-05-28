use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "function", rename_all = "snake_case")]
pub enum ProviderFunction {
  Audio(AudioFunction),
  Systray(SystrayFunction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", content = "args", rename_all = "snake_case")]
pub enum AudioFunction {
  SetVolume(SetVolumeArgs),
  SetMute(SetMuteArgs),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVolumeArgs {
  pub volume: f32,
  pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMuteArgs {
  pub mute: bool,
  pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", content = "args", rename_all = "snake_case")]
pub enum SystrayFunction {
  IconHoverEnter(SystrayIconArgs),
  IconHoverLeave(SystrayIconArgs),
  IconHoverMove(SystrayIconArgs),
  IconLeftClick(SystrayIconArgs),
  IconRightClick(SystrayIconArgs),
  IconMiddleClick(SystrayIconArgs),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystrayIconArgs {
  pub icon_id: String,
}

pub type ProviderFunctionResult = Result<ProviderFunctionResponse, String>;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProviderFunctionResponse {
  Null,
}
