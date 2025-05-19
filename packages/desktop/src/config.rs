use std::{
  collections::HashMap,
  fs::{self},
  path::PathBuf,
  sync::Arc,
};

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Deserializer, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tokio::sync::{broadcast, Mutex};
use tracing::{error, info};

use crate::common::{copy_dir_all, has_extension, read_and_parse_json, LengthValue, PathExt};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsConfig {
  /// JSON schema URL to validate the settings file.
  #[serde(rename = "$schema")]
  schema: Option<String>,

  /// Widget configs to be launched on startup.
  pub startup_configs: Vec<StartupConfig>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupConfig {
  /// Relative path to widget configs to launch on startup.
  pub path: PathBuf,

  /// Preset name within the widget config.
  pub preset: String,
}

// Deserializer that handles `StartupConfig` objects and string format from
// v2.3.0 and earlier.
impl<'de> Deserialize<'de> for StartupConfig {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrObject {
      String(String),
      Object { path: PathBuf, preset: String },
    }

    let value = StringOrObject::deserialize(deserializer)?;

    Ok(match value {
      StringOrObject::String(s) => StartupConfig {
        path: PathBuf::from(s),
        preset: "default".to_string(),
      },
      StringOrObject::Object { path, preset } => StartupConfig { path, preset },
    })
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfig {
  /// JSON schema URL to validate the widget config file.
  #[serde(rename = "$schema")]
  schema: Option<String>,

  /// Relative path to entry point HTML file.
  pub html_path: PathBuf,

  /// Whether to show the Tauri window above/below all others.
  pub z_order: ZOrder,

  /// Whether the Tauri window should be shown in the taskbar.
  pub shown_in_taskbar: bool,

  /// Whether the Tauri window should be focused when opened.
  pub focused: bool,

  /// Whether the Tauri window should have resize handles.
  pub resizable: bool,

  /// Whether the Tauri window frame should be transparent.
  pub transparent: bool,

  /// How network requests should be cached.
  #[serde(default)]
  pub caching: WidgetCaching,

  /// Privileges for the widget.
  #[serde(default)]
  pub privileges: WidgetPrivileges,

  /// Where to place the widget. Add alias for `defaultPlacements` for
  /// compatibility with v2.3.0 and earlier.
  #[serde(alias = "defaultPlacements")]
  pub presets: Vec<WidgetPreset>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ZOrder {
  BottomMost,
  Normal,
  TopMost,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WidgetCaching {
  /// Default duration to cache network resources for (in seconds).
  pub default_duration: u32,

  /// Custom cache rules.
  pub rules: Vec<WidgetCachingRule>,
}

impl Default for WidgetCaching {
  fn default() -> Self {
    Self {
      default_duration: 604800,
      rules: Vec::new(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetCachingRule {
  /// URL regex pattern to match.
  pub url_regex: String,

  /// Duration to cache the matched requests for (in seconds).
  pub duration: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPreset {
  #[serde(default = "default_preset_name")]
  pub name: String,

  #[serde(flatten)]
  pub placement: WidgetPlacement,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPlacement {
  /// Anchor-point of the widget.
  pub anchor: AnchorPoint,

  /// Offset from the anchor-point.
  pub offset_x: LengthValue,

  /// Offset from the anchor-point.
  pub offset_y: LengthValue,

  /// Width of the widget in % or physical pixels.
  pub width: LengthValue,

  /// Height of the widget in % or physical pixels.
  pub height: LengthValue,

  /// Monitor(s) to place the widget on.
  pub monitor_selection: MonitorSelection,

  /// How to reserve space for the widget.
  #[serde(default)]
  pub dock_to_edge: DockConfig,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, ValueEnum)]
#[clap(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AnchorPoint {
  TopLeft,
  TopCenter,
  TopRight,
  CenterLeft,
  Center,
  CenterRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "match", rename_all = "snake_case")]
pub enum MonitorSelection {
  All,
  Primary,
  Secondary,
  Index(usize),
  Name(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPrivileges {
  /// Shell commands that the widget is allowed to run.
  pub shell_commands: Vec<ShellPrivilege>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShellPrivilege {
  /// Program name (if in PATH) or full path to the program.
  pub program: String,

  /// Arguments to pass to the program.
  pub args_regex: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DockConfig {
  /// Whether to dock the widget to the monitor edge and reserve screen
  /// space for it.
  #[serde(default = "default_bool::<false>")]
  pub enabled: bool,

  /// Edge to dock the widget to.
  pub edge: Option<DockEdge>,

  /// Margin to reserve after the widget window. Can be positive or
  /// negative.
  #[serde(default)]
  pub window_margin: LengthValue,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DockEdge {
  Top,
  Bottom,
  Left,
  Right,
}

impl DockEdge {
  pub fn is_horizontal(&self) -> bool {
    matches!(self, Self::Top | Self::Bottom)
  }
}

#[derive(Debug)]
pub struct Config {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Directory where config files are stored.
  pub config_dir: PathBuf,

  /// Global settings.
  pub settings: Arc<Mutex<SettingsConfig>>,

  /// List of widget configs.
  pub widget_configs: Arc<Mutex<HashMap<PathBuf, WidgetConfig>>>,

  _settings_change_rx: broadcast::Receiver<SettingsConfig>,

  pub settings_change_tx: broadcast::Sender<SettingsConfig>,

  _widget_configs_change_rx: broadcast::Receiver<HashMap<PathBuf, WidgetConfig>>,

  pub widget_configs_change_tx: broadcast::Sender<HashMap<PathBuf, WidgetConfig>>,
}

impl Config {
  // Method to retrieve AppHandle
  pub fn app_handle(&self) -> &AppHandle {
    &self.app_handle
  }

  /// Reads the config files within the config directory.
  ///
  /// Returns a new `Config` instance.
  pub fn new(app_handle: &AppHandle, config_dir_override: Option<PathBuf>) -> anyhow::Result<Self> {
    let config_dir = match config_dir_override {
      Some(dir) => dir,
      None => app_handle
        .path()
        .resolve(".edgebar", BaseDirectory::Home)
        .context("Unable to get home directory.")?,
    };

    let settings = Self::read_settings_or_init(app_handle, &config_dir)?;
    let widget_configs = Self::read_widget_configs(&config_dir)?;

    let (settings_change_tx, _settings_change_rx) = broadcast::channel(16);
    let (widget_configs_change_tx, _widget_configs_change_rx) = broadcast::channel(16);

    Ok(Self {
      app_handle: app_handle.clone(),
      config_dir: config_dir.to_absolute()?,
      settings: Arc::new(Mutex::new(settings)),
      widget_configs: Arc::new(Mutex::new(widget_configs)),
      _settings_change_rx,
      settings_change_tx,
      _widget_configs_change_rx,
      widget_configs_change_tx,
    })
  }

  /// Reads the global settings file or initializes it with the starter.
  ///
  /// Returns the parsed `SettingsConfig`.
  fn read_settings_or_init(
    app_handle: &AppHandle,
    dir: &PathBuf,
  ) -> anyhow::Result<SettingsConfig> {
    let settings = Self::read_settings(&dir)?;

    match settings {
      Some(settings) => Ok(settings),
      None => {
        Self::create_from_examples(app_handle, dir)?;

        Self::read_settings(&dir)?.context("Failed to create settings config.")
      }
    }
  }

  /// Reads the global settings file.
  ///
  /// Returns the parsed `SettingsConfig` if found.
  fn read_settings(dir: &PathBuf) -> anyhow::Result<Option<SettingsConfig>> {
    let settings_path = dir.join("settings.json");

    match settings_path.exists() {
      false => Ok(None),
      true => read_and_parse_json(&settings_path),
    }
  }

  /// Aggregates all valid widget configs at the 2nd-level of the given
  /// directory (i.e. `<CONFIG_DIR>/*/*.edgebar.json`).
  ///
  /// Returns a hashmap of config paths to their `WidgetConfig` instances.
  fn read_widget_configs(dir: &PathBuf) -> anyhow::Result<HashMap<PathBuf, WidgetConfig>> {
    let dir_paths = fs::read_dir(dir)
      .with_context(|| format!("Failed to read directory: {}", dir.display()))?
      .filter_map(|entry| Some(entry.ok()?.path()));

    // Scan the 2nd-level of the config directory.
    let subdir_paths = dir_paths
      .filter(|path| path.is_dir())
      .filter_map(|dir| fs::read_dir(dir).ok())
      .flatten()
      .filter_map(|entry| Some(entry.ok()?.path()));

    // Collect the found config files.
    let config_paths = subdir_paths
      .filter(|path| path.is_file() && has_extension(&path, ".edgebar.json"))
      .collect::<Vec<PathBuf>>();

    let mut configs = HashMap::new();

    // Parse the found config files.
    for path in config_paths {
      match Self::parse_widget_config(&path) {
        Ok((config_path, config)) => {
          info!("Found valid widget config at: {}", config_path.display());
          configs.insert(config_path, config);
        }
        Err(err) => {
          error!("{:?}", err);
        }
      }
    }

    Ok(configs)
  }

  fn parse_widget_config(config_path: &PathBuf) -> anyhow::Result<(PathBuf, WidgetConfig)> {
    let abs_path = config_path
      .to_absolute()
      .with_context(|| format!("Invalid widget config path '{}'.", config_path.display()))?;

    let config = read_and_parse_json::<WidgetConfig>(&abs_path).map_err(|err| {
      anyhow::anyhow!(
        "Failed to parse widget config at '{}': {:?}",
        abs_path.display(),
        err
      )
    })?;

    Ok((abs_path, config))
  }

  /// Initializes settings and widget configs at the given path.
  ///
  /// `settings.json` is initialized with either `starter/vanilla` or
  /// `starter/with-glazewm` as startup config. Widget configs are
  /// initialized from `examples/` directory.
  fn create_from_examples(app_handle: &AppHandle, config_dir: &PathBuf) -> anyhow::Result<()> {
    let starter_path = app_handle
      .path()
      .resolve("../../examples", BaseDirectory::Resource)
      .context("Unable to resolve starter config resource.")?;

    info!(
      "Copying starter configs from {} to {}.",
      starter_path.display(),
      config_dir.display()
    );

    copy_dir_all(&starter_path, config_dir, false)?;

    let default_settings = SettingsConfig {
      schema: Some(
        "https://github.com/krishenriksen/edgebar/raw/v2.4.0/resources/menu-schema.json".into(),
      ),
      startup_configs: vec![StartupConfig {
        path: "macos/macos.edgebar.json".into(),
        preset: "default".into(),
      }],
    };

    let settings_path = config_dir.join("settings.json");
    fs::write(
      &settings_path,
      serde_json::to_string_pretty(&default_settings)? + "\n",
    )?;

    Ok(())
  }

  pub async fn widget_configs(&self) -> HashMap<PathBuf, WidgetConfig> {
    self.widget_configs.lock().await.clone()
  }

  /// Returns the widget configs to open on startup.
  pub async fn startup_configs(&self) -> Vec<StartupConfig> {
    self.settings.lock().await.startup_configs.clone()
  }

  /// Updates the widget config at the given path.
  ///
  /// Config path can be either absolute or relative.
  pub async fn update_widget_config(
    &self,
    config_path: &PathBuf,
    new_config: WidgetConfig,
  ) -> anyhow::Result<()> {
    info!("Updating widget config at {}.", config_path.display());

    {
      let mut widget_configs = self.widget_configs.lock().await;

      let config_entry = widget_configs.get_mut(config_path).context(format!(
        "Widget config not found at {}.",
        config_path.display()
      ))?;

      // Update the config in state.
      *config_entry = new_config.clone();
    }

    // Emit the changed config.
    self
      .widget_configs_change_tx
      .send(HashMap::from([(config_path.clone(), new_config.clone())]))?;

    // Write the updated config to file.
    fs::write(
      &config_path,
      serde_json::to_string_pretty(&new_config)? + "\n",
    )?;

    Ok(())
  }

  /// Joins the given path with the config directory path.
  ///
  /// Returns an absolute path.
  pub fn to_absolute_path(&self, config_path: &PathBuf) -> anyhow::Result<PathBuf> {
    match config_path.is_absolute() {
      false => self.config_dir.join(config_path).to_absolute(),
      // Ensure path is canonicalized even if already absolute.
      true => config_path.to_absolute(),
    }
  }

  /// Strips the config directory path from the given path.
  ///
  /// Returns a relative path.
  pub fn to_relative_path(&self, config_path: &PathBuf) -> PathBuf {
    config_path
      .strip_prefix(&self.config_dir)
      .unwrap_or(&config_path)
      .into()
  }

  /// Formats a widget's config path for display.
  ///
  /// Returns relative path without the `.edgebar.json` suffix (e.g.
  /// `starter/vanilla`).
  pub fn formatted_widget_path(&self, config_path: &PathBuf) -> String {
    let path = self.to_relative_path(config_path).to_unicode_string();

    // Ensure path delimiters are forward slashes on Windows.
    let path = path.replace('\\', "/");

    path.strip_suffix(".edgebar.json").unwrap_or(&path).into()
  }

  /// Returns the widget config at the given path.
  ///
  /// Config path can be either absolute or relative.
  pub async fn widget_config_by_path(
    &self,
    config_path: &PathBuf,
  ) -> Option<(PathBuf, WidgetConfig)> {
    let abs_path = self.to_absolute_path(config_path).ok()?;

    let widget_configs = self.widget_configs.lock().await;
    let config = widget_configs.get(&abs_path)?;

    Some((abs_path, config.clone()))
  }
}

/// Helper function for setting a default value for a boolean field.
const fn default_bool<const V: bool>() -> bool {
  V
}

/// Helper function for setting the default value for a
/// `WidgetPreset::name` field.
fn default_preset_name() -> String {
  "default".into()
}
