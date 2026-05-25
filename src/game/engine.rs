use crate::core::constants;
use crate::core::game_state::GameStateManager;
use crate::core::types::GameState;
use crate::game::chaos_gate::ChaosGate;
use crate::game::quest::QuestManager;
use crate::game::title_screen::TitleScreen;
use crate::render::pipeline::RenderPipeline;
use crate::render::sprite::SpriteBatch;
use crate::render::ui_render::UIRenderer;
use crate::resources::asset_manager::AssetManager;
use crate::resources::audio_manager::AudioManager;
use crate::resources::camera::Camera;
use crate::resources::input_state::InputStateResource;
use crate::resources::time::GameTime;

/// Central game engine that owns all resources and orchestrates the update loop.
///
/// The engine owns:
/// - **Core resources**: state machine, time, input, camera
/// - **Asset systems**: texture/audio asset manager, audio manager
/// - **Rendering**: optional wgpu pipeline, sprite batch, UI renderer
/// - **Game screens**: title screen, chaos gate, quest manager
///
/// # Lifecycle
///
/// 1. [`new()`](Self::new) — create the engine (starts in Boot state)
/// 2. [`initialize()`](Self::initialize) — register default assets
/// 3. [`update()`](Self::update) — advance time, clear frame input, update audio
/// 4. [`shutdown()`](Self::shutdown) — graceful shutdown
pub struct GameEngine<'window> {
    pub state_manager: GameStateManager,
    pub time: GameTime,
    pub input: InputStateResource,
    pub camera: Camera,
    pub assets: AssetManager,
    pub audio: AudioManager,
    pub render_pipeline: Option<RenderPipeline<'window>>,
    pub sprite_batch: SpriteBatch,
    pub ui_renderer: UIRenderer,
    pub running: bool,
    pub title_screen: TitleScreen,
    pub chaos_gate: ChaosGate,
    pub quest_manager: QuestManager,
}

impl<'window> GameEngine<'window> {
    /// Create a new engine in the Boot state.
    ///
    /// Registers the tutorial quest automatically.
    pub fn new() -> Self {
        let mut quest_manager = QuestManager::new();
        quest_manager.register_quest(QuestManager::create_tutorial_quest());

        Self {
            state_manager: GameStateManager::new(),
            time: GameTime::new(),
            input: InputStateResource::new(),
            camera: Camera::new(0.0, 0.0, constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT),
            assets: AssetManager::new(),
            audio: AudioManager::new(),
            render_pipeline: None,
            sprite_batch: SpriteBatch::new(),
            ui_renderer: UIRenderer::new(),
            running: true,
            title_screen: TitleScreen::new(),
            chaos_gate: ChaosGate::new(),
            quest_manager,
        }
    }

    /// Initialise the engine: transition to Boot and register default assets.
    ///
    /// Must be called after [`new()`](Self::new) and before the event loop starts.
    pub fn initialize(&mut self) {
        log::info!("Initializing game engine...");
        self.state_manager.transition_to(GameState::Boot);
        self.register_default_assets();
        log::info!("Game engine initialized");
    }

    /// Attach a wgpu render pipeline to the engine.
    ///
    /// Must be called after the window is created but before the main loop.
    pub fn set_render_pipeline(&mut self, pipeline: RenderPipeline<'window>) {
        self.render_pipeline = Some(pipeline);
    }

    /// Register placeholder asset paths for sprites, tiles, UI, and audio.
    ///
    /// These paths are registered so the engine can reference them by key;
    /// actual file loading requires disk I/O at runtime.
    fn register_default_assets(&mut self) {
        // Player sprites
        for class in &["TwinBlade", "HeavyBlade", "LongArm", "Wavemaster"] {
            self.assets.register_texture(
                &format!("player_{}", class),
                &format!("assets/sprites/player/{}.svg", class),
            );
        }
        // Enemy sprites
        for enemy in &["Goblin", "Wolf", "Skeleton", "Mage", "Boss"] {
            self.assets.register_texture(
                &format!("enemy_{}", enemy),
                &format!("assets/sprites/enemy/{}.svg", enemy),
            );
        }
        // Tile textures
        for tile in &[
            "floor", "wall", "water", "grass", "path", "entrance", "exit", "treasure",
        ] {
            self.assets.register_texture(
                &format!("tile_{}", tile),
                &format!("assets/tiles/{}.svg", tile),
            );
        }
        // UI assets
        self.assets
            .register_texture("hud_frame", "assets/ui/hud_frame.svg");
        self.assets
            .register_texture("hp_bar", "assets/ui/hp_bar.svg");
        self.assets
            .register_texture("mp_bar", "assets/ui/mp_bar.svg");
        // Audio
        self.assets
            .register_texture("bgm_root_town", "assets/audio/bgm/root_town.ogg");
        self.assets
            .register_texture("bgm_field", "assets/audio/bgm/field.ogg");
    }

    /// Advance the engine by one frame.
    ///
    /// Updates the game clock, clears one-shot input states, and
    /// adjusts BGM based on the current game state.
    pub fn update(&mut self, dt: f32) {
        self.time.update(dt);
        self.input.clear_frame();
        self.audio_manager_update();
    }

    /// Switch BGM track based on the current [`GameState`].
    fn audio_manager_update(&mut self) {
        let bgm = match self.state_manager.current() {
            GameState::Title | GameState::Menu => "bgm_menu",
            GameState::Exploring => "bgm_field",
            GameState::Combat => "bgm_boss",
            _ => "bgm_default",
        };
        self.audio.play_bgm(bgm);
    }

    /// Gracefully shut down the engine (sets `running = false`).
    pub fn shutdown(&mut self) {
        log::info!("Shutting down game engine...");
        self.running = false;
    }
}

impl Default for GameEngine<'_> {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_sprite_batch_in_engine() {
        let engine = GameEngine::new();
        assert_eq!(engine.sprite_batch.sprite_count(), 0);
    }
}
