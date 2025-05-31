use tokio::task;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GWL_EXSTYLE, SetWindowLongPtrW, WS_EX_TOOLWINDOW},
};

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Emitter};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Represents the position of a monitor.
#[derive(Deserialize)]
pub struct MonitorPosition {
    pub x: f64,
    pub y: f64,
}

/// Represents the position and size of a button.
#[derive(Deserialize)]
pub struct ButtonPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Builds a hidden, non-interactive menu window for the app.
fn build_menu_window(app_handle: &AppHandle) -> tauri::Result<WebviewWindow> {
    WebviewWindowBuilder::new(app_handle, "macos", WebviewUrl::App("/".into()))
        .title("Dropdown - EdgeBar")
        .focused(false)
        .visible(false)
        .position(0.0, 35.0)
        .inner_size(10.0, 10.0)
        .resizable(false)
        .decorations(false)
        .closable(false)
        .skip_taskbar(true)
        .always_on_top(true)
        .build()
}

/// Initializes the menu window if it does not already exist.
///
/// Spawns a background task to create the window and set its style.
pub fn initialize_menu_window(app_handle: &AppHandle) -> anyhow::Result<()> {
    if app_handle.get_webview_window("macos").is_none() {
        let app_handle_clone = app_handle.clone();
        task::spawn(async move {
            match build_menu_window(&app_handle_clone) {
                Ok(window) => {
                    if let Ok(app_hwnd) = window.hwnd() {
                        unsafe {
                            SetWindowLongPtrW(
                                HWND(app_hwnd.0),
                                GWL_EXSTYLE,
                                WS_EX_TOOLWINDOW.0 as isize,
                            );
                        }
                    }
                }
                Err(err) => {
                    eprintln!("[EdgeBar] Failed to create menu window: {err}");
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let _ = hide_menu(&app_handle_clone);
        });
    }
    Ok(())
}

/// Shows the menu window at the correct position and emits menu items.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `sub_items` - The menu items to display.
/// * `button` - The button's position and size.
/// * `monitor` - The monitor's position.
pub fn show_menu(
    app_handle: &AppHandle,
    sub_items: Vec<HashMap<String, Value>>,
    button: ButtonPosition,
    monitor: MonitorPosition,
) -> anyhow::Result<()> {
    if let Some(window) = app_handle.get_webview_window("macos") {
        window.hide()?;

        let scale = window.scale_factor().unwrap_or(1.0);
        let left = (monitor.x + button.x) * scale;
        let top = (monitor.y + button.y + button.height) * scale + 1.0;

        window
            .set_position(tauri::PhysicalPosition::new(left, top))
            .map_err(|e| anyhow::anyhow!("Failed to set menu window position: {e}"))?;

        window.emit("updateMenuItems", sub_items).ok();
        window.show()?;
    } else {
        eprintln!("[EdgeBar] No menu window found to show.");
    }
    Ok(())
}

/// Resizes the menu window to the specified logical size.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle.
/// * `logical_width` - The desired logical width.
/// * `logical_height` - The desired logical height.
pub fn resize_menu(
    app_handle: &AppHandle,
    logical_width: f64,
    logical_height: f64,
) -> anyhow::Result<()> {
    if let Some(window) = app_handle.get_webview_window("macos") {
        window.hide()?;

        let scale = window.scale_factor().unwrap_or(1.0);
        let width = (logical_width * scale).ceil() as u32;
        let height = (logical_height * scale).ceil() as u32;

        window
            .set_size(tauri::Size::Physical(tauri::PhysicalSize { width, height }))
            .map_err(|e| anyhow::anyhow!("Failed to resize menu window: {e}"))?;

        window.show()?;
    }
    Ok(())
}

/// Hides the menu window if it exists.
pub fn hide_menu(app_handle: &AppHandle) -> anyhow::Result<()> {
    if let Some(window) = app_handle.get_webview_window("macos") {
        window.hide()?;
    }
    Ok(())
}