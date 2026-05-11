use shambala::core::constants;
use shambala::game::engine::GameEngine;

use winit::event_loop::EventLoop;
use winit::window::WindowAttributes;
use winit::event::{Event, WindowEvent};

fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();
    
    log::info!("Starting {} v{}", constants::TITLE, "0.2.0");
    log::info!("Window: {}x{}", constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT);
    
    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = event_loop.create_window(
        WindowAttributes::default()
            .with_title(constants::TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(
                constants::WINDOW_WIDTH,
                constants::WINDOW_HEIGHT,
            ))
    )?;
    
    // Initialize game engine
    let mut engine = GameEngine::new();
    engine.initialize();
    
    println!("╔══════════════════════════════════════╗");
    println!("║           SHAMBALA v0.2.0            ║");
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
    
    // Run the event loop
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    engine.shutdown();
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    if let Some(ref mut pipeline) = engine.render_pipeline {
                        pipeline.resize(size);
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;
    
    Ok(())
}
