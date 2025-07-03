use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub device_settings: DeviceSettings,
    pub spatial_settings: SpatialSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceSettings {
    pub dmx_port: Option<String>,
    pub osc_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpatialSettings {
    pub zone_count: usize,
    pub default_bounds: (f32, f32, f32),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device_settings: DeviceSettings {
                dmx_port: None,
                osc_port: 8000,
            },
            spatial_settings: SpatialSettings {
                zone_count: 4,
                default_bounds: (10.0, 10.0, 3.0),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.device_settings.osc_port, 8000);
        assert_eq!(config.spatial_settings.zone_count, 4);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());
    }
}