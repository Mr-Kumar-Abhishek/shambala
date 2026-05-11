# Shambala — Test-Driven Development Guide

> **Version:** 1.0
> **Status:** Final
> **Last Updated:** 2026-05-11
> **Engine:** Custom (Rust, ECS, 2D Top-Down)
> **Testing Framework:** `cargo test` + `proptest` + `criterion`

---

## Table of Contents

1. [TDD Philosophy](#1-tdd-philosophy)
2. [Testing Framework Setup](#2-testing-framework-setup)
3. [Unit Testing Strategy](#3-unit-testing-strategy)
4. [Integration Testing Strategy](#4-integration-testing-strategy)
5. [Property-Based Testing](#5-property-based-testing)
6. [Test Organization](#6-test-organization)
7. [Mocking & Test Fixtures](#7-mocking--test-fixtures)
8. [TDD Workflow for Game Dev](#8-tdd-workflow-for-game-dev)
9. [Example TDD Sessions](#9-example-tdd-sessions)
10. [Coverage Goals](#10-coverage-goals)
11. [CI/CD Integration](#11-cicd-integration)
12. [Conventional Commits](#12-conventional-commits)

---

## 1. TDD Philosophy

### 1.1 What is TDD?

Test-Driven Development is a software development discipline where you write a **failing test before** you write the production code. The cycle is:

```
┌─────────────────────────────────────────────────────┐
│                    TDD CYCLE                         │
│                                                      │
│   ┌──────────┐     ┌──────────┐     ┌──────────┐   │
│   │   RED    │────▶│  GREEN   │────▶│ REFACTOR │   │
│   │          │     │          │     │          │   │
│   │ Write a  │     │ Write the│     │ Clean up │   │
│   │ failing  │     │ minimum  │     │ code     │   │
│   │ test     │     │ code to  │     │ while    │   │
│   │          │     │ pass     │     │ tests    │   │
│   │          │     │          │     │ stay     │   │
│   │          │     │          │     │ green    │   │
│   └──────────┘     └──────────┘     └──────────┘   │
│        │                │                │          │
│        └────────────────┴────────────────┘          │
│                        │                            │
│                        ▼                            │
│               Next test / feature                    │
└─────────────────────────────────────────────────────┘
```

### 1.2 Why TDD for Game Development?

Game development has unique challenges that TDD addresses directly:

| Challenge | How TDD Helps |
|---|---|
| **Complex systems interactions** (combat + physics + AI + data drain) | Tests validate each system in isolation before integration |
| **Procedural content** (area generation, loot tables) | Property-based tests validate invariants across all possible inputs |
| **Balance tuning** (damage formulas, cooldowns) | Tests encode expected ranges; benchmarks detect regressions |
| **State machine complexity** (game phases, UI states) | Tests cover every transition and boundary |
| **ECS architecture** (components are pure data) | Components are trivially testable — no mocking needed |
| **Multiplayer simulation** (local server emulator) | Integration tests validate client-server event flow |

### 1.3 The Red-Green-Refactor Cycle Explained

#### 🔴 RED — Write a Failing Test

Before writing any implementation code, write a test that expresses the desired behavior:

```rust
// This test will FAIL because Health::new() doesn't exist yet
#[test]
fn test_health_new_with_max_hp() {
    let health = Health::new(100, 50);
    assert_eq!(health.max_hp, 100);
    assert_eq!(health.current_hp, 100);  // Starts at max
    assert_eq!(health.current_sp, 50);
    assert_eq!(health.max_sp, 50);
}
```

**Rules of RED:**
- The test must fail for the *right reason* (missing feature, not a compile error)
- The test should be as simple as possible
- Test one behavior per test function
- Use descriptive names: `test_<system>_<behavior>_<condition>`

#### 🟢 GREEN — Write the Minimum Code to Pass

Write the simplest possible implementation to make the test pass:

```rust
pub struct Health {
    pub current_hp: u32,
    pub max_hp: u32,
    pub current_sp: u32,
    pub max_sp: u32,
    pub hp_regen: f32,
    pub sp_regen: f32,
    pub invulnerable: bool,
    pub invuln_timer: f32,
}

impl Health {
    pub fn new(max_hp: u32, max_sp: u32) -> Self {
        Self {
            current_hp: max_hp,
            max_hp,
            current_sp: max_sp,
            max_sp,
            hp_regen: 0.0,
            sp_regen: 0.0,
            invulnerable: false,
            invuln_timer: 0.0,
        }
    }
}
```

**Rules of GREEN:**
- Write the *minimum* code to pass — no gold-plating
- It's okay to hardcode values temporarily
- Don't optimize yet — that comes in REFACTOR
- If the test fails, diagnose why and fix the implementation

#### 🔵 REFACTOR — Clean Up While Staying Green

Improve the code quality without changing behavior:

```rust
impl Health {
    /// Creates a new Health component with full HP and SP.
    pub fn new(max_hp: u32, max_sp: u32) -> Self {
        Self {
            current_hp: max_hp,
            max_hp,
            current_sp: max_sp,
            max_sp,
            hp_regen: DEFAULT_HP_REGEN,
            sp_regen: DEFAULT_SP_REGEN,
            invulnerable: false,
            invuln_timer: 0.0,
        }
    }
}
```

**Rules of REFACTOR:**
- Only change code structure, not behavior
- Run the full test suite after each refactor step
- Extract duplicated logic into helper functions
- Add documentation comments
- Improve naming
- **Never refactor without a green test suite**

### 1.4 TDD Mantras for Shambala

> *"Write the test that proves the feature doesn't exist yet."*

> *"If it's not tested, it's broken — you just haven't noticed yet."*

> *"A test suite is not a safety net — it's the specification."*

> *"Refactoring without tests is just moving broken code around."*

---

## 2. Testing Framework Setup

### 2.1 Dependencies

Add these to your [`Cargo.toml`](Cargo.toml) `[dev-dependencies]`:

```toml
[dev-dependencies]
# ── Testing ──
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1.5"                      # Property-based testing
test-case = "3.3"                     # Test case parameterization
mockall = "0.13"                      # Mock objects for testing
pretty_assertions = "1.4"            # Better assertion output
test-log = "0.2"                      # Log output in tests
csv = "1.3"                           # Export benchmark data

# ── Benchmark-specific ──
fastrand = "2.1"                      # Fast RNG for benchmarks
```

### 2.2 `cargo test` Configuration

Create a [`.cargo/config.toml`](.cargo/config.toml) to set default test options:

```toml
[alias]
tdd = "test --lib"                    # Fast: unit tests only (runs in < 1s)
t-all = "test"                        # Full: unit + integration
t-quick = "test --lib -- --include-ignored"  # Include #[ignore] tests
t-doc = "test --doc"                  # Doc tests
coverage = "llvm-cov --lcov --output-path lcov.info"  # Code coverage

[test]
# Default test runner arguments
# Run tests in parallel, show output on failure
```

### 2.3 Running Tests

| Command | What It Runs | When to Use |
|---|---|---|
| `cargo tdd` | All unit tests (lib only) | During development, every few keystrokes |
| `cargo t-all` | Unit + integration tests | Before commit |
| `cargo test -- --ignored` | Slow/integration-only tests | Before merging a PR |
| `cargo test test_combat` | Single test by name | Debugging a specific test |
| `cargo test combat` | All tests containing "combat" | Working on a feature area |
| `cargo test --test integration_combat` | Single integration test file | Full scenario testing |
| `cargo coverage` | Generate coverage report | Before CI submission |

### 2.4 Benchmark Setup (`criterion`)

Create [`benches/bench_main.rs`](benches/bench_main.rs):

```rust
use criterion::{criterion_group, criterion_main};

mod render_benchmarks;
mod physics_benchmarks;
mod ecs_benchmarks;

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5));
    targets = render_benchmarks::sprite_batching_benchmark,
              physics_benchmarks::collision_query_benchmark,
              ecs_benchmarks::query_iteration_benchmark,
);
criterion_main!(benches);
```

Run benchmarks with:

```bash
cargo bench
```

Compare against a baseline to catch regressions:

```bash
cargo bench -- --save-baseline current
# ... make changes ...
cargo bench -- --baseline current
```

### 2.5 Feature Gates for Headless Testing

Use Cargo features to enable/disable test requirements:

```rust
// In tests/integration/combat_tests.rs
#[test]
#[cfg_attr(feature = "headless", ignore)]
fn test_render_combat_ui() {
    // This test requires a GPU, so skip in headless CI
}
```

The [`Cargo.toml`](Cargo.toml) defines:

```toml
[features]
default = ["audio", "physics"]
headless = []    # No graphics or audio
```

CI runs with:

```bash
cargo test --features headless
```

---

## 3. Unit Testing Strategy

### 3.1 Testing Components (Pure Data)

Components are plain data structs — the easiest thing to test. Every component should have tests for:

- **Construction** — `new()` with valid and edge-case values
- **Default implementations** — `Default::default()` produces sane values
- **Serialization round-trips** — Serde serialize/deserialize preserves data
- **Derived calculations** — Any `impl` methods on the component

```rust
// src/components/health.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new_sets_current_to_max() {
        let health = Health::new(100, 50);
        assert_eq!(health.current_hp, 100);
        assert_eq!(health.max_hp, 100);
        assert_eq!(health.current_sp, 50);
        assert_eq!(health.max_sp, 50);
    }

    #[test]
    fn test_take_damage_reduces_hp() {
        let mut health = Health::new(100, 50);
        health.take_damage(30);
        assert_eq!(health.current_hp, 70);
    }

    #[test]
    fn test_take_damage_does_not_go_below_zero() {
        let mut health = Health::new(100, 50);
        health.take_damage(999);
        assert_eq!(health.current_hp, 0);
    }

    #[test]
    fn test_heal_increases_hp() {
        let mut health = Health::new(100, 50);
        health.take_damage(50);
        health.heal(30);
        assert_eq!(health.current_hp, 80);
    }

    #[test]
    fn test_heal_does_not_exceed_max() {
        let mut health = Health::new(100, 50);
        health.take_damage(10);
        health.heal(999);
        assert_eq!(health.current_hp, 100);
    }

    #[test]
    fn test_is_alive_positive_hp() {
        let health = Health::new(100, 50);
        assert!(health.is_alive());
    }

    #[test]
    fn test_is_dead_zero_hp() {
        let mut health = Health::new(100, 50);
        health.take_damage(100);
        assert!(!health.is_alive());
    }

    #[test]
    fn test_invulnerability_blocks_damage() {
        let mut health = Health::new(100, 50);
        health.invulnerable = true;
        health.take_damage(50);
        assert_eq!(health.current_hp, 100);
    }

    #[test]
    fn test_regen_tick() {
        let mut health = Health {
            current_hp: 50,
            max_hp: 100,
            hp_regen: 10.0,
            ..Health::new(100, 50)
        };
        health.regen_tick(1.0); // 1 second of regen
        assert_eq!(health.current_hp, 60);
    }

    #[test]
    fn test_serialization_round_trip() {
        let health = Health::new(100, 50);
        let json = serde_json::to_string(&health).unwrap();
        let deserialized: Health = serde_json::from_str(&json).unwrap();
        assert_eq!(health.current_hp, deserialized.current_hp);
        assert_eq!(health.max_hp, deserialized.max_hp);
        assert_eq!(health.current_sp, deserialized.current_sp);
        assert_eq!(health.max_sp, deserialized.max_sp);
    }
}
```

### 3.2 Testing Systems (Logic in Isolation)

Systems operate on component data. Test them by constructing inputs and calling the system function directly.

Use the pattern: **Arrange → Act → Assert**

```rust
// src/systems/combat.rs
#[cfg(test)]
mod tests {
    use super::*;

    // ── Helper: Create a default test context ──
    fn create_combat_context() -> (Stats, Health, Combat) {
        let stats = Stats {
            level: 1,
            strength: 10,
            vitality: 8,
            attack: 25,
            defense: 15,
            ..Default::default()
        };
        let health = Health::new(100, 50);
        let combat = Combat {
            base_damage: 10,
            attack_range: 48.0,
            attack_speed: 1.5,
            ..Default::default()
        };
        (stats, health, combat)
    }

    // ── Damage Calculation Tests ──

    #[test]
    fn test_calculate_damage_basic() {
        let (attacker, _, _) = create_combat_context();
        let (_, defender_health, _) = create_combat_context();
        let defender_stats = Stats {
            defense: 10,
            ..Default::default()
        };

        let damage = calculate_damage(
            &attacker, &defender_stats, 1.0 // skill multiplier
        );

        assert!(damage > 0, "Damage should be positive");
        assert!(damage < attacker.attack * 2, "Damage should be bounded");
    }

    #[test]
    fn test_calculate_damage_critical_hit() {
        let attacker = Stats {
            crit_rate: 1.0,  // Always crit
            crit_damage: 2.0, // Double damage
            attack: 50,
            ..Default::default()
        };
        let defender = Stats {
            defense: 10,
            ..Default::default()
        };

        let (damage, was_crit) = calculate_damage_with_crit(&attacker, &defender, 1.0);
        assert!(was_crit, "Should be a critical hit");
        assert!(damage > 20, "Critical damage should be significant");
    }

    #[test]
    fn test_calculate_damage_zero_defense() {
        let attacker = Stats {
            attack: 50,
            ..Default::default()
        };
        let defender = Stats {
            defense: 0,
            ..Default::default()
        };

        let damage = calculate_damage(&attacker, &defender, 1.0);
        assert!(damage >= attacker.attack, "Zero defense should take full damage");
    }

    #[test]
    fn test_calculate_damage_very_high_defense() {
        let attacker = Stats {
            attack: 10,
            ..Default::default()
        };
        let defender = Stats {
            defense: 999,
            ..Default::default()
        };

        let damage = calculate_damage(&attacker, &defender, 1.0);
        assert_eq!(damage, 1, "Minimum damage should be 1");
    }

    // ── Damage Application Tests ──

    #[test]
    fn test_apply_damage_to_entity() {
        let mut health = Health::new(100, 50);
        apply_damage(&mut health, 30, DamageType::Physical);

        assert_eq!(health.current_hp, 70);
    }

    #[test]
    fn test_apply_damage_triggers_invulnerability() {
        let mut health = Health::new(100, 50);
        // Some systems grant brief invulnerability after being hit
        apply_damage(&mut health, 30, DamageType::Physical);
        assert!(health.invulnerable);
        assert!(health.invuln_timer > 0.0);
    }

    // ── Healing Tests ──

    #[test]
    fn test_apply_healing() {
        let mut health = Health {
            current_hp: 30,
            ..Health::new(100, 50)
        };
        apply_healing(&mut health, 40);
        assert_eq!(health.current_hp, 70);
    }

    #[test]
    fn test_apply_healing_does_not_overflow() {
        let mut health = Health {
            current_hp: 90,
            ..Health::new(100, 50)
        };
        apply_healing(&mut health, 40);
        assert_eq!(health.current_hp, 100);
    }

    // ── Death Tests ──

    #[test]
    fn test_entity_dies_when_hp_reaches_zero() {
        let mut health = Health::new(100, 50);
        let is_dead = try_kill(&mut health, 100);
        assert!(is_dead);
        assert_eq!(health.current_hp, 0);
    }

    #[test]
    fn test_overkill_does_not_go_negative() {
        let mut health = Health::new(100, 50);
        try_kill(&mut health, 999);
        assert_eq!(health.current_hp, 0);
    }
}
```

### 3.3 Testing Resources (Global State)

Resources are singletons. Test their methods and state transitions:

```rust
// src/resources/game_state.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_game_state_is_boot() {
        let state = GameState::new();
        assert_eq!(state.current, GamePhase::Boot);
    }

    #[test]
    fn test_transition_to_title() {
        let mut state = GameState::new();
        state.transition(GamePhase::Title);
        assert_eq!(state.current, GamePhase::Title);
        assert_eq!(state.previous, GamePhase::Boot);
    }

    #[test]
    fn test_transition_from_exploring_to_combat() {
        let mut state = GameState::new();
        state.transition(GamePhase::Exploring);
        state.transition(GamePhase::Combat);
        assert_eq!(state.current, GamePhase::Combat);
        assert_eq!(state.previous, GamePhase::Exploring);
    }

    #[test]
    fn test_invalid_transition_returns_error() {
        let mut state = GameState::new();
        // Cannot go from Boot directly to Combat
        let result = state.try_transition(GamePhase::Combat);
        assert!(result.is_err());
    }

    #[test]
    fn test_pause_toggle() {
        let mut state = GameState::new();
        state.transition(GamePhase::Exploring);
        assert!(!state.paused);
        state.toggle_pause();
        assert!(state.paused);
        state.toggle_pause();
        assert!(!state.paused);
    }
}
```

### 3.4 Testing `procgen` Modules

Procedural generation functions should be deterministic (same seed → same output):

```rust
// src/procgen/terrain.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_terrain_is_deterministic() {
        let map_a = generate_terrain(42, TerrainType::Forest, 100, 100);
        let map_b = generate_terrain(42, TerrainType::Forest, 100, 100);
        assert_eq!(map_a.tiles, map_b.tiles, "Same seed must produce identical terrain");
    }

    #[test]
    fn test_different_seeds_produce_different_terrains() {
        let map_a = generate_terrain(42, TerrainType::Forest, 100, 100);
        let map_b = generate_terrain(99, TerrainType::Forest, 100, 100);
        assert_ne!(map_a.tiles, map_b.tiles, "Different seeds must produce different terrain");
    }

    #[test]
    fn test_terrain_dimensions_match_request() {
        let map = generate_terrain(42, TerrainType::Forest, 50, 75);
        assert_eq!(map.map_width, 50);
        assert_eq!(map.map_height, 75);
        assert_eq!(map.tiles.len(), 50 * 75);
    }

    #[test]
    fn test_all_tiles_have_valid_indices() {
        let map = generate_terrain(42, TerrainType::Forest, 100, 100);
        let max_tile = map.tiles.iter().max().unwrap();
        assert!(*max_tile < map.tileset_columns * map.tileset_rows,
                "Tile index must be within tileset bounds");
    }

    #[test]
    fn test_edge_tiles_are_walls() {
        let map = generate_terrain(42, TerrainType::Forest, 100, 100);
        // Top edge
        for x in 0..100 {
            assert!(map.collision_tiles[x], "Top edge tiles should be walls");
        }
        // Bottom edge
        for x in 0..100 {
            let idx = 99 * 100 + x;
            assert!(map.collision_tiles[idx], "Bottom edge tiles should be walls");
        }
    }
}
```

---

## 4. Integration Testing Strategy

### 4.1 System Interaction Tests

Integration tests validate that multiple systems work together correctly. They use a real (or minimal) ECS world.

```rust
// tests/integration/combat_tests.rs
use shambala::*;

/// Helper: Create a test app with a minimal ECS world
fn setup_combat_scenario() -> (TestApp, Entity, Entity) {
    let mut app = TestApp::new();
    let player = app.spawn_player(
        ClassType::TwinBlade,
        Position::new(0.0, 0.0),
        Stats::new(10, 8, 5, 12),
    );
    let enemy = app.spawn_enemy(
        EnemyCategory::Goblin,
        1,
        Position::new(100.0, 0.0),
    );
    (app, player, enemy)
}

#[test]
fn test_player_moves_toward_enemy_and_attacks() {
    let (mut app, player, enemy) = setup_combat_scenario();

    // Player moves right toward enemy
    app.simulate_input(vec![GameAction::MoveRight]);
    app.update(2.0); // Run 2 seconds of game time

    // Verify player is closer to enemy
    let player_pos = app.world().get::<Position>(player).unwrap();
    assert!(player_pos.x > 150.0, "Player should have moved right");

    // Player attacks
    app.simulate_input(vec![GameAction::BasicAttack]);
    app.update(0.5);

    // Enemy should have taken damage
    let enemy_health = app.world().get::<Health>(enemy).unwrap();
    assert!(
        enemy_health.current_hp < enemy_health.max_hp,
        "Enemy should have taken damage"
    );
}

#[test]
fn test_attacking_out_of_range_does_no_damage() {
    let (mut app, player, enemy) = setup_combat_scenario();

    // Enemy is 500px away — too far for melee
    app.world()
        .get_mut::<Position>(enemy)
        .unwrap()
        .set(500.0, 0.0);

    app.simulate_input(vec![GameAction::BasicAttack]);
    app.update(0.5);

    let enemy_health = app.world().get::<Health>(enemy).unwrap();
    assert_eq!(
        enemy_health.current_hp,
        enemy_health.max_hp,
        "Out-of-range attack should not damage"
    );
}

#[test]
fn test_heal_skill_restores_hp() {
    let (mut app, player, _enemy) = setup_combat_scenario();

    // Damage the player first
    let mut health = app.world().get_mut::<Health>(player).unwrap();
    health.take_damage(50);
    assert_eq!(health.current_hp, 50);

    // Drop health tracking — borrow released
    drop(health);

    // Use heal skill (slot 1 for Wavemaster)
    app.simulate_input(vec![GameAction::Skill1]);
    app.update(1.0); // Heal cast time + effect

    let health = app.world().get::<Health>(player).unwrap();
    assert!(
        health.current_hp > 50,
        "Heal should restore HP"
    );
}
```

### 4.2 Game State Transition Tests

Test that the state machine correctly manages system enable/disable:

```rust
// tests/integration/game_state_tests.rs
use shambala::*;

#[test]
fn test_game_boot_to_title_transition() {
    let mut app = TestApp::new();
    assert_eq!(app.game_phase(), GamePhase::Boot);

    app.trigger_event(GameEvent::InitComplete);
    app.update(0.1);

    assert_eq!(app.game_phase(), GamePhase::Title);
}

#[test]
fn test_combat_phase_disables_exploration_inputs() {
    let mut app = TestApp::new();

    // Start in exploration
    app.set_game_phase(GamePhase::Exploring);
    assert!(app.is_input_enabled(GameAction::MoveRight));

    // Enter combat
    app.set_game_phase(GamePhase::Combat);
    assert!(
        !app.is_input_enabled(GameAction::Interact),
        "Interact should be disabled during combat"
    );
}

#[test]
fn test_pause_during_exploration_freezes_physics() {
    let mut app = TestApp::new();
    app.set_game_phase(GamePhase::Exploring);

    let player = app.spawn_player(
        ClassType::HeavyBlade,
        Position::new(0.0, 0.0),
        Stats::default(),
    );

    // Apply velocity and pause
    app.world()
        .get_mut::<Velocity>(player)
        .unwrap()
        .set(100.0, 0.0);
    app.set_paused(true);
    app.update(1.0);

    // Position should not have changed while paused
    let pos = app.world().get::<Position>(player).unwrap();
    assert_eq!(pos.x, 0.0, "Position should not change while paused");
}
```

### 4.3 Full Scenario Tests

End-to-end tests that simulate a complete gameplay flow:

```rust
// tests/integration/data_drain_tests.rs
use shambala::*;

#[test]
fn test_data_drain_complete_cycle() {
    let mut app = TestApp::new();

    // ── 1. Setup ──
    let player = app.spawn_player(
        ClassType::Wavemaster,
        Position::new(0.0, 0.0),
        Stats::default(),
    );
    let enemies = app.spawn_enemy_group("goblin", 3, Position::new(50.0, 0.0));

    // ── 2. Defeat enemies to charge the gauge ──
    for enemy in &enemies {
        app.simulate_kill(*enemy);
    }
    app.update(0.1);

    let drain = app.world().get::<DataDrain>(player).unwrap();
    assert!(drain.gauge > 0.5, "Gauge should be partially charged after 3 kills");
    assert_eq!(drain.phase, DrainPhase::Charging);

    // ── 3. Activate Data Drain on a boss enemy ──
    let boss = app.spawn_boss("corrupted_knight", Position::new(80.0, 0.0));
    app.simulate_input(vec![GameAction::DataDrain]);
    app.update(0.2);

    let drain = app.world().get::<DataDrain>(player).unwrap();
    assert_eq!(drain.phase, DrainPhase::Targeting);
    assert!(drain.minigame_active);

    // ── 4. Complete the minigame ──
    app.simulate_drain_minigame(true, 85); // 85% success score
    app.update(0.5);

    let drain = app.world().get::<DataDrain>(player).unwrap();
    assert_eq!(drain.phase, DrainPhase::Complete);

    // ── 5. Verify rewards ──
    let inventory = app.world().get::<Inventory>(player).unwrap();
    let has_virus_core = inventory
        .slots
        .iter()
        .any(|s| matches!(s, Some(ItemData { item_type: ItemType::VirusCore, .. })));
    assert!(has_virus_core, "Should have received a Virus Core");

    let boss_corruption = app.world().get::<CorruptionMeter>(boss).unwrap();
    assert!(
        boss_corruption.current > 0.0,
        "Boss should have accumulated corruption"
    );
}

#[test]
fn test_failed_drain_minigame_applies_data_overflow() {
    let mut app = TestApp::new();
    let player = app.spawn_player(
        ClassType::TwinBlade,
        Position::new(0.0, 0.0),
        Stats::default(),
    );
    let enemy = app.spawn_enemy(EnemyCategory::Goblin, 1, Position::new(30.0, 0.0));

    // Kill enemy to charge gauge
    app.simulate_kill(enemy);
    app.update(0.1);

    // Activate and fail the minigame
    app.simulate_input(vec![GameAction::DataDrain]);
    app.update(0.2);
    app.simulate_drain_minigame(false, 30); // Failed with 30% score
    app.update(0.5);

    // Player should have Data Overflow debuff
    let status = app.world().get::<StatusEffects>(player).unwrap();
    let has_overflow = status
        .effects
        .iter()
        .any(|e| matches!(e.effect_type, StatusEffectType::DataOverflow));
    assert!(has_overflow, "Failed drain should cause Data Overflow");
}
```

### 4.4 Test Organization Principles

| Principle | Rationale | Example |
|---|---|---|
| **One scenario per file** | Keeps integration tests focused and readable | `combat_tests.rs`, `data_drain_tests.rs` |
| **Shared helpers in `tests/helpers/`** | Avoid duplication across test files | `test_ecs.rs`, `fixtures.rs` |
| **Use `#[ignore]` for slow tests** | Keep `cargo test` fast; run slow tests in CI only | `#[ignore]` on full scenario tests |
| **Deterministic seeds** | All procgen tests use fixed seeds | `seed: 42` |
| **Clean up world** | Each test gets a fresh world via `TestApp::new()` | No shared mutable state |

---

## 5. Property-Based Testing

### 5.1 Why Property-Based Testing?

Traditional unit tests check specific input/output pairs. Property-based testing with [`proptest`](https://crates.io/crates/proptest) checks **invariants** across a wide range of random inputs, finding edge cases you didn't think to test.

### 5.2 Combat Damage Invariants

```rust
// tests/property/combat_properties.rs
use proptest::prelude::*;
use shambala::*;

proptest! {
    // ── Damage Formula Invariants ──

    #[test]
    fn test_damage_is_always_at_least_one(
        attack in 0..9999u32,
        defense in 0..9999u32,
        multiplier in 0.0f32..10.0,
    ) {
        let atk_stats = Stats {
            attack,
            ..Default::default()
        };
        let def_stats = Stats {
            defense,
            ..Default::default()
        };
        let damage = calculate_damage(&atk_stats, &def_stats, multiplier);

        prop_assert!(
            damage >= 1,
            "Damage must always be >= 1, got {} with attack={} def={} mult={}",
            damage, attack, defense, multiplier
        );
    }

    #[test]
    fn test_damage_is_at_most_double_attack(
        attack in 1..5000u32,
        defense in 0..5000u32,
        multiplier in 0.5f32..5.0,
    ) {
        let atk_stats = Stats {
            attack,
            ..Default::default()
        };
        let def_stats = Stats {
            defense,
            ..Default::default()
        };
        let damage = calculate_damage(&atk_stats, &def_stats, multiplier);

        let theoretical_max = (attack as f32 * multiplier * 2.0) as u32;
        prop_assert!(
            damage <= theoretical_max,
            "Damage {} exceeds theoretical max {}",
            damage, theoretical_max
        );
    }

    #[test]
    fn test_higher_attack_always_means_more_or_equal_damage(
        attack in 1..5000u32,
        defense in 0..5000u32,
        multiplier in 0.5f32..5.0,
    ) {
        let low_atk = Stats { attack, ..Default::default() };
        let high_atk = Stats { attack: attack + 100, ..Default::default() };
        let def_stats = Stats { defense, ..Default::default() };

        let low_dmg = calculate_damage(&low_atk, &def_stats, multiplier);
        let high_dmg = calculate_damage(&high_atk, &def_stats, multiplier);

        prop_assert!(
            high_dmg >= low_dmg,
            "Higher attack should never produce less damage"
        );
    }

    #[test]
    fn test_higher_defense_reduces_damage(
        attack in 50..5000u32,
        defense_low in 0..1000u32,
        defense_high in 2000..5000u32,
        multiplier in 0.5f32..5.0,
    ) {
        let atk_stats = Stats { attack, ..Default::default() };
        let low_def = Stats { defense: defense_low, ..Default::default() };
        let high_def = Stats { defense: defense_high, ..Default::default() };

        let low_def_dmg = calculate_damage(&atk_stats, &low_def, multiplier);
        let high_def_dmg = calculate_damage(&atk_stats, &high_def, multiplier);

        prop_assert!(
            high_def_dmg <= low_def_dmg,
            "Higher defense should never increase damage taken"
        );
    }

    // ── Critical Hit Invariants ──

    #[test]
    fn test_crit_rate_between_zero_and_one(
        base_crit_rate in 0.0f32..1.0,
        rng_seed in any::<u64>(),
    ) {
        let stats = Stats {
            crit_rate: base_crit_rate,
            ..Default::default()
        };
        let mut rng = SeededRng::new(rng_seed);

        // Run 1000 attacks and count crits
        let crit_count = (0..1000)
            .filter(|_| is_critical_hit(&stats, &mut rng))
            .count();
        let observed_rate = crit_count as f32 / 1000.0;

        // Should be within reasonable variance
        let expected_variance = (base_crit_rate * (1.0 - base_crit_rate) / 1000.0).sqrt() * 3.0;
        prop_assert!(
            (observed_rate - base_crit_rate).abs() < expected_variance + 0.05,
            "Crit rate {} observed as {} after 1000 trials (expected variance {})",
            base_crit_rate, observed_rate, expected_variance
        );
    }

    // ── Status Effect Invariants ──

    #[test]
    fn test_status_effect_damage_over_time(
        tick_damage in 1..100u32,
        ticks in 1..20u32,
        tick_interval in 0.5f32..5.0,
    ) {
        let mut health = Health::new(9999, 999);
        let effect = StatusEffectInstance {
            effect_type: StatusEffectType::Poison,
            magnitude: tick_damage as f32,
            tick_interval,
            tick_timer: 0.0,
            remaining_duration: tick_interval * ticks as f32,
            ..Default::default()
        };

        let total_time = tick_interval * (ticks as f32 + 0.5);
        let mut elapsed = 0.0;
        while elapsed < total_time {
            tick_status_effect(&effect, &mut health, tick_interval);
            elapsed += tick_interval;
        }

        let expected_damage = tick_damage * ticks;
        let actual_damage = 9999 - health.current_hp;
        prop_assert!(
            (actual_damage as i32 - expected_damage as i32).abs() <= tick_damage as i32,
            "Poison should deal approximately {} damage, got {}",
            expected_damage, actual_damage
        );
    }
}
```

### 5.3 Area Generation Invariants

```rust
// tests/property/area_generation_properties.rs
use proptest::prelude::*;
use shambala::*;

proptest! {
    // ── Terrain Generation Invariants ──

    #[test]
    fn test_area_has_valid_dimensions(
        seed in any::<u64>(),
        width in 50u32..200,
        height in 50u32..200,
        terrain in prop_oneof![
            Just(TerrainType::Forest),
            Just(TerrainType::Desert),
            Just(TerrainType::Ice),
            Just(TerrainType::Volcano),
            Just(TerrainType::Ruins),
            Just(TerrainType::Void),
        ],
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(terrain, DifficultyLevel::Normal, width, height);

        prop_assert_eq!(area.map_width, width, "Map width should match request");
        prop_assert_eq!(area.map_height, height, "Map height should match request");
        prop_assert_eq!(
            area.tiles.len() as u32,
            width * height,
            "Tile count should equal width * height"
        );
    }

    #[test]
    fn test_area_has_entrance_and_exit(
        seed in any::<u64>(),
        terrain in prop_oneof![
            Just(TerrainType::Forest),
            Just(TerrainType::Desert),
            Just(TerrainType::Ice),
            Just(TerrainType::Volcano),
            Just(TerrainType::Ruins),
            Just(TerrainType::Void),
        ],
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(terrain, DifficultyLevel::Normal, 100, 100);

        prop_assert!(
            area.entrance.is_some(),
            "Area must have an entrance marker"
        );
        prop_assert!(
            area.exit.is_some(),
            "Area must have an exit marker"
        );
    }

    #[test]
    fn test_entrance_and_exit_are_within_bounds(
        seed in any::<u64>(),
        width in 50u32..200,
        height in 50u32..200,
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(TerrainType::Forest, DifficultyLevel::Normal, width, height);

        if let Some((ex, ey)) = area.entrance {
            prop_assert!(ex < width, "Entrance X {} out of bounds width {}", ex, width);
            prop_assert!(ey < height, "Entrance Y {} out of bounds height {}", ey, height);
        }
        if let Some((xx, xy)) = area.exit {
            prop_assert!(xx < width, "Exit X {} out of bounds width {}", xx, width);
            prop_assert!(xy < height, "Exit Y {} out of bounds height {}", xy, height);
        }
    }

    #[test]
    fn test_path_exists_between_entrance_and_exit(
        seed in any::<u64>(),
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(TerrainType::Ruins, DifficultyLevel::Normal, 100, 100);

        let entrance = area.entrance.expect("Must have entrance");
        let exit = area.exit.expect("Must have exit");

        let path = find_path(
            &area.tiles,
            area.map_width,
            area.map_height,
            entrance,
            exit,
        );

        prop_assert!(
            path.is_some(),
            "A walkable path must exist between entrance {:?} and exit {:?}",
            entrance, exit
        );
    }

    #[test]
    fn test_path_does_not_cover_entire_map(
        seed in any::<u64>(),
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(TerrainType::Forest, DifficultyLevel::Normal, 100, 100);

        let entrance = area.entrance.unwrap();
        let exit = area.exit.unwrap();

        if let Some(path) = find_path(&area.tiles, area.map_width, area.map_height, entrance, exit) {
            let map_area = area.map_width * area.map_height;
            prop_assert!(
                (path.len() as u32) < map_area * 80 / 100,
                "Path length {} should not exceed 80% of map area {}",
                path.len(),
                map_area
            );
        }
    }

    #[test]
    fn test_walkable_tiles_percentage(
        seed in any::<u64>(),
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(TerrainType::Desert, DifficultyLevel::Normal, 100, 100);

        let walkable = area
            .tiles
            .iter()
            .filter(|&&t| t == 0) // Tile index 0 = walkable
            .count() as f32;
        let total = area.tiles.len() as f32;
        let percentage = walkable / total;

        prop_assert!(
            percentage > 0.25,
            "At least 25% of tiles must be walkable, got {:.1}%",
            percentage * 100.0
        );
        prop_assert!(
            percentage < 0.95,
            "At most 95% of tiles should be walkable, got {:.1}%",
            percentage * 100.0
        );
    }
}
```

### 5.4 Writing Good Property Tests

| Principle | Explanation | Example |
|---|---|---|
| **Test invariants, not values** | Check properties that must always hold | "Damage >= 1" not "Damage == 47" |
| **Use bounded strategies** | Constrain inputs to realistic ranges | `attack in 1..9999u32` not `any::<u32>()` |
| **Include edge cases explicitly** | Add `prop_oneof!` with edge values | Include 0, 1, MAX in the strategy |
| **Check multiple outputs** | Verify all states after an operation | HP, status effects, inventory all correct |
| **Test determinism** | Same input always produces same output | Same seed → same area generation |
| **Document failure cases** | Save proptest `.shrinking` output as unit tests | Add a concrete unit test for each discovered edge case |

### 5.5 Converting Shrunk Failures to Unit Tests

When `proptest` finds a failure, it **shrinks** the input to the minimal failing case. Add these as permanent unit tests:

```rust
// Found by proptest on 2026-05-01:
// Test: test_damage_is_always_at_least_one
// Shrunk input: attack=0, defense=9999, multiplier=0.0
#[test]
fn test_damage_minimum_when_attack_is_zero() {
    let atk_stats = Stats {
        attack: 0,
        ..Default::default()
    };
    let def_stats = Stats {
        defense: 9999,
        ..Default::default()
    };
    let damage = calculate_damage(&atk_stats, &def_stats, 0.0);
    assert_eq!(damage, 1, "Even with zero attack, damage should be 1");
}
```

---

## 6. Test Organization

### 6.1 Complete Directory Structure

```
shambala/
├── src/
│   ├── components/
│   │   ├── mod.rs
│   │   ├── position.rs          # + #[cfg(test)] mod tests
│   │   ├── health.rs            # + #[cfg(test)] mod tests
│   │   ├── combat.rs            # + #[cfg(test)] mod tests
│   │   ├── data_drain.rs        # + #[cfg(test)] mod tests
│   │   ├── party.rs             # + #[cfg(test)] mod tests
│   │   ├── skill.rs             # + #[cfg(test)] mod tests
│   │   ├── status.rs            # + #[cfg(test)] mod tests
│   │   └── ...
│   ├── systems/
│   │   ├── mod.rs
│   │   ├── combat.rs            # + #[cfg(test)] mod tests
│   │   ├── physics.rs           # + #[cfg(test)] mod tests
│   │   ├── data_drain.rs        # + #[cfg(test)] mod tests
│   │   ├── ai.rs                # + #[cfg(test)] mod tests
│   │   └── ...
│   ├── resources/               # + #[cfg(test)] mod tests in each
│   ├── procgen/                 # + #[cfg(test)] mod tests in each
│   ├── data/                    # + #[cfg(test)] mod tests in each
│   └── ...
│
├── tests/
│   ├── integration/
│   │   ├── combat_tests.rs          # Full combat scenarios
│   │   ├── area_generation_tests.rs # Area gen scenarios
│   │   ├── data_drain_tests.rs      # Data Drain complete cycle
│   │   ├── party_tests.rs           # Party mechanics
│   │   ├── physics_tests.rs         # Movement & collision
│   │   ├── save_load_tests.rs       # Save/load round-trip
│   │   ├── game_state_tests.rs      # State machine transitions
│   │   └── network_tests.rs         # Server-client event flow
│   │
│   ├── property/
│   │   ├── combat_properties.rs     # Damage formula invariants
│   │   ├── area_gen_properties.rs   # Map generation invariants
│   │   ├── data_drain_properties.rs # Drain mechanics invariants
│   │   ├── status_properties.rs     # Status effect invariants
│   │   └── party_properties.rs      # Bond XP invariants
│   │
│   └── helpers/
│       ├── mod.rs                   # Re-exports
│       ├── test_ecs.rs             # TestApp, TestWorld builder
│       └── fixtures.rs             # Sample entities, configs, data
│
├── benches/
│   ├── bench_main.rs               # Criterion entry point
│   ├── render_benchmarks.rs        # Sprite batching throughput
│   ├── physics_benchmarks.rs       # Collision query performance
│   ├── ai_benchmarks.rs            # Behavior tree evaluation
│   └── ecs_benchmarks.rs           # ECS query iteration speed
│
└── docs/
    ├── GDD.md                      # Game Design Document
    ├── TECHNICAL_DESIGN.md         # Technical Design Document
    └── TDD_GUIDE.md                # This document
```

### 6.2 Test File Naming Conventions

| Location | Pattern | Purpose |
|---|---|---|
| `src/*.rs` | `#[cfg(test)] mod tests { ... }` | Unit tests inline with source |
| `tests/integration/*_tests.rs` | `test_<scenario>` | Integration scenarios |
| `tests/property/*_properties.rs` | `proptest! { ... }` | Property-based invariants |
| `tests/helpers/*.rs` | `pub fn helper_name` | Shared test utilities |
| `benches/*_benchmarks.rs` | `fn bench_name(c: &mut Criterion)` | Performance benchmarks |

### 6.3 Test Attribute Conventions

```rust
// Fast unit tests — run on every `cargo test`
#[test]
fn test_quick_validation() { ... }

// Slow integration tests — run on `cargo test -- --include-ignored`
#[test]
#[ignore = "Slow: requires full ECS world setup"]
fn test_full_combat_scenario() { ... }

// Property tests — run on `cargo test` (proptest generates many cases)
#[test]
fn property_test_combat_invariants() { ... }

// Parameterized tests — run with multiple input combinations
#[test_case(10, 3, 7)]
#[test_case(100, 0, 100)]
#[test_case(1, 999, 0)]
fn test_take_damage_boundaries(start_hp: u32, damage: u32, expected_hp: u32) {
    let mut health = Health::new(100, 50);
    health.current_hp = start_hp;
    health.take_damage(damage);
    assert_eq!(health.current_hp, expected_hp);
}

// Benchmark tests — run on `cargo bench`
#[bench]
fn bench_damage_calculation(b: &mut Bencher) { ... }
```

---

## 7. Mocking & Test Fixtures

### 7.1 TestApp — The Test World Builder

The central test utility is [`TestApp`](tests/helpers/test_ecs.rs). It wraps a minimal ECS [`World`](https://docs.rs/bevy_ecs) and provides convenience methods for tests.

```rust
// tests/helpers/test_ecs.rs
use bevy_ecs::world::World;
use bevy_ecs::system::Resource;
use shambala::*;

/// A test application that manages a minimal ECS world for testing.
///
/// Usage:
/// ```rust
/// let mut app = TestApp::new();
/// let player = app.spawn_player(ClassType::TwinBlade, pos, stats);
/// app.simulate_input(vec![GameAction::MoveRight]);
/// app.update(1.0);
/// ```
pub struct TestApp {
    world: World,
    systems: Vec<Box<dyn SystemFn>>,
    time: f32,
}

impl TestApp {
    /// Create a new test world with default resources
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(GameState::new());
        world.insert_resource(Time::new());
        world.insert_resource(InputState::new());

        Self {
            world,
            systems: Vec::new(),
            time: 0.0,
        }
    }

    /// Register a system to run during updates
    pub fn add_system<F>(&mut self, system: F)
    where
        F: Fn(&mut World) + 'static,
    {
        self.systems.push(Box::new(system));
    }

    /// Advance the simulation by `delta` seconds
    pub fn update(&mut self, delta: f32) {
        self.world
            .get_resource_mut::<Time>()
            .unwrap()
            .delta = delta;
        self.world
            .get_resource_mut::<Time>()
            .unwrap()
            .elapsed += delta;
        self.time += delta;

        for system in &self.systems {
            system(&mut self.world);
        }
    }

    // ── Entity Spawners ──

    pub fn spawn_player(
        &mut self,
        class: ClassType,
        position: Position,
        stats: Stats,
    ) -> Entity {
        // Construct a full player entity with all required components
        self.world.spawn((
            position,
            stats,
            Health::new(100, 50),
            Combat::new(class),
            SkillSet::for_class(class),
            Inventory::new(20),
            DataDrain::new(),
            PartyMember::new_leader(),
            Renderable::player_sprite(class),
            Animation::idle(),
        )).id()
    }

    pub fn spawn_enemy(
        &mut self,
        category: EnemyCategory,
        level: u32,
        position: Position,
    ) -> Entity {
        let template = EnemyTemplate::for_category(category, level);
        self.world.spawn((
            position,
            template.stats,
            Health::new(template.max_hp, 0),
            template.combat,
            AIState::for_enemy(category),
            EnemyType::new(category, level),
            Renderable::enemy_sprite(category),
            Animation::idle(),
            CorruptionMeter::new(template.corruption_resistance),
        )).id()
    }

    pub fn spawn_boss(
        &mut self,
        boss_id: &str,
        position: Position,
    ) -> Entity {
        let template = BossTemplate::from_id(boss_id);
        self.world.spawn((
            position,
            template.stats,
            Health::new(template.max_hp, 0),
            template.combat,
            AIState::for_boss(),
            EnemyType::new(EnemyCategory::Boss, template.level),
            Renderable::boss_sprite(boss_id),
            Animation::idle(),
            CorruptionMeter::new(template.corruption_threshold),
            AreaData::boss_room(),
        )).id()
    }

    // ── Simulation Helpers ──

    pub fn simulate_input(&mut self, actions: Vec<GameAction>) {
        let mut input = self.world.get_resource_mut::<InputState>().unwrap();
        input.actions = actions;
    }

    pub fn simulate_kill(&mut self, entity: Entity) {
        // Set HP to 0 and mark as dead
        if let Some(mut health) = self.world.get_mut::<Health>(entity) {
            health.current_hp = 0;
        }
    }

    pub fn simulate_drain_minigame(&mut self, success: bool, score: u32) {
        let mut drain = self.world
            .get_resource_mut::<DataDrainState>()
            .unwrap();
        drain.minigame_result = Some(DrainMinigameResult { success, score });
    }

    // ── Accessors ──

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn game_phase(&self) -> GamePhase {
        self.world
            .get_resource::<GameState>()
            .unwrap()
            .current
            .clone()
    }
}
```

### 7.2 Test Fixtures

Common test data lives in [`tests/helpers/fixtures.rs`](tests/helpers/fixtures.rs):

```rust
// tests/helpers/fixtures.rs
use shambala::*;

/// Standard test player stats (Twin Blade level 1)
pub fn default_player_stats() -> Stats {
    Stats {
        level: 1,
        strength: 10,
        vitality: 8,
        intelligence: 5,
        agility: 12,
        attack: 25,
        defense: 15,
        speed: 10,
        crit_rate: 0.1,
        crit_damage: 1.5,
        ..Default::default()
    }
}

/// Standard test goblin stats (level 1)
pub fn default_goblin_stats() -> Stats {
    Stats {
        level: 1,
        strength: 5,
        vitality: 4,
        attack: 10,
        defense: 5,
        speed: 6,
        ..Default::default()
    }
}

/// A sample inventory with some items
pub fn sample_inventory() -> Inventory {
    Inventory {
        slots: vec![
            Some(ItemData {
                id: "potion_hp_small".into(),
                item_type: ItemType::Consumable,
                quantity: 5,
                max_stack: 10,
                rarity: ItemRarity::Common,
                stats: None,
            }),
            Some(ItemData {
                id: "virus_core_fire".into(),
                item_type: ItemType::VirusCore,
                quantity: 3,
                max_stack: 99,
                rarity: ItemRarity::Uncommon,
                stats: None,
            }),
            None,
            None,
            None,
        ],
        max_slots: 20,
        gold: 500,
    }
}

/// Sample skill set for a level 1 Twin Blade
pub fn twin_blade_skills() -> SkillSet {
    SkillSet {
        skills: [
            Some(Skill {
                id: "tb_sword_flurry".into(),
                name: "Sword Flurry".into(),
                skill_type: SkillType::Melee,
                damage_multiplier: 1.5,
                sp_cost: 15,
                cooldown: 3.0,
                current_cooldown: 0.0,
                cast_time: 0.2,
                range: 48.0,
                area_of_effect: 0.0,
                status_effect: None,
                animation_key: "tb_attack_3".into(),
            }),
            Some(Skill {
                id: "tb_quick_step".into(),
                name: "Quick Step".into(),
                skill_type: SkillType::Utility,
                damage_multiplier: 0.0,
                sp_cost: 10,
                cooldown: 5.0,
                current_cooldown: 0.0,
                cast_time: 0.1,
                range: 0.0,
                area_of_effect: 0.0,
                status_effect: Some(StatusEffectData {
                    effect_type: StatusEffectType::Haste,
                    duration: 3.0,
                    magnitude: 50.0,
                    tick_interval: 1.0,
                }),
                animation_key: "tb_dodge".into(),
            }),
            None, None, None, None,
        ],
    }
}

/// A combat scenario configuration for parameterized tests
pub struct CombatScenario {
    pub attacker: (Stats, Health, Combat),
    pub defender: (Stats, Health, Combat),
    pub expected_damage_range: (u32, u32),
    pub expected_result: CombatResult,
}

/// Pre-built combat scenarios
pub fn combat_scenarios() -> Vec<CombatScenario> {
    vec![
        CombatScenario {
            attacker: (default_player_stats(), Health::new(100, 50), Combat::default()),
            defender: (default_goblin_stats(), Health::new(50, 0), Combat::default()),
            expected_damage_range: (5, 30),
            expected_result: CombatResult::Hit,
        },
        // ... more scenarios
    ]
}

/// Sample area keywords for generation tests
pub fn sample_area_keywords() -> AreaKeywords {
    AreaKeywords {
        terrain: TerrainType::Forest,
        difficulty: DifficultyLevel::Normal,
        weather: WeatherType::Clear,
        modifier: AreaModifier::None,
    }
}
```

### 7.3 Mocking with `mockall`

Use [`mockall`](https://crates.io/crates/mockall) for systems with external dependencies:

```rust
// src/render/renderer.rs
#[cfg_attr(test, mockall::automock)]
pub trait GpuRenderer {
    fn create_texture(&mut self, data: &[u8], width: u32, height: u32) -> TextureId;
    fn draw_sprite(&mut self, texture: TextureId, position: (f32, f32), size: (f32, f32));
    fn present(&mut self) -> Result<(), RenderError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_render_system_with_mock_gpu() {
        let mut mock_renderer = MockGpuRenderer::new();

        // Expect a texture creation call
        mock_renderer
            .expect_create_texture()
            .with(always(), always(), always())
            .returning(|_, _, _| TextureId(1));

        // Expect a draw call
        mock_renderer
            .expect_draw_sprite()
            .with(
                eq(TextureId(1)),
                eq((100.0, 200.0)),
                eq((32.0, 32.0)),
            )
            .return_const(());

        // Expect present
        mock_renderer
            .expect_present()
            .returning(|| Ok(()));

        // Run the system with the mock
        let mut system = RenderSystem::new(Box::new(mock_renderer));
        system.render(&[RenderCommand::DrawSprite {
            texture: TextureId(1),
            position: (100.0, 200.0),
            size: (32.0, 32.0),
        }]);
    }
}
```

### 7.4 Mock Resources for Headless Testing

Some resources (GPU, audio, file I/O) are unavailable in CI. Use mock implementations:

```rust
// tests/helpers/test_ecs.rs

/// A mock audio manager that records calls instead of playing sounds
pub struct MockAudioManager {
    pub played_music: Vec<String>,
    pub played_sfx: Vec<String>,
}

impl MockAudioManager {
    pub fn new() -> Self {
        Self {
            played_music: Vec::new(),
            played_sfx: Vec::new(),
        }
    }

    pub fn play_music(&mut self, track_id: &str) {
        self.played_music.push(track_id.to_string());
    }

    pub fn play_sfx(&mut self, sfx_id: &str) {
        self.played_sfx.push(sfx_id.to_string());
    }
}

/// A mock asset manager that returns dummy handles
pub struct MockAssetManager {
    pub loaded_textures: Vec<String>,
}

impl MockAssetManager {
    pub fn new() -> Self {
        Self {
            loaded_textures: Vec::new(),
        }
    }

    pub fn texture_exists(&self, id: &str) -> bool {
        self.loaded_textures.contains(&id.to_string())
    }

    pub fn load_texture(&mut self, id: &str) {
        self.loaded_textures.push(id.to_string());
    }
}
```

---

## 8. TDD Workflow for Game Dev

### 8.1 The Complete Workflow

```
┌─────────────────────────────────────────────────────────────────────┐
│                    TDD WORKFLOW FOR SHAMBALA                         │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  1. PICK A FEATURE                                            │   │
│  │     └── From the sprint backlog or issue tracker               │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│                              ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  2. UNDERSTAND THE BEHAVIOR                                   │   │
│  │     └── Read the GDD / TECHNICAL_DESIGN spec for this feature │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│                              ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  3. WRITE A FAILING TEST 🔴                                  │   │
│  │     ├── Unit test for a component method                      │   │
│  │     ├── Unit test for a system function                       │   │
│  │     ├── Property test for invariants                          │   │
│  │     └── Integration test for a scenario                       │   │
│  │     └── Run: cargo test → test FAILS (expected)               │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│                              ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  4. IMPLEMENT MINIMUM CODE 🟢                                │   │
│  │     ├── Write the simplest code to make the test pass         │   │
│  │     ├── No optimization, no gold-plating                      │   │
│  │     └── Run: cargo test → test PASSES                        │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│                              ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  5. REFACTOR 🔵                                               │   │
│  │     ├── Rename variables, extract methods, add docs           │   │
│  │     ├── Remove duplication                                    │   │
│  │     ├── Run: cargo test → still PASSES                       │   │
│  │     └── Run: cargo clippy → no warnings                      │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│                              ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  6. COMMIT                                                    │   │
│  │     └── Use conventional commit format (see Section 12)       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│                              ▼                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  7. REPEAT                                                    │   │
│  │     └── Next test for the same feature, or next feature      │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

### 8.2 Development Loop with `cargo-watch`

For continuous TDD, use [`cargo-watch`](https://crates.io/crates/cargo-watch):

```bash
# Install
cargo install cargo-watch

# Run all unit tests on every file change
cargo watch -x 'tdd'

# Run a specific test on every change
cargo watch -x 'test test_health_take_damage'

# Run tests + clippy on every change
cargo watch -x 'tdd' -x 'clippy'
```

### 8.3 TDD Decision Flowchart

When implementing a new feature, follow this decision tree:

```
New feature or bug fix?
        │
        ▼
Can I test this with a pure function?
        │
   ┌────┴────┐
   YES       NO
   │          │
   ▼          ▼
Write unit   Does this test systems interacting?
   test       │
   │      ┌───┴───┐
   │      YES      NO
   │      │        │
   │      ▼        ▼
   │   Write      Does this test random input invariants?
   │   integ.      │
   │   test    ┌───┴───┐
   │           YES      NO
   │           │        │
   │           ▼        ▼
   │        Write      Does this test performance?
   │        prop.      │
   │        test   ┌───┴───┐
   │               YES      NO
   │               │        │
   │               ▼        ▼
   │            Write      Write a test
   │            bench.     documenting the bug
   │               │        │
   └───────────────┴────────┘
                        │
                        ▼
         1. Write failing test (RED)
         2. Implement minimum code (GREEN)
         3. Refactor (REFACTOR)
         4. Commit
                        │
                        ▼
              Run `cargo t-all`
              ┌────┴────┐
             PASS       FAIL
              │          │
              ▼          ▼
            Done!     Fix code
           (commit)   or test
```

### 8.4 Four-Phase TDD for Game Systems

Game systems often have complex setup. Use this four-phase approach:

```rust
// Phase 1: Setup — Build the test world
let mut app = TestApp::new();
let player = app.spawn_player(
    ClassType::TwinBlade,
    Position::new(0.0, 0.0),
    default_player_stats(),
);
let enemy = app.spawn_enemy(
    EnemyCategory::Goblin,
    1,
    Position::new(50.0, 0.0),
);

// Phase 2: Act — Simulate the game action
app.simulate_input(vec![GameAction::MoveRight]);
app.update(1.5);
app.simulate_input(vec![GameAction::BasicAttack]);
app.update(0.5);

// Phase 3: Assert — Verify the outcome
let enemy_health = app.world().get::<Health>(enemy).unwrap();
assert!(enemy_health.current_hp < enemy_health.max_hp);

// Phase 4: Tear Down — World is dropped automatically
// (The World and TestApp go out of scope)
```

---

## 9. Example TDD Sessions

### 9.1 TDD for a Health Component

**Feature:** The `Health` component needs to support damage, healing, regen, and death.

#### 🔴 RED — Write failing tests

```rust
// src/components/health.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new_sets_hp_to_max() {
        let health = Health::new(100, 50);
        assert_eq!(health.current_hp, 100);
        assert_eq!(health.max_hp, 100);
    }

    #[test]
    fn test_take_damage_reduces_hp() {
        let mut health = Health::new(100, 50);
        health.take_damage(30);
        assert_eq!(health.current_hp, 70);
    }

    #[test]
    fn test_take_damage_clamps_to_zero() {
        let mut health = Health::new(100, 50);
        health.take_damage(999);
        assert_eq!(health.current_hp, 0);
    }

    #[test]
    fn test_heal_restores_hp() {
        let mut health = Health::new(100, 50);
        health.current_hp = 50;
        health.heal(30);
        assert_eq!(health.current_hp, 80);
    }

    #[test]
    fn test_heal_does_not_exceed_max() {
        let mut health = Health::new(100, 50);
        health.current_hp = 90;
        health.heal(999);
        assert_eq!(health.current_hp, 100);
    }

    #[test]
    fn test_is_alive_returns_true_when_hp_positive() {
        let health = Health::new(100, 50);
        assert!(health.is_alive());
    }

    #[test]
    fn test_is_alive_returns_false_when_hp_zero() {
        let mut health = Health::new(100, 50);
        health.current_hp = 0;
        assert!(!health.is_alive());
    }
}
```

**Run:** `cargo test` → All tests fail (`Health::new` doesn't exist yet).

#### 🟢 GREEN — Implement minimum code

```rust
// src/components/health.rs

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    pub current_hp: u32,
    pub max_hp: u32,
    pub current_sp: u32,
    pub max_sp: u32,
    pub hp_regen: f32,
    pub sp_regen: f32,
    pub invulnerable: bool,
    pub invuln_timer: f32,
}

impl Health {
    pub fn new(max_hp: u32, max_sp: u32) -> Self {
        Self {
            current_hp: max_hp,
            max_hp,
            current_sp: max_sp,
            max_sp,
            hp_regen: 0.0,
            sp_regen: 0.0,
            invulnerable: false,
            invuln_timer: 0.0,
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        if self.invulnerable {
            return;
        }
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.current_hp = (self.current_hp + amount).min(self.max_hp);
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}
```

**Run:** `cargo test` → All 7 tests pass.

#### 🔵 REFACTOR — Clean up

```rust
impl Health {
    /// Creates a new Health component with full HP and SP.
    pub fn new(max_hp: u32, max_sp: u32) -> Self {
        Self {
            current_hp: max_hp,
            max_hp,
            current_sp: max_sp,
            max_sp,
            hp_regen: DEFAULT_HP_REGEN,
            sp_regen: DEFAULT_SP_REGEN,
            invulnerable: false,
            invuln_timer: 0.0,
        }
    }

    /// Applies damage, respecting invulnerability.
    /// Returns the actual damage dealt.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        if self.invulnerable {
            return 0;
        }
        let actual = amount.min(self.current_hp);
        self.current_hp = self.current_hp.saturating_sub(amount);
        actual
    }

    /// Heals HP without exceeding max_hp.
    /// Returns the actual amount healed.
    pub fn heal(&mut self, amount: u32) -> u32 {
        let before = self.current_hp;
        self.current_hp = (self.current_hp + amount).min(self.max_hp);
        self.current_hp - before
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}
```

**Run:** `cargo test` → Still passes. Run `cargo clippy` → No warnings.

**Commit:**
```
feat(components): add Health component with damage, healing, and death

- Implemented take_damage with invulnerability guard
- Implemented heal with max_hp clamping
- Added is_alive/is_dead checks
- 7 unit tests covering all methods and edge cases
```

### 9.2 TDD for a CombatSystem

**Feature:** `CombatSystem` processes attack events, calculates damage, applies to HP.

#### 🔴 RED — Write failing tests

```rust
// src/systems/combat.rs

#[cfg(test)]
mod tests {
    use super::*;

    fn default_combat_world() -> (World, Entity, Entity) {
        let mut world = World::new();
        world.insert_resource(GameState::new());
        world.insert_resource(Time::new());

        let player = world.spawn((
            Stats { attack: 50, defense: 20, ..Default::default() },
            Health::new(200, 100),
            Combat::for_class(ClassType::HeavyBlade),
        )).id();

        let goblin = world.spawn((
            Stats { attack: 10, defense: 5, ..Default::default() },
            Health::new(50, 0),
            Combat::default(),
            EnemyType::new(EnemyCategory::Goblin, 1),
        )).id();

        (world, player, goblin)
    }

    #[test]
    fn test_basic_attack_damages_enemy() {
        let (mut world, player, goblin) = default_combat_world();
        let mut system = CombatSystem;

        // Fire an attack event
        world.send_event(AttackEvent {
            source: player,
            target: goblin,
            skill_id: None,
            position: (0.0, 0.0),
        });

        system.run(&mut world);

        let goblin_hp = world.get::<Health>(goblin).unwrap();
        assert!(goblin_hp.current_hp < 50, "Goblin should have taken damage");
    }

    #[test]
    fn test_dead_entity_cannot_attack() {
        let (mut world, player, goblin) = default_combat_world();

        // Kill the goblin first
        world.get_mut::<Health>(goblin).unwrap().current_hp = 0;

        world.send_event(AttackEvent {
            source: goblin,  // Dead goblin attacks player
            target: player,
            skill_id: None,
            position: (0.0, 0.0),
        });

        let mut system = CombatSystem;
        system.run(&mut world);

        let player_hp = world.get::<Health>(player).unwrap();
        assert_eq!(player_hp.current_hp, 200, "Dead entities should not deal damage");
    }

    #[test]
    fn test_skill_uses_sp() {
        let (mut world, player, goblin) = default_combat_world();

        world.send_event(SkillEvent {
            source: player,
            target: goblin,
            skill_id: Some("tb_sword_flurry".into()),
            position: (0.0, 0.0),
        });

        let mut system = CombatSystem;
        system.run(&mut world);

        let player_health = world.get::<Health>(player).unwrap();
        assert!(
            player_health.current_sp < 50,
            "Using a skill should consume SP"
        );
    }

    #[test]
    fn test_attack_out_of_range_does_nothing() {
        let (mut world, player, goblin) = default_combat_world();

        // Place goblin far away
        world.get_mut::<Combat>(player).unwrap().attack_range = 48.0;
        // (Position component would be checked by the system)

        world.send_event(AttackEvent {
            source: player,
            target: goblin,
            skill_id: None,
            position: (1000.0, 1000.0), // Too far
        });

        let mut system = CombatSystem;
        system.run(&mut world);

        let goblin_hp = world.get::<Health>(goblin).unwrap();
        assert_eq!(goblin_hp.current_hp, 50, "Out-of-range attack should not hit");
    }
}
```

#### 🟢 GREEN — Implement minimum code

```rust
// src/systems/combat.rs

use bevy_ecs::prelude::*;

pub struct CombatSystem;

impl CombatSystem {
    pub fn run(&mut self, world: &mut World) {
        let mut events = world.get_resource_mut::<Events<AttackEvent>>().unwrap();
        let attacks: Vec<AttackEvent> = events.drain().collect();
        drop(events);

        for attack in attacks {
            self.process_attack(world, &attack);
        }
    }

    fn process_attack(&self, world: &mut World, attack: &AttackEvent) {
        // Skip if attacker is dead
        if let Some(health) = world.get::<Health>(attack.source) {
            if !health.is_alive() {
                return;
            }
        }

        // Get attacker and defender stats
        let attacker_stats = world.get::<Stats>(attack.source).unwrap().clone();
        let defender_stats = world.get::<Stats>(attack.target).unwrap().clone();

        // Calculate damage
        let damage = calculate_damage(&attacker_stats, &defender_stats, 1.0);

        // Apply damage
        if let Some(mut health) = world.get_mut::<Health>(attack.target) {
            health.take_damage(damage);
        }
    }
}

pub fn calculate_damage(attacker: &Stats, defender: &Stats, multiplier: f32) -> u32 {
    let base = attacker.attack as f32 * multiplier;
    let reduction = defender.defense as f32 * 0.5;
    let damage = (base - reduction).max(1.0);
    damage as u32
}
```

#### 🔵 REFACTOR

```rust
pub struct CombatSystem;

impl CombatSystem {
    /// Processes all pending AttackEvents for this frame.
    pub fn run(&mut self, world: &mut World) {
        let attacks = world
            .get_resource_mut::<Events<AttackEvent>>()
            .unwrap()
            .drain()
            .collect::<Vec<_>>();

        for attack in &attacks {
            self.process_attack(world, attack);
        }
    }

    fn process_attack(&self, world: &World, attack: &AttackEvent) {
        // Guard: skip if attacker is dead
        if world.get::<Health>(attack.source).is_some_and(|h| !h.is_alive()) {
            return;
        }

        // Guard: skip if attacker or defender is missing
        let (Ok(atk_stats), Ok(def_stats)) = (
            world.get::<Stats>(attack.source),
            world.get::<Stats>(attack.target),
        ) else {
            return;
        };

        let damage = calculate_damage(atk_stats, def_stats, 1.0);

        if let Some(mut health) = world.get_mut::<Health>(attack.target) {
            health.take_damage(damage);
        }
    }
}
```

**Commit:**
```
feat(systems): implement CombatSystem with damage calculation

- Added CombatSystem that processes AttackEvents
- Implemented calculate_damage with attack/defense formula
- Dead entities cannot attack
- Out-of-range attacks are rejected
- 4 unit tests covering core combat scenarios
```

### 9.3 TDD for Data Drain Mechanic

**Feature:** The Data Drain system manages gauge charging, activation, minigame, and rewards.

#### 🔴 RED — Write failing tests

```rust
// src/systems/data_drain.rs
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_drain_world() -> (World, Entity) {
        let mut world = World::new();
        world.insert_resource(Time::new());

        let player = world.spawn((
            DataDrain::new(),
            Stats::default(),
            Inventory::new(20),
            Position::new(0.0, 0.0),
        )).id();

        (world, player)
    }

    #[test]
    fn test_kill_enemy_charges_gauge() {
        let (mut world, player) = setup_drain_world();
        let mut system = DataDrainSystem;

        // Simulate an enemy death
        world.send_event(DeathEvent {
            entity: Entity::from_raw(999), // Some enemy
            killer: Some(player),
            position: (0.0, 0.0),
        });

        system.run(&mut world);

        let drain = world.get::<DataDrain>(player).unwrap();
        assert!(drain.gauge > 0.0, "Killing an enemy should charge the gauge");
    }

    #[test]
    fn test_gauge_caps_at_max() {
        let (mut world, player) = setup_drain_world();
        let mut system = DataDrainSystem;

        // Kill many enemies
        for _ in 0..100 {
            world.send_event(DeathEvent {
                entity: Entity::from_raw(rand::random()),
                killer: Some(player),
                position: (0.0, 0.0),
            });
        }

        system.run(&mut world);

        let drain = world.get::<DataDrain>(player).unwrap();
        assert!(drain.gauge <= 1.0, "Gauge should not exceed max (1.0)");
    }

    #[test]
    fn test_activate_data_drain_enters_minigame() {
        let (mut world, player) = setup_drain_world();
        let mut system = DataDrainSystem;

        // Fill gauge to 100%
        world.get_mut::<DataDrain>(player).unwrap().gauge = 1.0;

        // Activate drain
        world.send_event(DataDrainEvent {
            source: player,
            target: Entity::from_raw(1),
        });

        system.run(&mut world);

        let drain = world.get::<DataDrain>(player).unwrap();
        assert_eq!(drain.phase, DrainPhase::Minigame);
        assert!(drain.minigame_active);
    }

    #[test]
    fn test_successful_minigame_gives_virus_core() {
        let (mut world, player) = setup_drain_world();
        let mut system = DataDrainSystem;

        // Full gauge and activate
        {
            let drain = world.get_mut::<DataDrain>(player).unwrap();
            drain.gauge = 1.0;
            drain.phase = DrainPhase::Minigame;
            drain.minigame_active = true;
        }

        // Complete minigame successfully
        world.send_event(DrainResultEvent {
            source: player,
            success: true,
            score: 85,
        });

        system.run(&mut world);

        let drain = world.get::<DataDrain>(player).unwrap();
        assert_eq!(drain.phase, DrainPhase::Complete);
        assert_eq!(drain.gauge, 0.0, "Gauge should reset after successful drain");

        let inventory = world.get::<Inventory>(player).unwrap();
        let has_virus_core = inventory
            .slots
            .iter()
            .any(|s| matches!(s, Some(ItemData { item_type: ItemType::VirusCore, .. })));
        assert!(has_virus_core, "Should have received a Virus Core");
    }

    #[test]
    fn test_failed_minigame_applies_data_overflow() {
        let (mut world, player) = setup_drain_world();
        let mut system = DataDrainSystem;

        {
            let drain = world.get_mut::<DataDrain>(player).unwrap();
            drain.gauge = 1.0;
            drain.phase = DrainPhase::Minigame;
            drain.minigame_active = true;
        }

        // Fail minigame
        world.send_event(DrainResultEvent {
            source: player,
            success: false,
            score: 20,
        });

        system.run(&mut world);

        let drain = world.get::<DataDrain>(player).unwrap();
        assert_eq!(drain.phase, DrainPhase::Failed);

        let status = world.get::<StatusEffects>(player).unwrap();
        let has_overflow = status
            .effects
            .iter()
            .any(|e| matches!(e.effect_type, StatusEffectType::DataOverflow));
        assert!(has_overflow, "Failed drain should apply Data Overflow debuff");
    }
}
```

#### 🟢 GREEN — Implement minimum code

```rust
pub struct DataDrainSystem;

impl DataDrainSystem {
    pub fn run(&mut self, world: &mut World) {
        // Process death events → charge gauge
        let deaths = world
            .get_resource_mut::<Events<DeathEvent>>()
            .unwrap()
            .drain()
            .collect::<Vec<_>>();

        for death in &deaths {
            if let Some(killer) = death.killer {
                if let Some(mut drain) = world.get_mut::<DataDrain>(killer) {
                    drain.gauge = (drain.gauge + drain.charge_rate).min(1.0);
                }
            }
        }

        // Process drain activations
        let activations = world
            .get_resource_mut::<Events<DataDrainEvent>>()
            .unwrap()
            .drain()
            .collect::<Vec<_>>();

        for activation in &activations {
            if let Some(mut drain) = world.get_mut::<DataDrain>(activation.source) {
                if drain.gauge >= 1.0 {
                    drain.phase = DrainPhase::Minigame;
                    drain.minigame_active = true;
                    drain.gauge = 0.0;
                }
            }
        }

        // Process minigame results
        let results = world
            .get_resource_mut::<Events<DrainResultEvent>>()
            .unwrap()
            .drain()
            .collect::<Vec<_>>();

        for result in &results {
            if let Some(mut drain) = world.get_mut::<DataDrain>(result.source) {
                if result.success {
                    drain.phase = DrainPhase::Complete;
                    Self::reward_player(world, result.source);
                } else {
                    drain.phase = DrainPhase::Failed;
                    Self::apply_data_overflow(world, result.source);
                }
            }
        }
    }

    fn reward_player(world: &mut World, player: Entity) {
        if let Some(mut inventory) = world.get_mut::<Inventory>(player) {
            let _ = inventory.add_item(ItemData {
                id: "virus_core_generic".into(),
                item_type: ItemType::VirusCore,
                quantity: 1,
                max_stack: 99,
                rarity: ItemRarity::Common,
                stats: None,
            });
        }
    }

    fn apply_data_overflow(world: &mut World, player: Entity) {
        if let Some(mut status) = world.get_mut::<StatusEffects>(player) {
            status.effects.push(StatusEffectInstance {
                effect_id: "data_overflow".into(),
                effect_type: StatusEffectType::DataOverflow,
                remaining_duration: 30.0,
                tick_interval: 0.0,
                tick_timer: 0.0,
                magnitude: 0.5,
                source: "data_drain".into(),
            });
        }
    }
}
```

**Commit:**
```
feat(systems): implement DataDrainSystem with full lifecycle

- Charge gauge on enemy kills with max cap
- Activate drain when gauge is full, transition to minigame
- Successful minigame grants Virus Core reward
- Failed minigame applies Data Overflow debuff
- 5 unit tests covering gauge, activation, success, and failure
```

### 9.4 TDD for Area Generation

**Feature:** The area generator creates valid, playable maps from keywords and a seed.

#### 🔴 RED — Write failing tests

```rust
// src/procgen/mod.rs
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEED: u64 = 42;

    #[test]
    fn test_generate_area_has_correct_dimensions() {
        let generator = AreaGenerator::new(TEST_SEED);
        let area = generator.generate(
            TerrainType::Forest,
            DifficultyLevel::Normal,
            100, 100,
        );
        assert_eq!(area.map_width, 100);
        assert_eq!(area.map_height, 100);
        assert_eq!(area.tiles.len(), 100 * 100);
    }

    #[test]
    fn test_generate_area_deterministic() {
        let gen_a = AreaGenerator::new(TEST_SEED);
        let area_a = gen_a.generate(TerrainType::Forest, DifficultyLevel::Normal, 50, 50);

        let gen_b = AreaGenerator::new(TEST_SEED);
        let area_b = gen_b.generate(TerrainType::Forest, DifficultyLevel::Normal, 50, 50);

        assert_eq!(area_a.tiles, area_b.tiles);
    }

    #[test]
    fn test_area_has_entrance_and_exit() {
        let generator = AreaGenerator::new(TEST_SEED);
        let area = generator.generate(TerrainType::Forest, DifficultyLevel::Normal, 100, 100);

        assert!(area.entrance.is_some(), "Area must have an entrance");
        assert!(area.exit.is_some(), "Area must have an exit");
    }

    #[test]
    fn test_entrance_and_exit_are_different() {
        let generator = AreaGenerator::new(TEST_SEED);
        let area = generator.generate(TerrainType::Forest, DifficultyLevel::Normal, 100, 100);

        let entrance = area.entrance.unwrap();
        let exit = area.exit.unwrap();
        assert_ne!(entrance, exit, "Entrance and exit must be different positions");
    }

    #[test]
    fn test_walkable_path_exists() {
        let generator = AreaGenerator::new(TEST_SEED);
        let area = generator.generate(TerrainType::Forest, DifficultyLevel::Normal, 100, 100);

        let entrance = area.entrance.unwrap();
        let exit = area.exit.unwrap();
        let path = find_path(
            &area.tiles,
            area.map_width,
            area.map_height,
            entrance,
            exit,
        );

        assert!(path.is_some(), "A walkable path must exist between entrance and exit");
    }

    #[test]
    fn test_different_seeds_different_maps() {
        let gen_a = AreaGenerator::new(1);
        let area_a = gen_a.generate(TerrainType::Forest, DifficultyLevel::Normal, 50, 50);

        let gen_b = AreaGenerator::new(2);
        let area_b = gen_b.generate(TerrainType::Forest, DifficultyLevel::Normal, 50, 50);

        assert_ne!(area_a.tiles, area_b.tiles);
    }

    #[test]
    fn test_different_terrains_have_different_tilesets() {
        let generator = AreaGenerator::new(TEST_SEED);
        let forest = generator.generate(TerrainType::Forest, DifficultyLevel::Normal, 50, 50);
        let desert = generator.generate(TerrainType::Desert, DifficultyLevel::Normal, 50, 50);

        assert_ne!(forest.texture_id, desert.texture_id);
    }
}
```

#### 🟢 GREEN — Implement minimum code

```rust
pub struct AreaGenerator {
    rng: SeededRng,
}

impl AreaGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SeededRng::new(seed),
        }
    }

    pub fn generate(
        &mut self,
        terrain: TerrainType,
        difficulty: DifficultyLevel,
        width: u32,
        height: u32,
    ) -> GeneratedArea {
        let texture_id = match terrain {
            TerrainType::Forest => "tileset_forest",
            TerrainType::Desert => "tileset_desert",
            TerrainType::Ice => "tileset_ice",
            TerrainType::Volcano => "tileset_volcano",
            TerrainType::Ruins => "tileset_ruins",
            TerrainType::Void => "tileset_void",
        }.to_string();

        // Place entrance at top-left area, exit at bottom-right
        let entrance = (2, 2);
        let exit = (width - 3, height - 3);

        // Generate tiles using noise
        let tiles = self.generate_tiles(terrain, width, height);

        // Ensure walkable path
        let tiles = self.carve_path(tiles, width, height, entrance, exit);

        GeneratedArea {
            map_width: width,
            map_height: height,
            tiles,
            texture_id,
            entrance: Some(entrance),
            exit: Some(exit),
            collision_tiles: vec![false; (width * height) as usize],
            enemy_spawns: self.place_enemies(difficulty, width, height),
            chest_spawns: self.place_chests(difficulty, width, height),
        }
    }

    fn generate_tiles(&mut self, terrain: TerrainType, width: u32, height: u32) -> Vec<u32> {
        let total = (width * height) as usize;
        let mut tiles = vec![1u32; total]; // 1 = wall by default

        // Simple noise-based generation
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let noise = self.rng.f32();
                if noise > 0.4 {
                    tiles[(y * width + x) as usize] = 0; // walkable
                }
            }
        }

        tiles
    }

    fn carve_path(
        &self,
        mut tiles: Vec<u32>,
        width: u32,
        height: u32,
        start: (u32, u32),
        end: (u32, u32),
    ) -> Vec<u32> {
        // Simple straight path carving
        // (A real implementation would use BFS or A*)
        tiles
    }

    fn place_enemies(&mut self, difficulty: DifficultyLevel, width: u32, height: u32) -> Vec<EnemySpawn> {
        vec![]
    }

    fn place_chests(&mut self, difficulty: DifficultyLevel, width: u32, height: u32) -> Vec<ChestSpawn> {
        vec![]
    }
}
```

**Commit:**
```
feat(procgen): implement AreaGenerator with noise-based terrain

- Deterministic generation from seed
- Terrain-specific tileset selection
- Entrance/exit placement with path carving
- 7 unit tests covering dimensions, determinism, and connectivity
```

---

## 10. Coverage Goals

### 10.1 Minimum Coverage Targets by Module

| Module | Minimum Coverage | Critical Paths | Notes |
|---|---|---|---|
| [`components/health.rs`](src/components/health.rs) | 95% | Damage, healing, death, regen, invulnerability | Pure data — easy to test exhaustively |
| [`components/stats.rs`](src/components/stats.rs) | 95% | Stat calculations, level-up, modifiers | Serialization round-trips |
| [`components/combat.rs`](src/components/combat.rs) | 90% | Attack range, combo chain, attack speed | Edge cases at 0 values |
| [`components/data_drain.rs`](src/components/data_drain.rs) | 95% | Gauge progression, phase transitions | All `DrainPhase` variants |
| [`components/party.rs`](src/components/party.rs) | 90% | Bond XP thresholds, tactic enum | Deserialization from config |
| [`components/position.rs`](src/components/position.rs) | 95% | Movement, AABB overlaps, distance calc | Deterministic math |
| [`systems/combat.rs`](src/systems/combat.rs) | 95% | Damage calc, crits, healing, status, death | Property tests for invariants |
| [`systems/physics.rs`](src/systems/physics.rs) | 90% | Movement, collision, wall sliding, queries | Integration with rapier2d |
| [`systems/ai.rs`](src/systems/ai.rs) | 85% | Behavior transitions, patrol path, aggro | Complex state machines |
| [`systems/data_drain.rs`](src/systems/data_drain.rs) | 95% | Charge, activate, minigame, results, overflow | Full lifecycle coverage |
| [`systems/party.rs`](src/systems/party.rs) | 85% | Bond XP, companion swap, tactics | Integration with other systems |
| [`procgen/`](src/procgen/) | 90% | Valid map, path existence, seeding | Property tests mandatory |
| [`data/`](src/data/) | 90% | Config deserialization, validation, errors | Test every config file |
| [`entities/`](src/entities/) | 85% | Spawn with correct components, defaults | Verify all component combos |
| [`resources/`](src/resources/) | 90% | State transitions, default values | GamePhase enum coverage |
| [`render/`](src/render/) | 70% | Pipeline creation, buffer updates | Complex infrastructure |
| [`ui/`](src/ui/) | 60% | Widget layout, event routing | High-level visual testing |
| [`network/`](src/network/) | 85% | Event serialization, state consistency | Server emulator logic |
| [`events/`](src/events/) | 90% | Event dispatch, handler registration | All event types |
| [`util/`](src/util/) | 95% | Math helpers, RNG, timer | Pure utility functions |

### 10.2 Coverage Enforcement

Configure [`llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) in CI:

```yaml
# .github/workflows/ci.yml
- name: Check coverage
  run: |
    cargo llvm-cov --lcov --output-path lcov.info
    # Fail if overall coverage < 80%
    cargo llvm-cov report --fail-under-lines 80
```

Add a `coverage` alias to [`.cargo/config.toml`](.cargo/config.toml):

```toml
[alias]
coverage = "llvm-cov --lcov --output-path lcov.info"
coverage-report = "llvm-cov report --fail-under-lines 80"
```

### 10.3 Coverage Reporting

```bash
# Generate coverage report
cargo coverage

# View HTML report (opens in browser)
cargo llvm-cov report --open

# Check coverage with threshold
cargo llvm-cov report --fail-under-lines 85

# Generate CI-compatible report
cargo llvm-cov --lcov --output-path lcov.info
# Then upload to Codecov or Coveralls
```

---

## 11. CI/CD Integration

### 11.1 GitHub Actions Workflow

Create [`.github/workflows/ci.yml`](.github/workflows/ci.yml):

```yaml
name: CI

on:
  push:
    branches: [main, develop, 'feature/**']
  pull_request:
    branches: [main, develop]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  # ── Static Analysis ──
  lint:
    name: Lint & Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

  # ── Unit Tests (fast) ──
  unit-tests:
    name: Unit Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Run unit tests
        run: cargo test --lib --verbose

  # ── Integration Tests (slow) ──
  integration-tests:
    name: Integration Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Run integration tests
        run: cargo test --test '*' --verbose

      - name: Run property tests
        run: cargo test --test property_* --verbose

  # ── All Tests (cross-platform) ──
  all-tests:
    name: Tests (${{ matrix.os }})
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2
        with:
          key: ${{ matrix.os }}

      - name: Run all tests (headless)
        run: cargo test --features headless --verbose

  # ── Benchmarks (performance regression check) ──
  benchmarks:
    name: Benchmarks
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Run benchmarks (smoke test)
        run: cargo bench -- --sample-size 10 --measurement-time 1

      - name: Compare with baseline
        run: |
          # Store baseline on main branch, compare on PR
          if [ "${{ github.event_name }}" = "pull_request" ]; then
            cargo bench -- --baseline main --output-format bencher | tee bench_output.txt
            # Parse and comment on PR if regressions > 5%
          fi

  # ── Coverage ──
  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview

      - name: Install cargo-llvm-cov
        uses: taiki-e/install-action@cargo-llvm-cov

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Generate coverage report
        run: |
          cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: lcov.info
          fail_ci_if_error: true
          verbose: true

  # ── Build Check (ensure it compiles) ──
  build:
    name: Build Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Build
        run: cargo build --all-features --verbose

      - name: Build documentation
        run: cargo doc --no-deps --all-features

      - name: Check for dead code
        run: RUSTFLAGS="-D dead_code" cargo check --all-features
```

### 11.2 PR Checklist Automation

Create [`.github/pull_request_template.md`](.github/pull_request_template.md):

```markdown
## Description

<!-- Brief description of the feature or fix -->

## Type of Change

- [ ] feat: New feature
- [ ] fix: Bug fix
- [ ] test: Test addition/modification
- [ ] refactor: Code restructuring
- [ ] chore: Build/config changes

## TDD Checklist

- [ ] I wrote a **failing test** first (RED)
- [ ] I implemented the **minimum code** to pass (GREEN)
- [ ] I **refactored** while keeping tests green
- [ ] All tests pass: `cargo t-all`
- [ ] Clippy passes: `cargo clippy -- -D warnings`
- [ ] Formatted: `cargo fmt`
- [ ] Coverage meets module targets (see TDD_GUIDE.md §10)

## Test Coverage

<!-- New or modified tests -->
- `test_<feature>_<behavior>` — Tests that ...

## Related Issues

Closes #...
```

### 11.3 Status Badges

Add to [`README.md`](README.md):

```markdown
# Shambala

![CI](https://github.com/shambala-game/shambala/actions/workflows/ci.yml/badge.svg)
![Coverage](https://codecov.io/gh/shambala-game/shambala/branch/main/graph/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg)
```

---

## 12. Conventional Commits

### 12.1 Commit Message Format

All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### 12.2 Types for Shambala TDD

| Type | When to Use | TDD Phase | Example |
|---|---|---|---|
| `feat` | A new feature or component | 🟢 GREEN | `feat(combat): add damage calculation formula` |
| `fix` | A bug fix | 🟢 GREEN | `fix(health): prevent HP from underflowing below 0` |
| `test` | Adding or modifying tests | 🔴 RED | `test(health): add take_damage edge case tests` |
| `refactor` | Code restructuring without behavior change | 🔵 REFACTOR | `refactor(combat): extract damage formula into helper` |
| `chore` | Build, config, CI changes | Any | `chore(ci): add cargo-llvm-cov to CI pipeline` |
| `docs` | Documentation changes | Any | `docs(tdd): add TDD workflow section` |
| `perf` | Performance improvements | 🔵 REFACTOR | `perf(render): batch sprite draw calls` |
| `style` | Formatting, whitespace | 🔵 REFACTOR | `style: cargo fmt` |

### 12.3 Scope Values

| Scope | Module | Example |
|---|---|---|
| `components` | Any file in `src/components/` | `feat(components): add CorruptionMeter component` |
| `systems` | Any system in `src/systems/` | `feat(systems): implement AISystem patrol behavior` |
| `procgen` | `src/procgen/` | `test(procgen): add path existence property test` |
| `render` | `src/render/` | `perf(render): batch sprites by texture atlas` |
| `ui` | `src/ui/` | `feat(ui): add Data Drain gauge to HUD` |
| `network` | `src/network/` | `fix(network): correct latency simulation offset` |
| `entities` | `src/entities/` | `refactor(entities): use builder pattern for player` |
| `resources` | `src/resources/` | `test(resources): add GamePhase transition tests` |
| `events` | `src/events/` | `feat(events): add DrainResultEvent` |
| `data` | `src/data/` | `feat(data): add enemy stat tables for Delta server` |
| `ci` | `.github/workflows/` | `chore(ci): add property test job` |
| `docs` | `docs/` | `docs: add Data Drain mechanic to GDD` |

### 12.4 TDD Commit Examples

#### 🔴 RED Phase — Adding tests

```
test(health): add take_damage edge case tests

- Test damage with invulnerability active
- Test damage when already at 0 HP
- Test healing beyond max_hp
```

```
test(combat): add property tests for damage invariants

- Damage >= 1 for all attack/defense combinations
- Damage does not exceed theoretical maximum
- Higher attack always produces >= damage
```

#### 🟢 GREEN Phase — Implementing features

```
feat(health): implement take_damage with invulnerability support

- saturating_sub prevents underflow
- Invulnerability guard returns 0 damage
- Returns actual damage dealt for event system
```

```
feat(combat): implement damage calculation formula

Formula: base_damage = attack * skill_multiplier - defense * 0.5
Minimum damage floor of 1
Critical hits multiply final damage by crit_damage multiplier
```

#### 🔵 REFACTOR Phase — Cleaning up

```
refactor(combat): extract damage pipeline into modular helpers

- calculate_base_damage, apply_defense, apply_crit
- Each function is independently testable
- No behavior change — all existing tests pass
```

```
refactor(procgen): use builder pattern for AreaGenerator

- AreaConfig builder replaces long parameter lists
- Default values for optional parameters
- Better documentation on each configuration option
```

### 12.5 Full TDD Cycle Commits

A complete TDD cycle for a feature typically produces 3 commits:

```
# 1. RED: Write the tests first
test(data_drain): add gauge charge and activation tests

# 2. GREEN: Implement the feature
feat(data_drain): implement gauge charging from kills

# 3. REFACTOR: Clean up
refactor(data_drain): extract reward generation into helper
```

Squashing is **not recommended** — each phase's commit tells a story:

```
git log --oneline
abc1234 refactor(data_drain): extract reward generation into helper
abc1223 feat(data_drain): implement gauge charging from kills
abc1213 test(data_drain): add gauge charge and activation tests
```

This makes code review easier: reviewers can see the tests first, then verify the implementation.

### 12.6 Commit Hooks

Install a commit-msg hook to enforce conventional commits:

```bash
# .githooks/commit-msg
#!/bin/bash
# Enforce Conventional Commits format

commit_regex='^(feat|fix|test|refactor|chore|docs|perf|style)(\([a-z_]+\))?: .+'
if ! grep -qE "$commit_regex" "$1"; then
    echo "ERROR: Commit message must follow Conventional Commits format:"
    echo "  <type>(<scope>): <description>"
    echo "  Types: feat, fix, test, refactor, chore, docs, perf, style"
    exit 1
fi
```

Enable the hook:

```bash
git config core.hooksPath .githooks
```

---

## Appendix A: Quick Reference

### A.1 Common Commands

```bash
cargo tdd              # Run unit tests only (fast)
cargo t-all            # Run all tests (unit + integration)
cargo test --test combat_tests  # Run specific integration test file
cargo test test_health          # Run tests matching "test_health"
cargo test -- --ignored         # Run ignored (slow) tests
cargo test --test '*' -- --nocapture  # Show output from passing tests
cargo bench            # Run benchmarks
cargo clippy           # Lint check
cargo fmt              # Format code
cargo coverage         # Generate coverage report
cargo watch -x tdd     # Auto-run tests on file change
```

### A.2 Test Coverage Cheat Sheet

| What to Test | How | Tool |
|---|---|---|
| Component construction and defaults | `assert_eq!` on all fields | `#[test]` |
| Component serialization | Serialize → deserialize → compare | `serde_json`, `ron` |
| System behavior in isolation | Call system function directly | `#[test]` |
| Multiple systems together | Build world, run systems in sequence | `TestApp` |
| Full game scenarios | Spawn entities, simulate input, verify state | `TestApp` |
| Random input invariants | Property-based tests with `proptest!` | `proptest` |
| Performance regression | Compare benchmark results | `criterion` |
| Edge cases from proptest | Save shrunken input as unit test | `#[test_case]` |

### A.3 Common Anti-Patterns

| Anti-Pattern | Why It's Wrong | Fix |
|---|---|---|
| Testing implementation details | Tests break on refactor | Test *behavior*, not internal state |
| Testing without assertions | "It compiles" is not a test | Always assert expected outcomes |
| Shared mutable test state | Tests become order-dependent | Each test gets a fresh `World` |
| Over-mocking | Tests pass but real code fails | Mock at boundaries (GPU, filesystem) only |
| Ignoring slow tests permanently | Tech debt accumulates | Run `#[ignore]` tests in CI weekly |
| Golden-locks value assertions | Brittle, breaks on balance changes | Test ranges and invariants, not exact values |
| Forgetting to test failure paths | Happy-path only coverage | Test invalid inputs, edge cases, errors |

---

> **Document Status:** v1.0
> **Next Steps:** Review with team, set up CI pipeline, begin Milestone 0 test-first implementation.
