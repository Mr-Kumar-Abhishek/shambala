use shambala::components::data_drain::DataDrain;
use shambala::components::enemy::EnemyType;
use shambala::components::party::Party;
use shambala::components::position::Position;
use shambala::core::types::Class;
use shambala::entities::enemy::EnemyEntity;
use shambala::entities::player::PlayerEntity;
use shambala::game::engine::GameEngine;
use shambala::game::event::{EventBus, GameEvent};
use shambala::game::scene::{Scene, SceneManager};
use shambala::resources::camera::Camera;
use shambala::systems::combat::CombatSystem;
use shambala::systems::data_drain::DataDrainSystem;
use shambala::systems::party::PartySystem;
use shambala::systems::physics::PhysicsSystem;

#[test]
fn test_full_combat_flow() {
    // Create player and enemy
    let (_, mut player_stats, _, _, _, _, _, _) = PlayerEntity::create("Kite", Class::TwinBlade);
    let (_, mut enemy_stats, _, _) = EnemyEntity::create(EnemyType::Goblin, 100.0, 100.0);

    let initial_hp = enemy_stats.hp;

    // Player attacks enemy
    let damage = CombatSystem::calculate_damage(&player_stats, &enemy_stats, None);
    enemy_stats.take_damage(damage);

    assert!(enemy_stats.hp < initial_hp);
    assert!(damage >= 1);

    // Enemy attacks player
    let enemy_damage = CombatSystem::calculate_damage(&enemy_stats, &player_stats, None);
    player_stats.take_damage(enemy_damage);

    assert!(player_stats.hp < 100);

    // Verify both are still alive
    assert!(player_stats.is_alive());
    assert!(enemy_stats.is_alive() || enemy_stats.hp == 0);
}

#[test]
fn test_data_drain_in_combat() {
    let mut dd = DataDrain::new();

    // Charge from combat
    DataDrainSystem::charge_from_combat(&mut dd, 50);
    assert_eq!(dd.charge_level, 25.0);

    // More combat
    DataDrainSystem::charge_from_combat(&mut dd, 100);
    assert_eq!(dd.charge_level, 75.0);

    // Finish charging
    DataDrainSystem::charge_from_combat(&mut dd, 50);
    assert!(dd.is_ready());

    // Execute drain
    let result = DataDrainSystem::execute_drain(&mut dd, 0.1);
    assert!(result.exp_bonus > 0);
}

#[test]
fn test_party_experience_distribution() {
    let mut party = Party::new(3);
    party.add_member("Kite").unwrap();
    party.add_member("BlackRose").unwrap();
    party.add_member("Mimiru").unwrap();

    let exp = PartySystem::distribute_experience(&party, 300);
    assert_eq!(exp.len(), 3);

    // Each member gets at least base share
    for e in &exp {
        assert!(*e >= 100);
    }
}

#[test]
fn test_game_engine_lifecycle() {
    let mut engine = GameEngine::new();
    assert!(engine.running);

    engine.initialize();
    engine.update(0.016);
    engine.update(0.016);
    engine.update(0.016);

    assert_eq!(engine.time.frame_count, 3);
    assert!(!engine.assets.textures.is_empty());
    assert!(!engine.assets.is_loaded("player_TwinBlade")); // Not loaded yet, just registered

    engine.shutdown();
    assert!(!engine.running);
}

#[test]
fn test_scene_transitions() {
    let mut manager = SceneManager::new();

    manager.register_scene(Scene {
        id: "title".to_string(),
        scene_type: shambala::game::scene::SceneType::Title,
        entities: vec!["title_bg".to_string()],
        is_loaded: false,
    });

    manager.register_scene(Scene {
        id: "mac_anu".to_string(),
        scene_type: shambala::game::scene::SceneType::RootTown,
        entities: vec!["player".to_string(), "npc_helba".to_string()],
        is_loaded: false,
    });

    assert!(manager.load_scene("title").is_ok());
    assert_eq!(manager.current_scene.as_ref().unwrap().id, "title");

    manager.unload_current();
    assert!(manager.current_scene.is_none());

    assert!(manager.load_scene("mac_anu").is_ok());
    assert_eq!(manager.current_scene.as_ref().unwrap().entities.len(), 2);
}

#[test]
fn test_event_system_integration() {
    let mut bus = EventBus::new(50);

    // Simulate a combat encounter
    bus.emit(GameEvent::EnemyDefeated {
        enemy_id: "goblin_1".to_string(),
        exp_reward: 15,
    });
    bus.emit(GameEvent::LevelUp { new_level: 2 });
    bus.emit(GameEvent::ItemObtained {
        item_id: "goblin_ear".to_string(),
        quantity: 1,
    });

    assert!(bus.has_event("EnemyDefeated"));
    assert!(bus.has_event("LevelUp"));

    let events = bus.drain();
    assert_eq!(events.len(), 3);
}

#[test]
fn test_camera_with_entities() {
    let mut camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
    let player_pos = Position::new(640.0, 360.0);

    // Camera follows player: centers on target
    camera.follow(player_pos.x, player_pos.y, 1.0);
    // After follow, camera.x = 640 - 1280/2 = 0, camera.y = 360 - 720/2 = 0
    assert_eq!(camera.x, 0.0);
    assert_eq!(camera.y, 0.0);

    // World-to-screen: player at (640,360) maps to center of screen
    let (screen_x, screen_y) = camera.world_to_screen(640.0, 360.0);
    assert!((screen_x - 640.0).abs() < f32::EPSILON);
    assert!((screen_y - 360.0).abs() < f32::EPSILON);

    // An entity at world (1280, 720) maps to screen edge
    let (sx, sy) = camera.world_to_screen(1280.0, 720.0);
    assert!((sx - 1280.0).abs() < f32::EPSILON);
    assert!((sy - 720.0).abs() < f32::EPSILON);
}

#[test]
fn test_timed_effects_integration() {
    use shambala::components::status::StatusEffects;
    use shambala::core::types::StatusEffect;

    let mut effects = StatusEffects::new();

    // Apply poison
    effects.add(StatusEffect::Poison, 10.0, 5);
    assert!(effects.has_effect(StatusEffect::Poison));

    // Simulate time passing
    effects.update(5.0);
    assert!(effects.has_effect(StatusEffect::Poison));

    effects.update(5.0);
    assert!(!effects.has_effect(StatusEffect::Poison));
}

#[test]
fn test_movement_and_collision() {
    let mut player_pos = Position::new(0.0, 0.0);
    let enemy_pos = Position::new(100.0, 0.0);

    // Player moves toward enemy
    let vel = shambala::components::position::Velocity::new(50.0, 0.0);
    PhysicsSystem::update_position(&mut player_pos, &vel, 1.0);
    assert_eq!(player_pos.x, 50.0);

    // Check distance
    let dist = PhysicsSystem::distance_between(&player_pos, &enemy_pos);
    assert!((dist - 50.0).abs() < f32::EPSILON);

    // No collision yet
    assert!(!PhysicsSystem::check_collision(
        &player_pos,
        32.0,
        32.0,
        &enemy_pos,
        32.0,
        32.0
    ));

    // Move closer
    PhysicsSystem::update_position(&mut player_pos, &vel, 1.0);
    assert!(PhysicsSystem::check_collision(
        &player_pos,
        32.0,
        32.0,
        &enemy_pos,
        32.0,
        32.0
    ));
}

#[test]
fn test_area_exploration() {
    use shambala::entities::area::AreaEntity;

    let area = AreaEntity::create_field(&["forest".to_string(), "lake".to_string()], 42);

    assert_eq!(area.area_type, shambala::entities::area::AreaType::Field);
    assert_eq!(area.keywords.len(), 2);
    assert!(area.generated_area.is_some());

    let generated = area.generated_area.unwrap();
    assert_eq!(generated.width, 30);
    assert_eq!(generated.height, 30);
    assert_eq!(generated.entrance, (1, 1));
    assert_eq!(generated.exit, (28, 28));
}
