//! Eddi-pad: High-performance simulation environment for interactive installations
//!
//! This library provides a unified interface for controlling various devices and protocols
//! commonly used in interactive installations, including DMX lighting, OSC communication,
//! and Bluetooth devices.

pub mod device;
pub mod spatial;
pub mod protocols;
pub mod ui;
pub mod config;

pub use device::{Device, DeviceManager, DeviceError};
pub use spatial::{SpatialZone, SpatialController, Position};
pub use config::Config;

/// Core result type for the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}