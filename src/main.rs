use shambala::core::constants;

fn main() {
    // Initialize logging
    env_logger::init();
    
    log::info!("Starting {} v{}", constants::TITLE, "0.1.0");
    log::info!("Window: {}x{}", constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT);
    
    // For now, just print game info
    println!("{} v0.1.0", constants::TITLE);
    println!("Window: {}x{}", constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT);
    println!("Game initialized successfully!");
    
    // TODO: Initialize game engine
    // TODO: Start game loop
}
