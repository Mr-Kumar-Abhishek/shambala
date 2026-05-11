use crate::components::enemy::{Enemy, EnemyType};
use crate::components::position::Position;
use crate::components::stats::Stats;
use crate::components::render::{Renderable, RenderLayer};
use crate::components::inventory::Item;
use crate::core::types::Class;

pub struct EnemyEntity;

impl EnemyEntity {
    pub fn create(enemy_type: EnemyType, x: f32, y: f32) -> (Enemy, Stats, Position, Renderable) {
        let enemy = Enemy::new(enemy_type);
        let stats = Self::create_stats(enemy_type);
        let position = Position::new(x, y);
        let renderable = Renderable::new(
            &format!("enemy_{:?}", enemy_type),
            RenderLayer::Characters,
        );

        (enemy, stats, position, renderable)
    }

    fn create_stats(enemy_type: EnemyType) -> Stats {
        let mut stats = Stats::new(Class::HeavyBlade);
        match enemy_type {
            EnemyType::Goblin => {
                stats.hp = 30; stats.max_hp = 30;
                stats.attack = 8; stats.defense = 4;
                stats.experience = 15;
            }
            EnemyType::Wolf => {
                stats.hp = 25; stats.max_hp = 25;
                stats.attack = 12; stats.defense = 3;
                stats.agility = 25;
                stats.experience = 20;
            }
            EnemyType::Skeleton => {
                stats.hp = 45; stats.max_hp = 45;
                stats.attack = 10; stats.defense = 8;
                stats.experience = 30;
            }
            EnemyType::Mage => {
                stats.hp = 20; stats.max_hp = 20;
                stats.magic_attack = 18; stats.magic_defense = 10;
                stats.experience = 35;
            }
            EnemyType::Boss => {
                stats.hp = 500; stats.max_hp = 500;
                stats.attack = 30; stats.defense = 20;
                stats.magic_attack = 25; stats.magic_defense = 20;
                stats.level = 10;
                stats.experience = 500;
            }
        }
        stats
    }

    pub fn get_drop_table(enemy_type: EnemyType) -> Vec<Item> {
        match enemy_type {
            EnemyType::Goblin => vec![
                Item { id: "goblin_ear".to_string(), name: "Goblin Ear".to_string(), quantity: 1, max_stack: 99, item_type: crate::components::inventory::ItemType::Material },
            ],
            EnemyType::Wolf => vec![
                Item { id: "wolf_fang".to_string(), name: "Wolf Fang".to_string(), quantity: 1, max_stack: 99, item_type: crate::components::inventory::ItemType::Material },
            ],
            EnemyType::Skeleton => vec![
                Item { id: "bone_fragment".to_string(), name: "Bone Fragment".to_string(), quantity: 1, max_stack: 99, item_type: crate::components::inventory::ItemType::Material },
            ],
            EnemyType::Mage => vec![
                Item { id: "magic_crystal".to_string(), name: "Magic Crystal".to_string(), quantity: 1, max_stack: 99, item_type: crate::components::inventory::ItemType::Material },
            ],
            EnemyType::Boss => vec![
                Item { id: "boss_core".to_string(), name: "Boss Core".to_string(), quantity: 1, max_stack: 1, item_type: crate::components::inventory::ItemType::KeyItem },
                Item { id: "rare_weapon".to_string(), name: "Rare Weapon".to_string(), quantity: 1, max_stack: 1, item_type: crate::components::inventory::ItemType::Weapon },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_goblin() {
        let (enemy, stats, pos, _) = EnemyEntity::create(EnemyType::Goblin, 100.0, 200.0);
        assert_eq!(enemy.enemy_type, EnemyType::Goblin);
        assert_eq!(stats.hp, 30);
        assert_eq!(pos.x, 100.0);
    }

    #[test]
    fn test_create_boss() {
        let (_, stats, _, _) = EnemyEntity::create(EnemyType::Boss, 0.0, 0.0);
        assert_eq!(stats.level, 10);
        assert_eq!(stats.hp, 500);
    }

    #[test]
    fn test_drop_tables() {
        let drops = EnemyEntity::get_drop_table(EnemyType::Boss);
        assert_eq!(drops.len(), 2);
        assert!(drops.iter().any(|i| i.id == "boss_core"));
    }

    #[test]
    fn test_all_enemies_have_drops() {
        for e in &[EnemyType::Goblin, EnemyType::Wolf, EnemyType::Skeleton, EnemyType::Mage, EnemyType::Boss] {
            let drops = EnemyEntity::get_drop_table(*e);
            assert!(!drops.is_empty(), "{:?} has no drops", e);
        }
    }
}
