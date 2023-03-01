pub mod core;
pub mod types;

pub use nix;

pub type Result<T> = std::result::Result<T, types::Error>;
