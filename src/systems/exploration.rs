use crate::components::position::{Position, Velocity};
use crate::components::enemy::Enemy;
use crate::systems::physics::PhysicsSystem;
use crate::systems::input::InputAction;
use crate::resources::input_state::InputStateResource;
use crate::resources::camera::Camera;
use crate::entities::area::Area;
use crate::core::constants;

pub const PLAYER_SPEED: f32 = 200.0;

pub struct ExplorationSystem;

impl ExplorationSystem {
    pub fn handle_movement(
        player_pos: &mut Position,
        player_vel: &mut Velocity,
        input: &InputStateResource,
        dt: f32,
    ) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if input.is_action_held(InputAction::MoveUp) {
            dy -= 1.0;
        }
        if input.is_action_held(InputAction::MoveDown) {
            dy += 1.0;
        }
        if input.is_action_held(InputAction::MoveLeft) {
            dx -= 1.0;
        }
        if input.is_action_held(InputAction::MoveRight) {
            dx += 1.0;
        }

        // Normalize diagonal
        if dx != 0.0 && dy != 0.0 {
            let len = f32::sqrt(dx * dx + dy * dy);
            dx /= len;
            dy /= len;
        }

        player_vel.x = dx * PLAYER_SPEED;
        player_vel.y = dy * PLAYER_SPEED;

        PhysicsSystem::update_position(player_pos, player_vel, dt);
    }

    pub fn check_area_boundaries(pos: &mut Position, area: &Area) {
        let max_x = (area.width as f32) * constants::TILE_SIZE - constants::TILE_SIZE;
        let max_y = (area.height as f32) * constants::TILE_SIZE - constants::TILE_SIZE;
        
        pos.x = pos.x.clamp(constants::TILE_SIZE, max_x);
        pos.y = pos.y.clamp(constants::TILE_SIZE, max_y);
    }

    pub fn check_wall_collision(pos: &Position, area: &Area) -> bool {
        if let Some(ref generated) = area.generated_area {
            let tile_x = (pos.x / constants::TILE_SIZE) as usize;
            let tile_y = (pos.y / constants::TILE_SIZE) as usize;
            
            if tile_x < area.width && tile_y < area.height {
                return !generated.tiles[tile_y][tile_x].walkable;
            }
        }
        false
    }

    pub fn check_enemy_proximity(
        player_pos: &Position,
        enemies: &[(Enemy, Position)],
        aggro_range: f32,
    ) -> Vec<usize> {
        enemies.iter()
            .enumerate()
            .filter(|(_, (_, pos))| {
                PhysicsSystem::distance_between(player_pos, pos) <= aggro_range
            })
            .map(|(i, _)| i)
            .collect()
    }

    pub fn check_area_exit(pos: &Position, area: &Area) -> Option<(usize, usize)> {
        if let Some(ref generated) = area.generated_area {
            let tile_x = (pos.x / constants::TILE_SIZE) as usize;
            let tile_y = (pos.y / constants::TILE_SIZE) as usize;
            
            if tile_x < area.width && tile_y < area.height {
                if tile_x == generated.exit.0 && tile_y == generated.exit.1 {
                    return Some((tile_x, tile_y));
                }
            }
        }
        None
    }

    pub fn update_camera(camera: &mut Camera, target: &Position, dt: f32) {
        let lerp_speed = 5.0 * dt;
        camera.follow(target.x, target.y, lerp_speed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::input::InputState;
    use crate::entities::area::AreaEntity;

    #[test]
    fn test_movement() {
        let mut pos = Position::new(400.0, 300.0);
        let mut vel = Velocity::new(0.0, 0.0);
        let mut input = InputStateResource::new();
        
        input.set_action(InputAction::MoveRight, InputState::Held);
        ExplorationSystem::handle_movement(&mut pos, &mut vel, &input, 1.0);
        
        assert!(pos.x > 400.0);
        assert_eq!(pos.y, 300.0);
    }

    #[test]
    fn test_diagonal_movement() {
        let mut pos = Position::new(400.0, 300.0);
        let mut vel = Velocity::new(0.0, 0.0);
        let mut input = InputStateResource::new();
        
        input.set_action(InputAction::MoveRight, InputState::Held);
        input.set_action(InputAction::MoveDown, InputState::Held);
        ExplorationSystem::handle_movement(&mut pos, &mut vel, &input, 1.0);
        
        assert!(pos.x > 400.0);
        assert!(pos.y > 300.0);
    }

    #[test]
    fn test_area_boundaries() {
        let area = AreaEntity::create_root_town();
        let mut pos = Position::new(-100.0, -100.0);
        ExplorationSystem::check_area_boundaries(&mut pos, &area);
        assert!(pos.x >= constants::TILE_SIZE);
        assert!(pos.y >= constants::TILE_SIZE);
    }

    #[test]
    fn test_wall_collision() {
        let area = AreaEntity::create_field(&["test".to_string()], 42);
        // Check that walls are detected
        let pos = Position::new(0.0, 0.0); // Wall at border
        let collision = ExplorationSystem::check_wall_collision(&pos, &area);
        // If tile at 0,0 is a wall, should be true
        if let Some(ref gen) = area.generated_area {
            assert_eq!(collision, !gen.tiles[0][0].walkable);
        }
    }

    #[test]
    fn test_no_movement_without_input() {
        let mut pos = Position::new(400.0, 300.0);
        let mut vel = Velocity::new(0.0, 0.0);
        let input = InputStateResource::new();
        
        ExplorationSystem::handle_movement(&mut pos, &mut vel, &input, 1.0);
        assert_eq!(pos.x, 400.0);
        assert_eq!(pos.y, 300.0);
    }

    #[test]
    fn test_camera_follow() {
        let mut camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        let target = Position::new(640.0, 360.0);
        
        ExplorationSystem::update_camera(&mut camera, &target, 1.0);
        assert_eq!(camera.x, target.x - 640.0);
        assert_eq!(camera.y, target.y - 360.0);
    }

    #[test]
    fn test_enemy_proximity() {
        let player = Position::new(0.0, 0.0);
        let enemies = vec![
            (Enemy::new(crate::components::enemy::EnemyType::Goblin), Position::new(50.0, 0.0)),
            (Enemy::new(crate::components::enemy::EnemyType::Goblin), Position::new(500.0, 0.0)),
        ];
        
        let nearby = ExplorationSystem::check_enemy_proximity(&player, &enemies, 100.0);
        assert_eq!(nearby.len(), 1);
        assert_eq!(nearby[0], 0);
    }

    #[test]
    fn test_area_exit() {
        let area = AreaEntity::create_field(&["test".to_string()], 42);
        let exit_pos = if let Some(ref gen) = area.generated_area {
            Position::new(
                gen.exit.0 as f32 * constants::TILE_SIZE,
                gen.exit.1 as f32 * constants::TILE_SIZE,
            )
        } else {
            Position::new(0.0, 0.0)
        };
        
        let exit = ExplorationSystem::check_area_exit(&exit_pos, &area);
        assert!(exit.is_some());
    }
}