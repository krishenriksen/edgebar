pub mod menu;

pub use menu::*;

pub type Result<T> = std::result::Result<T, anyhow::Error>;
