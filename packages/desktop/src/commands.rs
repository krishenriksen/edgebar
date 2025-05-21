use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::common::windows::WindowExtWindows;
use crate::{
  config::{Config, WidgetConfig, WidgetPlacement},
  providers::{ProviderConfig, ProviderFunction, ProviderFunctionResponse, ProviderManager},
  shell_state::{ShellCommandArgs, ShellState},
  widget_factory::{WidgetFactory, WidgetOpenOptions, WidgetState},
};
use menu_util::{ButtonPosition, MonitorPosition};
use tauri::{AppHandle, State, Window};
use window_util::Window as UtilWindow;

#[tauri::command]
pub async fn widget_configs(
  config: State<'_, Arc<Config>>,
) -> Result<HashMap<PathBuf, WidgetConfig>, String> {
  Ok(config.widget_configs().await)
}

#[tauri::command]
pub async fn widget_states(
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> Result<HashMap<String, WidgetState>, String> {
  Ok(widget_factory.states().await)
}

#[tauri::command]
pub async fn start_widget(
  config_path: String,
  placement: WidgetPlacement,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .start_widget(
      &PathBuf::from(config_path),
      &WidgetOpenOptions::Standalone(placement),
    )
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn start_preset(
  config_path: String,
  preset_name: String,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .start_widget(
      &PathBuf::from(config_path),
      &WidgetOpenOptions::Preset(preset_name),
    )
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn stop_preset(
  config_path: String,
  preset_name: String,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .stop_by_preset(&PathBuf::from(config_path), &preset_name)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn update_widget_config(
  config_path: String,
  new_config: WidgetConfig,
  config: State<'_, Arc<Config>>,
) -> Result<(), String> {
  config
    .update_widget_config(&PathBuf::from(config_path), new_config)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn listen_provider(
  config_hash: String,
  config: ProviderConfig,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<(), String> {
  provider_manager
    .create(config_hash, config)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn unlisten_provider(
  config_hash: String,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<(), String> {
  provider_manager
    .stop(config_hash)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn call_provider_function(
  config_hash: String,
  function: ProviderFunction,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<ProviderFunctionResponse, String> {
  provider_manager
    .call_function(config_hash, function)
    .await
    .map_err(|err| err.to_string())
}

/// Tauri's implementation of `always_on_top` places the window above
/// all normal windows (but not the MacOS menu bar). The following instead
/// sets the z-order of the window to be above the menu bar.
#[tauri::command]
pub fn set_always_on_top(window: Window) -> anyhow::Result<(), String> {
  let res = window.set_always_on_top(true);
  res.map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set_skip_taskbar(window: Window, skip: bool) -> anyhow::Result<(), String> {
  window
    .set_tool_window(skip)
    .map_err(|err| err.to_string())?;

  Ok(())
}

#[tauri::command]
pub async fn shell_exec(
  program: String,
  args: ShellCommandArgs,
  options: shell_util::CommandOptions,
  window: Window,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<shell_util::ShellExecOutput, String> {
  let widget_id = window.label();
  shell_state
    .exec(&widget_id, &program, args, &options)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn shell_spawn(
  program: String,
  args: ShellCommandArgs,
  options: shell_util::CommandOptions,
  window: Window,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<shell_util::ProcessId, String> {
  let widget_id = window.label();
  shell_state
    .spawn(&widget_id, &program, args, &options)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn shell_write(
  pid: shell_util::ProcessId,
  buffer: shell_util::Buffer,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<(), String> {
  shell_state
    .write(pid, buffer)
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn shell_kill(
  pid: shell_util::ProcessId,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<(), String> {
  shell_state.kill(pid).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set_foreground_window(hwnd: isize) -> Result<String, String> {
  match UtilWindow::set_foreground_window(hwnd) {
    Ok(_) => Ok(format!(
      "Successfully set window {} to the foreground.",
      hwnd
    )),
    Err(err) => Err(format!(
      "Failed to set window {} to the foreground: {}",
      hwnd, err
    )),
  }
}

#[tauri::command]
pub fn show_menu(
  sub_items: Vec<HashMap<String, Value>>,
  button: ButtonPosition,
  monitor: MonitorPosition,
  config: State<'_, Arc<Config>>,
) -> Result<String, String> {
  let app_handle: &AppHandle = config.app_handle();

  match menu_util::show_menu(app_handle, sub_items, button, monitor) {
    Ok(_) => Ok(format!("Successfully shown menu")),
    Err(err) => Err(format!("Failed to show menu: {}", err)),
  }
}

#[tauri::command]
pub fn hide_menu(config: State<'_, Arc<Config>>) -> Result<String, String> {
  let app_handle: &AppHandle = config.app_handle();

  match menu_util::hide_menu(app_handle) {
    Ok(_) => Ok(format!("Successfully shown menu")),
    Err(err) => Err(format!("Failed to show menu: {}", err)),
  }
}

#[tauri::command]
pub fn resize_menu(
  width: f64,
  height: f64,
  config: State<'_, Arc<Config>>,
) -> Result<String, String> {
  let app_handle: &AppHandle = config.app_handle();

  match menu_util::resize_menu(app_handle, width, height) {
    Ok(_) => Ok(format!("Successfully resized menu")),
    Err(err) => Err(format!("Failed to resize menu: {}", err)),
  }
}
