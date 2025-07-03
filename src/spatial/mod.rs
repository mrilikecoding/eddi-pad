#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub struct SpatialZone {
    pub id: String,
    pub bounds: (Position, Position),
}

impl SpatialZone {
    pub fn contains(&self, position: &Position) -> bool {
        let (min, max) = &self.bounds;
        position.x >= min.x && position.x <= max.x
            && position.y >= min.y && position.y <= max.y
            && position.z >= min.z && position.z <= max.z
    }
}

pub struct SpatialController {
    zones: Vec<SpatialZone>,
}

impl SpatialController {
    pub fn new() -> Self {
        Self {
            zones: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(1.0, 2.0, 3.0);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn test_spatial_zone_contains() {
        let zone = SpatialZone {
            id: "test_zone".to_string(),
            bounds: (Position::new(0.0, 0.0, 0.0), Position::new(10.0, 10.0, 10.0)),
        };

        let inside_pos = Position::new(5.0, 5.0, 5.0);
        let outside_pos = Position::new(15.0, 5.0, 5.0);

        assert!(zone.contains(&inside_pos));
        assert!(!zone.contains(&outside_pos));
    }

    #[test]
    fn test_spatial_controller_creation() {
        let controller = SpatialController::new();
        assert_eq!(controller.zones.len(), 0);
    }
}