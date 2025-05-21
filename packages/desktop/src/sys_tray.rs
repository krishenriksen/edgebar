use std::str::FromStr;

use anyhow::{bail, Context};
use tauri::{
  image::Image,
  menu::{Menu, MenuBuilder},
  tray::{TrayIcon, TrayIconBuilder},
  AppHandle, Wry,
};
use tokio::task;
use tracing::{error, info};

#[derive(Debug, Clone)]
enum MenuEvent {
  Exit,
}

impl ToString for MenuEvent {
  fn to_string(&self) -> String {
    match self {
      MenuEvent::Exit => "exit".to_string(),
    }
  }
}

impl FromStr for MenuEvent {
  type Err = anyhow::Error;

  fn from_str(event: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = event.split('_').collect();

    match parts.as_slice() {
      ["exit"] => Ok(Self::Exit),
      _ => bail!("Invalid menu event: {}", event),
    }
  }
}

/// System tray icon for EdgeBar.
pub struct SysTray {
  app_handle: AppHandle,
  tray_icon: Option<TrayIcon>,
}

impl SysTray {
  /// Creates a new system tray icon for EdgeBar.
  pub async fn new(app_handle: &AppHandle) -> anyhow::Result<SysTray> {
    let mut sys_tray = Self {
      app_handle: app_handle.clone(),
      tray_icon: None,
    };

    sys_tray.tray_icon = Some(sys_tray.create_tray_icon().await?);

    Ok(sys_tray)
  }

  async fn create_tray_icon(&self) -> anyhow::Result<TrayIcon> {
    let tooltip = format!("EdgeBar v{}", env!("VERSION_NUMBER"));

    // Linting: `mut` needed for Windows where `tray_icon` is modified with
    // additional click handler.
    #[allow(unused_mut)]
    let mut tray_icon = TrayIconBuilder::with_id("tray")
      .icon(self.icon_image()?)
      .menu(&self.create_tray_menu().await?)
      .tooltip(tooltip)
      .on_menu_event({
        move |app_handle, event| {
          if let Ok(menu_event) = MenuEvent::from_str(event.id.as_ref()) {
            Self::handle_menu_event(menu_event, app_handle.clone());
          }
        }
      });

    Ok(tray_icon.build(&self.app_handle)?)
  }

  pub async fn refresh(&self) -> anyhow::Result<()> {
    info!("Updating system tray menu.");

    if let Some(tray_icon) = self.tray_icon.as_ref() {
      let tray_menu = self.create_tray_menu().await?;
      tray_icon.set_menu(Some(tray_menu))?;
    }

    Ok(())
  }

  /// Returns the image to use for the system tray icon.
  fn icon_image(&self) -> anyhow::Result<Image> {
    self
      .app_handle
      .default_window_icon()
      .cloned()
      .context("No icon defined in Tauri config.")
  }

  /// Creates and returns the main system tray menu.
  async fn create_tray_menu(&self) -> anyhow::Result<Menu<Wry>> {
    let tray_menu = MenuBuilder::new(&self.app_handle)
      .text(MenuEvent::Exit, "Exit")
      .build()?;

    Ok(tray_menu)
  }

  /// Callback for system tray menu events.
  fn handle_menu_event(event: MenuEvent, app_handle: AppHandle) {
    task::spawn(async move {
      info!("Received tray menu event: {:?}", event);

      let event_res: Result<(), anyhow::Error> = match event {
        MenuEvent::Exit => {
          app_handle.exit(0);
          Ok(())
        }
      };

      if let Err(err) = event_res {
        error!("Error handling menu event: {:?}", err);
      }
    });
  }
}
