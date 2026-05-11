use shambala::game::title_screen::{TitleScreen, MenuOption};
use shambala::game::chaos_gate::ChaosGate;
use shambala::game::quest::QuestManager;
use shambala::game::event::QuestStatus;
use shambala::game::engine::GameEngine;
use shambala::render::sprite::{SpriteBatch, SpriteInstance};
use shambala::render::tilemap::Tilemap;
use shambala::render::ui_render::UIRenderer;
use shambala::render::text::TextRenderer;
use shambala::components::position::Position;
use shambala::components::render::RenderLayer;
use shambala::resources::camera::Camera;
use shambala::systems::area_gen::AreaGenerationSystem;
use shambala::core::types::GameState;

#[test]
fn test_title_to_character_select_flow() {
    let mut engine = GameEngine::new();
    engine.initialize();

    // Simulate title screen navigation
    engine.title_screen.reset();
    assert!(engine.title_screen.visible);

    // Navigate down and confirm
    engine.title_screen.selected_index = 0; // New Game

    // Simulate confirm
    TitleScreen::handle_selection(MenuOption::NewGame, &mut engine.state_manager);
    assert_eq!(engine.state_manager.current(), GameState::CharacterSelect);
}

#[test]
fn test_chaos_gate_full_flow() {
    let mut gate = ChaosGate::new();

    // Activate gate
    gate.activate();
    assert!(gate.is_active);

    // Select 3 keywords
    gate.select_keyword("forest");
    gate.select_keyword("lake");
    gate.select_keyword("temple");
    assert!(gate.is_ready());

    // Confirm area
    gate.confirm_area(42);
    assert!(gate.is_transitioning);
    assert!(gate.generated_area.is_some());

    // Complete transition
    let mut progress = 0.0;
    while progress < 2.0 {
        if gate.update_transition(0.5) {
            break;
        }
        progress += 0.5;
    }
    assert!(!gate.is_transitioning);
    assert_eq!(gate.transition_progress, 1.0);
}

#[test]
fn test_quest_lifecycle() {
    let mut manager = QuestManager::new();
    let quest = QuestManager::create_tutorial_quest();
    let quest_id = quest.id.clone();
    manager.register_quest(quest);

    // Start quest
    assert!(manager.start_quest(&quest_id).is_ok());
    assert_eq!(manager.active_quests.len(), 1);

    // Complete objectives one by one
    manager.update_objective(&quest_id, "talk_to_helba", 1);
    let quest = manager.get_quest(&quest_id).unwrap();
    assert!(quest.objectives[0].is_complete);
    assert_eq!(quest.status, QuestStatus::InProgress);

    manager.update_objective(&quest_id, "use_chaos_gate", 1);
    manager.update_objective(&quest_id, "defeat_goblins", 3);

    let quest = manager.get_quest(&quest_id).unwrap();
    assert_eq!(quest.status, QuestStatus::Completed);
    assert!(manager.active_quests.is_empty());
    assert_eq!(manager.completed_quests.len(), 1);
}

#[test]
fn test_sprite_batch_with_camera() {
    let mut batch = SpriteBatch::new();
    let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);

    // Add sprites at various positions
    batch.add_sprite(SpriteInstance {
        texture_id: "player".to_string(),
        position: Position::new(640.0, 360.0),
        size: (32.0, 32.0),
        layer: RenderLayer::Characters,
        color: [1.0, 1.0, 1.0, 1.0],
        visible: true,
    });

    batch.add_sprite(SpriteInstance {
        texture_id: "enemy".to_string(),
        position: Position::new(-100.0, -100.0),
        size: (32.0, 32.0),
        layer: RenderLayer::Characters,
        color: [1.0, 1.0, 1.0, 1.0],
        visible: true,
    });

    // Only player should be visible
    assert_eq!(batch.visible_count(&camera), 1);

    // Test texture grouping
    let visible = batch.collect_visible(&camera);
    let groups = SpriteBatch::group_by_texture(&visible);
    assert_eq!(groups.len(), 1);
}

#[test]
fn test_tilemap_from_generated_area() {
    let area = AreaGenerationSystem::generate_area(15, 15, 100);
    let tilemap = Tilemap::from_generated_area(&area, 32.0);

    assert_eq!(tilemap.width, 15);
    assert_eq!(tilemap.height, 15);

    // Test coordinate conversion
    let (tx, ty) = tilemap.world_to_tile(64.0, 96.0);
    assert_eq!(tx, 2);
    assert_eq!(ty, 3);

    let (wx, wy) = tilemap.tile_to_world(tx, ty);
    assert_eq!(wx, 64.0);
    assert_eq!(wy, 96.0);
}

#[test]
fn test_ui_renderer_with_hud() {
    let mut renderer = UIRenderer::new();

    // Add HUD elements
    renderer.add_element(shambala::systems::ui::UIElement {
        id: "hp_bar".to_string(),
        x: 20.0, y: 20.0, width: 200.0, height: 20.0,
        visible: true,
        element_type: shambala::systems::ui::UIElementType::ProgressBar { current: 75.0, max: 100.0 },
    });

    renderer.add_element(shambala::systems::ui::UIElement {
        id: "mp_bar".to_string(),
        x: 20.0, y: 45.0, width: 200.0, height: 20.0,
        visible: true,
        element_type: shambala::systems::ui::UIElementType::ProgressBar { current: 50.0, max: 100.0 },
    });

    assert_eq!(renderer.get_visible_elements().len(), 2);
    assert_eq!(renderer.get_progress_bars().len(), 2);
}

#[test]
fn test_text_rendering() {
    let renderer = TextRenderer::new("font_atlas", (16, 16), 16);

    let (w, h) = renderer.text_dimensions("Shambala", 2.0);
    assert_eq!(w, 16.0 * 8.0 * 2.0); // 8 chars * 16px * 2x scale
    assert_eq!(h, 16.0 * 2.0);

    let lines = renderer.wrap_text("Welcome to the world of Shambala", 160.0, 1.0);
    assert!(lines.len() >= 2); // Should wrap at least once
}

#[test]
fn test_engine_with_all_systems() {
    let mut engine = GameEngine::new();
    engine.initialize();

    // Verify all systems are initialized
    assert!(engine.title_screen.visible);
    assert!(!engine.chaos_gate.is_active);
    assert_eq!(engine.quest_manager.quests.len(), 1);
    assert_eq!(engine.sprite_batch.sprite_count(), 0);

    // Run a few update cycles
    for _ in 0..10 {
        engine.update(1.0 / 60.0);
    }

    assert_eq!(engine.time.frame_count, 10);
    assert!(engine.running);
}

#[test]
fn test_render_pipeline_module() {
    // Verify the render module compiles and basic types work
    let batch = SpriteBatch::new();
    assert_eq!(batch.sprite_count(), 0);

    let renderer = UIRenderer::new();
    assert!(renderer.get_visible_elements().is_empty());

    let text = TextRenderer::new("font", (16, 16), 16);
    let (w, _) = text.text_dimensions("Test", 1.0);
    assert_eq!(w, 64.0);
}
