use serde::Serialize;

use super::{audio::AudioOutput, systray::SystrayOutput, window::WindowOutput};

/// Implements `From<T>` for `ProviderOutput` for each given variant.
macro_rules! impl_provider_output {
  ($($variant:ident($type:ty)),* $(,)?) => {
    $(
      impl From<$type> for ProviderOutput {
        fn from(value: $type) -> Self {
          Self::$variant(value)
        }
      }
    )*
  };
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProviderOutput {
  Audio(AudioOutput),
  Systray(SystrayOutput),
  Window(WindowOutput)
}

impl_provider_output! {
  Audio(AudioOutput),
  Systray(SystrayOutput),
  Window(WindowOutput),
}
