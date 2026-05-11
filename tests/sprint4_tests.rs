use shambala::systems::combat_integration::{CombatEncounter, CombatIntegrationSystem};
use shambala::systems::exploration::ExplorationSystem;
use shambala::game::dialogue::NPCDialogue;
use shambala::game::options_menu::OptionsMenu;
use shambala::game::progression::{SkillTree, ProgressionSystem};
use shambala::components::stats::Stats;
use shambala::components::position::{Position, Velocity};
use shambala::resources::input_state::InputStateResource;
use shambala::resources::camera::Camera;
use shambala::systems::input::{InputAction, InputState};
use shambala::systems::effects::EffectsManager;
use shambala::systems::animation::AnimationManager;
use shambala::systems::audio_playback::AudioPlaybackSystem;
use shambala::game::event::EventBus;
use shambala::core::types::Class;
use shambala::entities::area::AreaEntity;
use shambala::components::enemy::{Enemy, EnemyType};

// ===========================================================================
// Sprint 4 Integration Tests
// ===========================================================================

#[test]
fn test_full_combat_encounter() {
    let player = Stats::new(Class::HeavyBlade);
    let mut enemy = Stats::new(Class::Wavemaster);
    enemy.hp = 1; // Guaranteed one-hit kill
    
    let mut encounter = CombatEncounter::new(player, enemy);
    assert!(encounter.is_active);

    // Player attacks until enemy is defeated
    while encounter.is_active && encounter.is_player_turn {
        let result = encounter.player_attack(None);
        assert!(result.is_valid());
        assert!(result.damage > 0);
    }

    assert!(encounter.is_over());
    assert!(encounter.player_won());
    assert!(!encounter.get_log().is_empty());
}

#[test]
fn test_combat_with_visualization() {
    let mut effects = EffectsManager::new();
    let mut animations = AnimationManager::new();
    let mut audio = AudioPlaybackSystem::new();
    let mut event_bus = EventBus::new(100);

    let result = shambala::systems::combat_integration::CombatActionResult {
        damage: 50,
        is_critical: true,
        is_player_turn: true,
        target_defeated: false,
        source: "player".to_string(),
        skill_name: None,
    };

    CombatIntegrationSystem::process_combat_result(
        &result, &mut effects, &mut animations, &mut audio, &mut event_bus, 100.0, 100.0,
    );

    assert_eq!(effects.damage_count(), 1);
    assert_eq!(effects.screen_shakes.len(), 1);
    assert_eq!(animations.animations.len(), 1);
}

#[test]
fn test_player_movement_in_area() {
    let area = AreaEntity::create_root_town();
    let mut pos = Position::new(400.0, 300.0);
    let mut vel = Velocity::new(0.0, 0.0);
    let mut input = InputStateResource::new();

    // Move right
    input.set_action(InputAction::MoveRight, InputState::Held);
    ExplorationSystem::handle_movement(&mut pos, &mut vel, &input, 1.0);
    assert!(pos.x > 400.0);

    // Check boundaries
    ExplorationSystem::check_area_boundaries(&mut pos, &area);
    assert!(pos.x >= 32.0);

    // Camera follows
    let mut camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
    ExplorationSystem::update_camera(&mut camera, &pos, 1.0);
    assert!(camera.x != 0.0, "Camera should follow the player position");
}

#[test]
fn test_npc_dialogue_flow() {
    let mut dialogue = NPCDialogue::new("npc_helba", "Helba");

    // Start dialogue
    dialogue.start_dialogue(vec![
        shambala::game::quest::DialogueNode {
            id: "start".to_string(),
            speaker: "Helba".to_string(),
            text: "Welcome to Shambala.".to_string(),
            choices: vec![
                shambala::game::quest::DialogueChoice {
                    text: "Tell me more.".to_string(),
                    next_node_id: "info".to_string(),
                    required_quest_status: None,
                },
            ],
            is_end: false,
            event_to_trigger: None,
        },
        shambala::game::quest::DialogueNode {
            id: "info".to_string(),
            speaker: "Helba".to_string(),
            text: "This world is a mystery.".to_string(),
            choices: vec![],
            is_end: true,
            event_to_trigger: None,
        },
    ]);

    assert!(dialogue.is_active);

    // Fast-forward typing
    dialogue.typing_progress = 999.0;
    dialogue.text_fully_displayed = true;

    // Navigate to choice
    let mut input = InputStateResource::new();
    input.set_action(InputAction::Confirm, InputState::Pressed);
    let mut quest_manager = shambala::game::quest::QuestManager::new();

    // Should advance to next node
    dialogue.handle_input(&input, &mut quest_manager);
    assert_eq!(dialogue.current_node, "info");
}

#[test]
fn test_options_menu_navigation() {
    let mut menu = OptionsMenu::new();
    menu.open();
    assert!(menu.visible);

    // Navigate tabs
    let mut input = InputStateResource::new();
    input.set_action(InputAction::MoveRight, InputState::Pressed);
    let mut audio = AudioPlaybackSystem::new();
    menu.update(&input, &mut audio);
    assert_eq!(menu.selected_tab, 1);

    // Close with cancel
    let mut cancel_input = InputStateResource::new();
    cancel_input.set_action(InputAction::Cancel, InputState::Pressed);
    menu.update(&cancel_input, &mut audio);
    assert!(!menu.visible);
}

#[test]
fn test_skill_tree_progression() {
    let mut tree = SkillTree::new(Class::Wavemaster);
    tree.available_points = 3;

    // Unlock skills in order
    assert!(tree.unlock_skill("heal").is_ok());
    assert!(tree.unlock_skill("thunder_bolt").is_ok());
    assert!(tree.unlock_skill("meteor").is_ok());

    assert_eq!(tree.available_points, 0);
    assert_eq!(tree.get_unlocked_skills().len(), 4); // 1 default + 3 unlocked
    assert!((tree.progress_percentage() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_level_up_flow() {
    let mut stats = Stats::new(Class::TwinBlade);
    let mut tree = SkillTree::new(Class::TwinBlade);
    let mut event_bus = EventBus::new(100);

    // Add enough exp to level up
    stats.add_experience(100);
    assert_eq!(stats.level, 2);

    // Apply level up
    let screen = ProgressionSystem::apply_level_up(&mut stats, &mut tree, &mut event_bus);
    assert_eq!(screen.new_level, 2);
    assert_eq!(tree.available_points, 1);

    // Verify stats increased
    assert_eq!(stats.max_hp, 110);
}

#[test]
fn test_exploration_enemy_proximity() {
    let player_pos = Position::new(0.0, 0.0);
    let enemy = Enemy::new(EnemyType::Goblin);
    let enemy_pos = Position::new(50.0, 0.0);

    let nearby = ExplorationSystem::check_enemy_proximity(
        &player_pos,
        &[(enemy, enemy_pos)],
        100.0,
    );
    assert_eq!(nearby.len(), 1);
}

#[test]
fn test_full_game_state_with_all_sprint4_systems() {
    let mut engine = shambala::game::engine::GameEngine::new();
    engine.initialize();

    // Verify all Sprint 4 systems are accessible
    assert!(engine.title_screen.visible);
    assert!(!engine.chaos_gate.is_active);
    assert_eq!(engine.quest_manager.quests.len(), 1);

    // Run multiple update cycles
    for _ in 0..120 {
        engine.update(1.0 / 60.0);
    }

    assert_eq!(engine.time.frame_count, 120);
    assert!(engine.running);
}