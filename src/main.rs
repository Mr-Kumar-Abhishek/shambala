use shambala::core::constants;
use shambala::game::engine::GameEngine;

fn main() {
    // Initialize logging
    env_logger::init();
    
    log::info!("Starting {} v{}", constants::TITLE, "0.1.0");
    log::info!("Window: {}x{}", constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT);
    
    // Initialize game engine
    let mut engine = GameEngine::new();
    engine.initialize();
    
    println!("╔══════════════════════════════════════╗");
    println!("║           SHAMBALA v0.1.0            ║");
    println!("║                                      ║");
    println!("║  A story-driven action RPG inspired  ║");
    println!("║  by .hack//sign                      ║");
    println!("║                                      ║");
    println!("║  Press Ctrl+C to exit                ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("Game Engine: Initialized");
    println!("State: {:?}", engine.state_manager.current());
    println!("Assets Registered: {}", engine.assets.textures.len());
    println!("Audio Clips: {}", engine.assets.audio.len());
    println!();
    println!("Game initialized successfully!");
    
    // TODO: Start game loop with winit/wgpu
    // TODO: Implement main game loop
}
