use crate::components::enemy::{Enemy, EnemyType};
use crate::components::position::Position;

pub struct AISystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
    Return,
}

impl AISystem {
    pub fn determine_state(enemy: &Enemy, enemy_pos: &Position, player_pos: &Position) -> AIState {
        let distance = enemy_pos.distance_to(player_pos);

        if distance <= enemy.aggro_range * 0.5 {
            AIState::Attack
        } else if distance <= enemy.aggro_range {
            AIState::Chase
        } else if enemy.is_aggroed {
            AIState::Return
        } else {
            match enemy.enemy_type {
                EnemyType::Goblin | EnemyType::Wolf => AIState::Patrol,
                EnemyType::Skeleton | EnemyType::Mage => AIState::Idle,
                EnemyType::Boss => AIState::Patrol,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idle_state() {
        let enemy = Enemy::new(EnemyType::Skeleton);
        let enemy_pos = Position::new(0.0, 0.0);
        let player_pos = Position::new(500.0, 500.0);
        let state = AISystem::determine_state(&enemy, &enemy_pos, &player_pos);
        assert_eq!(state, AIState::Idle);
    }

    #[test]
    fn test_chase_state() {
        let enemy = Enemy::new(EnemyType::Wolf);
        let enemy_pos = Position::new(0.0, 0.0);
        let player_pos = Position::new(150.0, 0.0);
        let state = AISystem::determine_state(&enemy, &enemy_pos, &player_pos);
        assert_eq!(state, AIState::Chase);
    }

    #[test]
    fn test_attack_state() {
        let enemy = Enemy::new(EnemyType::Goblin);
        let enemy_pos = Position::new(0.0, 0.0);
        let player_pos = Position::new(30.0, 0.0);
        let state = AISystem::determine_state(&enemy, &enemy_pos, &player_pos);
        assert_eq!(state, AIState::Attack);
    }
}
