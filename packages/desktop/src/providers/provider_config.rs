use serde::Deserialize;

use super::{
  audio::AudioProviderConfig, systray::SystrayProviderConfig, window::WindowProviderConfig
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderConfig {
  Audio(AudioProviderConfig),
  Systray(SystrayProviderConfig),
  Window(WindowProviderConfig),
}
