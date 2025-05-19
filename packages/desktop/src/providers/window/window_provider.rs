use serde::{Deserialize, Serialize};
use window_util::Window;

use crate::providers::{CommonProviderState, Provider, ProviderInputMsg, RuntimeType};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOutput {
  pub title: String,
  pub hwnd: isize,
}

pub struct WindowProvider {
  _config: WindowProviderConfig,
  common: CommonProviderState,
}

impl WindowProvider {
  pub fn new(_config: WindowProviderConfig, common: CommonProviderState) -> WindowProvider {
    WindowProvider { _config, common }
  }
}

#[async_trait]
impl Provider for WindowProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Async
  }

  async fn start_async(&mut self) {
    let Ok(mut window) = Window::new() else {
      self
        .common
        .emitter
        .emit_output::<WindowOutput>(Err(anyhow::anyhow!("Failed to initialize window.")));
      return;
    };

    loop {
      tokio::select! {
          Some(event) = window.events() => {
              self.common.emitter.emit_output(Ok(WindowOutput {
                  title: event.title,
                  hwnd: event.hwnd,
              }));
          }
          Some(input) = self.common.input.async_rx.recv() => {
              match input {
                  ProviderInputMsg::Stop => break,
                  _ => {}
              }
          }
      }
    }
  }
}
