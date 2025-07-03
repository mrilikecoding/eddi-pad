use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Device connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Device command failed: {0}")]
    CommandFailed(String),
}

pub trait Device {
    fn send_command(&mut self, command: &str) -> Result<(), DeviceError>;
}

pub struct DeviceManager {
    devices: Vec<Box<dyn Device>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDevice {
        should_fail: bool,
    }

    impl MockDevice {
        fn new(should_fail: bool) -> Self {
            Self { should_fail }
        }
    }

    impl Device for MockDevice {
        fn send_command(&mut self, _command: &str) -> Result<(), DeviceError> {
            if self.should_fail {
                Err(DeviceError::CommandFailed("Mock failure".to_string()))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_device_manager_creation() {
        let manager = DeviceManager::new();
        assert_eq!(manager.devices.len(), 0);
    }

    #[test]
    fn test_mock_device_success() {
        let mut device = MockDevice::new(false);
        assert!(device.send_command("test").is_ok());
    }

    #[test]
    fn test_mock_device_failure() {
        let mut device = MockDevice::new(true);
        assert!(device.send_command("test").is_err());
    }
}