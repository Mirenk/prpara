pub mod core;
pub mod error;

pub use nix;

pub type Result<T> = std::result::Result<T, error::Error>;
