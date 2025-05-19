pub mod window;

pub use window::*;

pub type Result<T> = std::result::Result<T, anyhow::Error>;
