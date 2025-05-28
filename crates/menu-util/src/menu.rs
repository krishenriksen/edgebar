use tokio::task;
use windows::Win32::{
  Foundation::HWND,
  UI::WindowsAndMessaging::{GWL_EXSTYLE, SetWindowLongPtrW, WS_EX_TOOLWINDOW},
};

use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub fn initialize_menu_window(app_handle: &AppHandle) -> anyhow::Result<()> {
  if app_handle.get_webview_window("macos").is_none() {
    let app_handle_clone = app_handle.clone(); // Clone the app_handle for the async task
    // Use a separate thread for window creation. This is crucial.
    task::spawn(async move {
      let window_result =
        WebviewWindowBuilder::new(&app_handle_clone, "macos", WebviewUrl::App("/".into()))
          .title("Dropdown - EdgeBar") // Include EdgeBar in the title so it can be ignored in the event callback
          .focused(true)
          .visible(false)
          .position(0.0, 35.0)
          .inner_size(10.0, 10.0)
          .resizable(false)
          .decorations(false)
          .closable(false)
          .skip_taskbar(true)
          .always_on_top(true)
          .build();

      match window_result {
        Ok(window) => {
          // Once built, update the style to mark the window as a tool window.
          if let Ok(app_hwnd) = window.hwnd() {
            unsafe {
              SetWindowLongPtrW(HWND(app_hwnd.0), GWL_EXSTYLE, WS_EX_TOOLWINDOW.0 as isize);
            }
          }
        }
        Err(err) => {
          eprintln!("Failed to create window: {}", err);
        }
      }
    });

    hide_menu(app_handle)?;
  }

  Ok(())
}

#[derive(Deserialize)]
pub struct MonitorPosition {
  pub x: f64,
  pub y: f64,
}

#[derive(Deserialize)]
pub struct ButtonPosition {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

pub fn show_menu(
  app_handle: &AppHandle,
  sub_items: Vec<HashMap<String, Value>>,
  button: ButtonPosition,
  monitor: MonitorPosition,
) -> anyhow::Result<()> {
  if let Some(existing_window) = app_handle.get_webview_window("macos") {
    existing_window.hide()?;

    let monitor_scale_factor = existing_window.scale_factor().unwrap_or(1.0);

    let adjusted_left = (monitor.x + button.x) * monitor_scale_factor;
    let adjusted_top = (monitor.y + button.y + button.height) * monitor_scale_factor + 1.0;

    existing_window
      .set_position(tauri::PhysicalPosition::new(adjusted_left, adjusted_top))
      .map_err(|e| anyhow::anyhow!("Failed to set position of window: {}", e))?;

    // Emit raw sub_items directly
    existing_window.emit("updateMenuItems", sub_items).unwrap();

    existing_window.show()?;
  } else {
    eprintln!("No existing window found for 'macos'.");
  }

  Ok(())
}

pub fn resize_menu(
  app_handle: &AppHandle,
  logical_width: f64,
  logical_height: f64,
) -> anyhow::Result<()> {
  if let Some(existing_window) = app_handle.get_webview_window("macos") {
    existing_window.hide()?;

    let monitor_scale_factor = existing_window.scale_factor().unwrap_or(1.0);

    let physical_width = (logical_width * monitor_scale_factor).ceil() as u32;
    let physical_height = (logical_height * monitor_scale_factor).ceil() as u32;

    existing_window
      .set_size(tauri::Size::Physical(tauri::PhysicalSize {
        width: physical_width,
        height: physical_height,
      }))
      .map_err(|e| anyhow::anyhow!("Failed to set menu size: {}", e))?;

    existing_window.show()?;
  }
  Ok(())
}

pub fn hide_menu(app_handle: &AppHandle) -> anyhow::Result<()> {
  if let Some(existing_window) = app_handle.get_webview_window("macos") {
    existing_window.hide()?;
  }

  Ok(())
}
