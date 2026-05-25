use crate::core::types::GameState;

#[derive(Debug, Clone)]
pub struct Scene {
    pub id: String,
    pub scene_type: SceneType,
    pub entities: Vec<String>,
    pub is_loaded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneType {
    Title,
    CharacterSelect,
    RootTown,
    Field,
    Dungeon,
    Boss,
    Menu,
    Cutscene,
}

pub struct SceneManager {
    pub current_scene: Option<Scene>,
    pub scenes: Vec<Scene>,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: None,
            scenes: Vec::new(),
        }
    }

    pub fn register_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn load_scene(&mut self, scene_id: &str) -> Result<(), &str> {
        if let Some(scene) = self.scenes.iter_mut().find(|s| s.id == scene_id) {
            scene.is_loaded = true;
            self.current_scene = Some(scene.clone());
            Ok(())
        } else {
            Err("Scene not found")
        }
    }

    pub fn unload_current(&mut self) {
        if let Some(ref mut scene) = self.current_scene {
            scene.is_loaded = false;
        }
        self.current_scene = None;
    }

    pub fn scene_for_state(state: GameState) -> Option<SceneType> {
        match state {
            GameState::Title => Some(SceneType::Title),
            GameState::CharacterSelect => Some(SceneType::CharacterSelect),
            GameState::Exploring => Some(SceneType::Field),
            GameState::Combat => Some(SceneType::Boss),
            GameState::Menu => Some(SceneType::Menu),
            GameState::Cutscene => Some(SceneType::Cutscene),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_scene() {
        let mut manager = SceneManager::new();
        let scene = Scene {
            id: "mac_anu".to_string(),
            scene_type: SceneType::RootTown,
            entities: vec![],
            is_loaded: false,
        };
        manager.register_scene(scene);
        assert_eq!(manager.scenes.len(), 1);
    }

    #[test]
    fn test_load_scene() {
        let mut manager = SceneManager::new();
        manager.register_scene(Scene {
            id: "field_1".to_string(),
            scene_type: SceneType::Field,
            entities: vec![],
            is_loaded: false,
        });
        assert!(manager.load_scene("field_1").is_ok());
        assert!(manager.current_scene.unwrap().is_loaded);
    }

    #[test]
    fn test_load_nonexistent_scene() {
        let mut manager = SceneManager::new();
        assert!(manager.load_scene("nonexistent").is_err());
    }

    #[test]
    fn test_scene_for_state() {
        assert_eq!(
            SceneManager::scene_for_state(GameState::Exploring),
            Some(SceneType::Field)
        );
        assert_eq!(SceneManager::scene_for_state(GameState::Boot), None);
    }
}
