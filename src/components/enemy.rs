#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Goblin,
    Wolf,
    Skeleton,
    Mage,
    Boss,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub aggro_range: f32,
    pub is_aggroed: bool,
    pub drop_table_id: String,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        let aggro_range = match enemy_type {
            EnemyType::Goblin => 100.0,
            EnemyType::Wolf => 200.0,
            EnemyType::Skeleton => 150.0,
            EnemyType::Mage => 120.0,
            EnemyType::Boss => 300.0,
        };
        Self {
            enemy_type,
            aggro_range,
            is_aggroed: false,
            drop_table_id: format!("{:?}_drops", enemy_type),
        }
    }
}