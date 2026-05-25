use crate::components::position::Position;
use crate::systems::area_gen::{AreaGenerationSystem, GeneratedArea};

#[derive(Debug, Clone)]
pub struct Area {
    pub id: String,
    pub name: String,
    pub area_type: AreaType,
    pub width: usize,
    pub height: usize,
    pub recommended_level: u32,
    pub keywords: Vec<String>,
    pub generated_area: Option<GeneratedArea>,
    pub spawn_points: Vec<Position>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AreaType {
    RootTown,
    Field,
    Dungeon,
    Boss,
    ChaosGate,
}

pub struct AreaEntity;

impl AreaEntity {
    pub fn create_root_town() -> Area {
        Area {
            id: "mac_anu".to_string(),
            name: "Mac Anu".to_string(),
            area_type: AreaType::RootTown,
            width: 50,
            height: 50,
            recommended_level: 1,
            keywords: vec![
                "beginner".to_string(),
                "town".to_string(),
                "safe".to_string(),
            ],
            generated_area: None,
            spawn_points: vec![
                Position::new(400.0, 300.0),
                Position::new(500.0, 300.0),
                Position::new(400.0, 400.0),
            ],
        }
    }

    pub fn create_field(keywords: &[String], seed: u64) -> Area {
        let generated = AreaGenerationSystem::generate_area(30, 30, seed);
        Area {
            id: format!("field_{}", seed),
            name: format!("Field {}", seed),
            area_type: AreaType::Field,
            width: 30,
            height: 30,
            recommended_level: 1,
            keywords: keywords.to_vec(),
            generated_area: Some(generated),
            spawn_points: vec![Position::new(64.0, 64.0)],
        }
    }

    pub fn create_dungeon(keywords: &[String], seed: u64, level: u32) -> Area {
        let generated = AreaGenerationSystem::generate_area(40, 40, seed);
        Area {
            id: format!("dungeon_{}", seed),
            name: format!("Dungeon {}", seed),
            area_type: AreaType::Dungeon,
            width: 40,
            height: 40,
            recommended_level: level,
            keywords: keywords.to_vec(),
            generated_area: Some(generated),
            spawn_points: vec![Position::new(64.0, 64.0)],
        }
    }

    pub fn create_boss_arena(seed: u64) -> Area {
        let generated = AreaGenerationSystem::generate_area(20, 20, seed);
        Area {
            id: format!("boss_{}", seed),
            name: "Boss Arena".to_string(),
            area_type: AreaType::Boss,
            width: 20,
            height: 20,
            recommended_level: 10,
            keywords: vec!["boss".to_string(), "danger".to_string()],
            generated_area: Some(generated),
            spawn_points: vec![Position::new(64.0, 64.0)],
        }
    }

    pub fn get_area_keywords() -> Vec<Vec<&'static str>> {
        vec![
            vec!["grunty", "lake", "forest"],
            vec!["temple", "ruins", "desert"],
            vec!["mountain", "cave", "waterfall"],
            vec!["castle", "dungeon", "dark"],
            vec!["sky", "cloud", "wind"],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_root_town() {
        let area = AreaEntity::create_root_town();
        assert_eq!(area.name, "Mac Anu");
        assert_eq!(area.area_type, AreaType::RootTown);
        assert_eq!(area.spawn_points.len(), 3);
    }

    #[test]
    fn test_create_field() {
        let keywords = vec!["forest".to_string(), "lake".to_string()];
        let area = AreaEntity::create_field(&keywords, 42);
        assert_eq!(area.area_type, AreaType::Field);
        assert!(area.generated_area.is_some());
    }

    #[test]
    fn test_create_boss_arena() {
        let area = AreaEntity::create_boss_arena(99);
        assert_eq!(area.area_type, AreaType::Boss);
        assert_eq!(area.recommended_level, 10);
    }

    #[test]
    fn test_keywords_available() {
        let keyword_sets = AreaEntity::get_area_keywords();
        assert_eq!(keyword_sets.len(), 5);
        assert!(keyword_sets[0].contains(&"grunty"));
    }
}
