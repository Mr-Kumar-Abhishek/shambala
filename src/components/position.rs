use serde::{Deserialize, Serialize};

/// 2D world-space position.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    /// Create a new position at `(x, y)`.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Euclidean distance to another position.
    pub fn distance_to(&self, other: &Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// 2D velocity vector, used during exploration movement.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    /// Create a new velocity vector `(x, y)`.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Magnitude (speed) of this velocity vector.
    pub fn speed(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(10.0, 20.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);
    }

    #[test]
    fn test_distance_to() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(3.0, 4.0);
        assert!((a.distance_to(&b) - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_velocity_speed() {
        let vel = Velocity::new(3.0, 4.0);
        assert!((vel.speed() - 5.0).abs() < f32::EPSILON);
    }
}
