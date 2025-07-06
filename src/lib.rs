//! Eddi-pad: High-performance simulation environment for interactive installations
//!
//! This library provides a unified interface for controlling various devices and protocols
//! commonly used in interactive installations, including DMX lighting, OSC communication,
//! and Bluetooth devices.

/// Core result type for the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Basic test to verify compilation
pub fn hello() -> &'static str {
    "Hello from eddi-pad!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello from eddi-pad!");
    }
}