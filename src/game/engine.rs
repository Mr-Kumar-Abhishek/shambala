use crate::core::types::GameState;
use crate::core::game_state::GameStateManager;
use crate::resources::time::GameTime;
use crate::resources::input_state::InputStateResource;
use crate::resources::camera::Camera;
use crate::resources::asset_manager::AssetManager;
use crate::resources::audio_manager::AudioManager;
use crate::core::constants;

pub struct GameEngine {
    pub state_manager: GameStateManager,
    pub time: GameTime,
    pub input: InputStateResource,
    pub camera: Camera,
    pub assets: AssetManager,
    pub audio: AudioManager,
    pub running: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            state_manager: GameStateManager::new(),
            time: GameTime::new(),
            input: InputStateResource::new(),
            camera: Camera::new(0.0, 0.0, constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT),
            assets: AssetManager::new(),
            audio: AudioManager::new(),
            running: true,
        }
    }

    pub fn initialize(&mut self) {
        log::info!("Initializing game engine...");
        self.state_manager.transition_to(GameState::Boot);
        self.register_default_assets();
        log::info!("Game engine initialized");
    }

    fn register_default_assets(&mut self) {
        // Player sprites
        for class in &["TwinBlade", "HeavyBlade", "LongArm", "Wavemaster"] {
            self.assets.register_texture(
                &format!("player_{}", class),
                &format!("assets/sprites/player/{}.png", class),
            );
        }
        // Enemy sprites
        for enemy in &["Goblin", "Wolf", "Skeleton", "Mage", "Boss"] {
            self.assets.register_texture(
                &format!("enemy_{}", enemy),
                &format!("assets/sprites/enemy/{}.png", enemy),
            );
        }
        // UI assets
        self.assets.register_texture("hud_frame", "assets/ui/hud_frame.png");
        self.assets.register_texture("hp_bar", "assets/ui/hp_bar.png");
        self.assets.register_texture("mp_bar", "assets/ui/mp_bar.png");
        // Audio
        self.assets.register_texture("bgm_root_town", "assets/audio/bgm/root_town.ogg");
        self.assets.register_texture("bgm_field", "assets/audio/bgm/field.ogg");
    }

    pub fn update(&mut self, dt: f32) {
        self.time.update(dt);
        self.input.clear_frame();
        self.audio_manager_update();
    }

    fn audio_manager_update(&mut self) {
        // Update audio based on current state
        let bgm = match self.state_manager.current() {
            GameState::Title | GameState::Menu => "bgm_menu",
            GameState::Exploring => "bgm_field",
            GameState::Combat => "bgm_boss",
            _ => "bgm_default",
        };
        self.audio.play_bgm(bgm);
    }

    pub fn shutdown(&mut self) {
        log::info!("Shutting down game engine...");
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = GameEngine::new();
        assert!(engine.running);
        assert_eq!(engine.state_manager.current(), GameState::Boot);
    }

    #[test]
    fn test_engine_initialize() {
        let mut engine = GameEngine::new();
        engine.initialize();
        assert!(engine.assets.textures.contains_key("player_TwinBlade"));
    }

    #[test]
    fn test_engine_update() {
        let mut engine = GameEngine::new();
        engine.initialize();
        engine.update(0.016);
        assert!(engine.time.frame_count > 0);
    }

    #[test]
    fn test_engine_shutdown() {
        let mut engine = GameEngine::new();
        engine.shutdown();
        assert!(!engine.running);
    }
}