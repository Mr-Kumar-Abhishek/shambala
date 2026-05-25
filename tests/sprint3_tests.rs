use shambala::core::types::Class;
use shambala::game::character_select::CharacterSelectScreen;
use shambala::game::event::GameEvent;
use shambala::game::network::NetworkSimulator;
use shambala::game::save_load::{SaveData, SaveManager};
use shambala::render::atlas::TextureAtlas;
use shambala::systems::animation::{Animation, AnimationManager};
use shambala::systems::audio_playback::{AudioPlaybackSystem, AudioTrackType};
use shambala::systems::effects::EffectsManager;

// ---------------------------------------------------------------------------
// Helper: unique test directory for save/load tests to avoid file-system
// contention when tests run in parallel.
// ---------------------------------------------------------------------------
use std::sync::atomic::{AtomicUsize, Ordering};
static SAVE_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn test_save_manager() -> SaveManager {
    let id = SAVE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let save_dir = std::path::PathBuf::from(format!("test_saves_s3_{}", id));
    SaveManager {
        save_dir,
        current_save: None,
        save_slots: vec![None, None, None],
    }
}

fn cleanup_save(manager: &SaveManager) {
    let _ = std::fs::remove_dir_all(&manager.save_dir);
}

// ===========================================================================
// Sprint 3 Integration Tests
// ===========================================================================

#[test]
fn test_character_creation_flow() {
    let mut screen = CharacterSelectScreen::new();
    assert_eq!(screen.characters.len(), 4);

    // Enter name
    screen.player_name = "Kite".to_string();
    screen.name_input_active = false;

    // Select class
    screen.selected_index = 2; // LongArm
    assert_eq!(screen.get_selected_class(), Class::LongArm);

    // Confirm
    let mut input = shambala::resources::input_state::InputStateResource::new();
    input.set_action(
        shambala::systems::input::InputAction::Confirm,
        shambala::systems::input::InputState::Pressed,
    );
    let result = screen.update(0.016, &input);
    assert!(result.is_some());
    let (class, name) = result.unwrap();
    assert_eq!(class, Class::LongArm);
    assert_eq!(name, "Kite");
}

#[test]
fn test_save_load_roundtrip() {
    let mut manager = test_save_manager();
    let data = SaveData::new("BlackRose", Class::HeavyBlade);

    // Save
    assert!(manager.save_to_slot(1, &data).is_ok());
    assert!(manager.slot_has_data(1));

    // Load
    let loaded = manager.load_from_slot(1).unwrap();
    assert_eq!(loaded.player_name, "BlackRose");
    assert_eq!(loaded.player_class, Class::HeavyBlade);
    assert_eq!(loaded.player_level, 1);

    // Cleanup
    let _ = manager.delete_slot(1);
    assert!(!manager.slot_has_data(1));
    cleanup_save(&manager);
}

#[test]
fn test_network_connection_flow() {
    let mut net = NetworkSimulator::new();
    assert!(!net.is_connected());

    net.connect();
    assert!(net.is_connecting());

    // Simulate connection time
    net.update(2.5);
    assert!(net.is_connected());

    // Should have received welcome event
    let events = net.receive_server_events();
    assert_eq!(events.len(), 1);

    // Send a message
    net.send_event(GameEvent::PlayerDamaged {
        amount: 15,
        source: "Goblin".to_string(),
    });
    assert_eq!(net.message_queue_size(), 1);

    // Disconnect
    net.disconnect("Test complete");
    net.update(0.1);
    assert!(!net.is_connected());
}

#[test]
fn test_animation_playback() {
    let mut anim = Animation::new("test_attack", 0.1, false);
    anim.add_frame("frame_1");
    anim.add_frame("frame_2");
    anim.add_frame("frame_3");

    assert_eq!(anim.frames.len(), 3);
    assert!(anim.playing);

    // Advance through frames
    anim.update(0.15);
    assert_eq!(anim.current_frame, 1);

    anim.update(0.15);
    assert_eq!(anim.current_frame, 2);

    anim.update(0.15);
    assert!(anim.finished);
}

#[test]
fn test_effects_lifecycle() {
    let mut effects = EffectsManager::new();

    // Spawn effects
    effects.spawn_damage_number(50, 100.0, 100.0, false, false);
    effects.spawn_damage_number(999, 200.0, 100.0, true, false); // Critical
    effects.spawn_damage_number(30, 150.0, 150.0, false, true); // Heal
    effects.shake_screen(10.0, 0.5);

    assert_eq!(effects.damage_count(), 3);
    assert_eq!(effects.screen_shakes.len(), 1);

    // Update past expiry (first call updates elapsed time,
    // second call removes expired entries)
    effects.update(2.0);
    effects.update(0.0);
    assert!(effects.damage_numbers.is_empty());
    assert!(effects.screen_shakes.is_empty());
}

#[test]
fn test_texture_atlas_integration() {
    let atlas = TextureAtlas::create_sprite_atlas();
    assert_eq!(atlas.region_count(), 17);

    // Verify all expected regions exist
    assert!(atlas.contains("player_TwinBlade"));
    assert!(atlas.contains("player_Wavemaster"));
    assert!(atlas.contains("enemy_Boss"));
    assert!(atlas.contains("tile_treasure"));

    // Verify UV coordinates
    let uv = atlas.get_uv("player_TwinBlade").unwrap();
    assert!(uv.0 >= 0.0 && uv.0 <= 1.0);
    assert!(uv.1 >= 0.0 && uv.1 <= 1.0);
    assert!(uv.2 > 0.0 && uv.2 <= 1.0);
    assert!(uv.3 > 0.0 && uv.3 <= 1.0);
}

#[test]
fn test_audio_system_integration() {
    let mut audio = AudioPlaybackSystem::new();

    // Register tracks
    audio.register_track(
        "bgm_field",
        "assets/audio/bgm/field.ogg",
        AudioTrackType::Bgm,
        true,
    );
    audio.register_track(
        "sfx_attack",
        "assets/audio/sfx/attack.wav",
        AudioTrackType::Sfx,
        false,
    );
    audio.register_track(
        "ambient_forest",
        "assets/audio/ambient/forest.ogg",
        AudioTrackType::Ambient,
        true,
    );

    assert_eq!(audio.track_count(), 3);

    // Play BGM
    audio.play_bgm("bgm_field");
    assert!(audio.is_playing());

    // Volume control
    audio.set_master_volume(0.8);
    audio.set_bgm_volume(0.5);
    let vol = audio.get_effective_volume(AudioTrackType::Bgm);
    assert!((vol - 0.4).abs() < f32::EPSILON); // 0.8 * 0.5

    // Mute
    audio.toggle_mute();
    assert_eq!(audio.get_effective_volume(AudioTrackType::Sfx), 0.0);
}

#[test]
fn test_animation_manager_factories() {
    let mut manager = AnimationManager::new();

    // Create animations using factory methods
    manager.add_animation(AnimationManager::create_attack_animation("TwinBlade"));
    manager.add_animation(AnimationManager::create_hit_animation());
    manager.add_animation(AnimationManager::create_spell_animation("Fire"));
    manager.add_animation(AnimationManager::create_idle_animation("Wavemaster"));

    assert_eq!(manager.animations.len(), 4);

    // Verify animation structures
    let attack = manager.get("attack_TwinBlade").unwrap();
    assert_eq!(attack.frames.len(), 4);
    assert!(!attack.loop_);

    let idle = manager.get("idle_Wavemaster").unwrap();
    assert!(idle.loop_);
    assert_eq!(idle.frames.len(), 2);
}

#[test]
fn test_full_game_initialization() {
    let mut engine = shambala::game::engine::GameEngine::new();
    engine.initialize();

    // Verify all Sprint 3 systems are initialized
    assert_eq!(engine.quest_manager.quests.len(), 1);
    assert!(engine.title_screen.visible);
    assert!(!engine.chaos_gate.is_active);

    // Run update cycles
    for _ in 0..60 {
        engine.update(1.0 / 60.0);
    }

    assert_eq!(engine.time.frame_count, 60);
    assert!(engine.running);
}
