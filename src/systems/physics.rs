use crate::components::position::{Position, Velocity};

pub struct PhysicsSystem;

impl PhysicsSystem {
    pub fn update_position(pos: &mut Position, vel: &Velocity, dt: f32) {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }

    pub fn check_collision(
        a: &Position,
        a_width: f32,
        a_height: f32,
        b: &Position,
        b_width: f32,
        b_height: f32,
    ) -> bool {
        let a_left = a.x;
        let a_right = a.x + a_width;
        let a_top = a.y;
        let a_bottom = a.y + a_height;

        let b_left = b.x;
        let b_right = b.x + b_width;
        let b_top = b.y;
        let b_bottom = b.y + b_height;

        a_left < b_right && a_right > b_left && a_top < b_bottom && a_bottom > b_top
    }

    pub fn distance_between(a: &Position, b: &Position) -> f32 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_position() {
        let mut pos = Position::new(0.0, 0.0);
        let vel = Velocity::new(10.0, 5.0);
        PhysicsSystem::update_position(&mut pos, &vel, 1.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 5.0);
    }

    #[test]
    fn test_update_position_with_dt() {
        let mut pos = Position::new(0.0, 0.0);
        let vel = Velocity::new(10.0, 5.0);
        PhysicsSystem::update_position(&mut pos, &vel, 0.5);
        assert_eq!(pos.x, 5.0);
        assert_eq!(pos.y, 2.5);
    }

    #[test]
    fn test_collision_detection() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(25.0, 25.0);
        assert!(PhysicsSystem::check_collision(
            &a, 32.0, 32.0, &b, 32.0, 32.0
        ));
    }

    #[test]
    fn test_no_collision() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(100.0, 100.0);
        assert!(!PhysicsSystem::check_collision(
            &a, 32.0, 32.0, &b, 32.0, 32.0
        ));
    }

    #[test]
    fn test_distance() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(3.0, 4.0);
        assert!((PhysicsSystem::distance_between(&a, &b) - 5.0).abs() < f32::EPSILON);
    }
}
