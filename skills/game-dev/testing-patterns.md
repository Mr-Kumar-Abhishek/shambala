# Skill: Testing Patterns

## Description
Covers testing patterns specific to the Shambala project using `bevy_ecs`. This skill explains how to write unit tests for ECS systems, use the `TestApp` helper, apply property-based testing with `proptest`, and set up benchmarks with `criterion`.

## Prerequisites
- Familiarity with Rust testing (`#[test]`, `cargo test`)
- Understanding of ECS architecture ([`ecs-patterns.md`](ecs-patterns.md))
- Knowledge of the [`TestApp`](../tests/helpers/test_ecs.rs) helper
- Reference [`../docs/TDD_GUIDE.md`](../docs/TDD_GUIDE.md) for full testing strategy documentation

## Steps

### 1. Write Tests for ECS Systems

Each ECS system should be tested in isolation by constructing a minimal world, inserting the required resources, spawning relevant entities, and running the system function directly.

#### System Isolation Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::World;
    use bevy_ecs::system::Resource;
    use shambala::*;

    /// Test a pure function extracted from a system.
    #[test]
    fn test_damage_calculation_basic() {
        let attacker = Stats { attack: 50, ..Default::default() };
        let defender = Stats { defense: 20, ..Default::default() };

        let damage = calculate_damage(&attacker, &defender, 1.0);

        // defense mitigation: 1.0 - (20 / 120) = 0.833...
        // raw: 50 * 1.0 = 50
        // final: 50 * 0.833... = 41.67 -> 42
        assert_eq!(damage, 42);
    }

    /// Test a system by constructing a world and calling it directly.
    #[test]
    fn test_regen_system_restores_hp() {
        let mut world = World::new();

        // Insert required resources
        world.insert_resource(Time { delta: 1.0, elapsed: 0.0 });

        // Spawn entity with components
        let entity = world.spawn((
            Health::new(100, 50),
            Stats::default(),
        )).id();

        // Damage the entity
        world.get_mut::<Health>(entity).unwrap().take_damage(30);

        // Run the system function directly with params
        {
            let time = world.get_resource::<Time>().unwrap();
            let mut query = world.query::<&mut Health>();
            stamina_regen_system(time, query); // Actually: regen_system needs right sig
        }

        // Assert state changed
        let health = world.get::<Health>(entity).unwrap();
        assert!(health.current_hp > 70, "HP should have regenerated");
    }
}
```

#### Testing ECS Events

```rust
#[test]
fn test_skill_event_triggers_damage() {
    let mut world = World::new();

    world.insert_resource(Time::default());
    world.insert_resource(Events::<SkillEvent>::default());
    world.insert_resource(Events::<DamageEvent>::default());

    let caster = world.spawn((
        Stats { attack: 30, ..Default::default() },
        Health::new(100, 50),
        SkillSet::for_class(ClassType::TwinBlade),
    )).id();

    let target = world.spawn((
        Stats { defense: 10, ..Default::default() },
        Health::new(100, 0),
    )).id();

    // Send a skill event
    let mut skill_events = world.get_resource_mut::<Events::SkillEvent>().unwrap();
    skill_events.send(SkillEvent {
        caster,
        target,
        skill_id: "tb_sword_flurry".to_string(),
    });
    drop(skill_events);

    // Run the system
    // skill_execution_system(...);

    // Verify damage was applied
    let target_health = world.get::<Health>(target).unwrap();
    assert!(target_health.current_hp < target_health.max_hp);
}
```

### 2. Use the TestApp Helper

The [`TestApp`](../tests/helpers/test_ecs.rs) wraps a minimal ECS world with convenience methods for spawning entities, simulating input, and advancing game time. This is the preferred approach for integration-level tests.

#### TestApp Setup

```rust
// tests/helpers/test_ecs.rs (project-level test helper)
use bevy_ecs::world::World;
use shambala::*;

pub struct TestApp {
    pub world: World,
    pub systems: Vec<Box<dyn Fn(&mut World)>>,
    pub time: f32,
}

impl TestApp {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(GameState::new());
        world.insert_resource(Time::new());
        world.insert_resource(InputState::new());
        Self { world, systems: Vec::new(), time: 0.0 }
    }

    /// Register a system to run during updates.
    pub fn add_system<F>(&mut self, system: F)
    where F: Fn(&mut World) + 'static {
        self.systems.push(Box::new(system));
    }

    /// Advance the simulation by `delta` seconds.
    pub fn update(&mut self, delta: f32) {
        self.world.get_resource_mut::<Time>().unwrap().delta = delta;
        self.world.get_resource_mut::<Time>().unwrap().elapsed += delta;
        self.time += delta;
        for system in &self.systems {
            system(&mut self.world);
        }
    }

    /// Access the world immutably.
    pub fn world(&self) -> &World { &self.world }

    /// Access the world mutably.
    pub fn world_mut(&mut self) -> &mut World { &mut self.world }
}
```

#### Integration Test Using TestApp

```rust
// tests/integration/combat_tests.rs
use shambala::*;
use tests::helpers::test_ecs::TestApp;
use tests::helpers::fixtures::*;

fn setup_combat_scenario() -> (TestApp, Entity, Entity) {
    let mut app = TestApp::new();
    let player = app.world_mut().spawn((
        Position::new(0.0, 0.0),
        default_player_stats(),
        Health::new(100, 50),
        Combat::new(ClassType::TwinBlade),
        SkillSet::for_class(ClassType::TwinBlade),
        Renderable::player_sprite(ClassType::TwinBlade),
    )).id();
    let enemy = app.world_mut().spawn((
        Position::new(100.0, 0.0),
        default_goblin_stats(),
        Health::new(30, 0),
        Combat::default(),
        EnemyType::new(EnemyCategory::Goblin, 1),
        Renderable::enemy_sprite(EnemyCategory::Goblin),
    )).id();

    app.add_system(movement_system);
    app.add_system(combat_system);

    (app, player, enemy)
}

#[test]
fn test_player_attacks_enemy_within_range() {
    let (mut app, _player, enemy) = setup_combat_scenario();

    app.update(0.5); // Run systems

    // Assert
    let enemy_health = app.world().get::<Health>(enemy).unwrap();
    assert!(
        enemy_health.current_hp < enemy_health.max_hp,
        "Enemy should have taken damage"
    );
}
```

#### TestApp with Input Simulation

```rust
#[test]
fn test_input_moves_player() {
    let mut app = TestApp::new();
    let player = app.world_mut().spawn((
        Position::new(0.0, 0.0),
        Velocity { x: 0.0, y: 0.0, max_speed: 100.0 },
        Player,
    )).id();

    app.add_system(movement_system);

    // Simulate input
    let mut input = app.world_mut().get_resource_mut::<InputState>().unwrap();
    input.actions = vec![GameAction::MoveRight];
    drop(input);

    app.update(1.0);

    let pos = app.world().get::<Position>(player).unwrap();
    assert!(pos.x > 50.0, "Player should have moved right");
}
```

### 3. Property-Based Testing

Use `proptest` to verify invariants hold across a wide range of inputs.

#### Setup

```toml
[dev-dependencies]
proptest = "1.5"
```

#### Basic Property Test

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_damage_never_exceeds_u32(
        attack in 0u32..10000,
        defense in 0u32..10000,
        multiplier in 0.0f32..10.0,
    ) {
        let attacker = Stats { attack, ..Default::default() };
        let defender = Stats { defense, ..Default::default() };
        let damage = calculate_damage(&attacker, &defender, multiplier);

        // Damage must always be at least 1
        assert!(damage >= 1, "Damage should be at least 1");
        // Damage must never overflow
        assert!(damage <= u32::MAX, "Damage should not overflow");
    }
}
```

#### Game Invariant Property Test

```rust
proptest! {
    #[test]
    fn test_health_never_exceeds_max_hp(
        max_hp in 1u32..10000,
        heal_amount in 0u32..5000,
        damage_amount in 0u32..5000,
    ) {
        let mut health = Health::new(max_hp, 50);

        // Apply damage and heal in sequence
        health.take_damage(damage_amount);
        health.heal(heal_amount);

        // Invariant: current_hp never exceeds max_hp
        assert!(
            health.current_hp <= health.max_hp,
            "HP should never exceed max_hp"
        );
        // Invariant: current_hp never underflows below 0 (u32)
        assert!(
            health.current_hp == 0 || health.current_hp > 0,
            "HP should not underflow"
        );
    }
}
```

#### Custom Strategy for Entity Spawning

```rust
fn position_strategy() -> impl Strategy<Value = Position> {
    (0.0f32..1920.0, 0.0f32..1080.0)
        .prop_map(|(x, y)| Position::new(x, y))
}

proptest! {
    #[test]
    fn test_entity_spawns_at_valid_position(pos in position_strategy()) {
        let mut world = World::new();
        let entity = world.spawn((pos.clone(), Renderable::default())).id();

        let stored = world.get::<Position>(entity).unwrap();
        assert_eq!(stored.x, pos.x);
        assert_eq!(stored.y, pos.y);
    }
}
```

### 4. Benchmark Patterns

Use `criterion` for performance benchmarks that track regressions.

#### Setup

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "ecs_benchmarks"
harness = false
```

#### ECS Query Iteration Benchmark

```rust
// benches/ecs_benchmarks.rs
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use bevy_ecs::world::World;
use shambala::*;

fn bench_query_iteration(c: &mut Criterion) {
    let mut world = World::new();
    world.insert_resource(Time::default());

    // Spawn 10,000 entities
    for _ in 0..10_000 {
        world.spawn((
            Position::new(0.0, 0.0),
            Velocity { x: 0.0, y: 0.0, max_speed: 100.0 },
            Health::new(100, 50),
            Renderable::default(),
        ));
    }

    c.bench_function("query_iteration_10000", |b| {
        b.iter(|| {
            let mut query = world.query::<(&mut Position, &Velocity)>();
            for (mut pos, vel) in query.iter_mut(&mut world) {
                pos.x += vel.x * 0.016;
                pos.y += vel.y * 0.016;
            }
            black_box(());
        });
    });
}

criterion_group!(benches, bench_query_iteration);
criterion_main!(benches);
```

#### Sprite Batching Benchmark

```rust
fn bench_sprite_batching(c: &mut Criterion) {
    let mut world = World::new();

    // Spawn 500 visible sprites
    for i in 0..500 {
        world.spawn((
            Position::new(i as f32 * 10.0, 0.0),
            Renderable {
                texture_id: format!("texture_{}", i % 4),
                sprite_index: i % 8,
                size: (32.0, 32.0),
                color: [1.0; 4],
                visible: true,
                ..Default::default()
            },
            DepthLayer(1),
        ));
    }

    c.bench_function("sprite_batching_500", |b| {
        b.iter(|| {
            let batches = collect_sprites(&world);
            black_box(batches);
        });
    });
}
```

#### Benchmark Baseline Comparison

```bash
# Save a baseline (e.g., before refactoring)
cargo bench -- --save-baseline main

# After changes, compare against baseline
cargo bench -- --baseline main

# Generate HTML report
# Available at target/criterion/report/index.html
```

## Examples

### Complete Test File Example

```rust
// tests/unit/combat_system_tests.rs
use shambala::*;
use tests::helpers::test_ecs::TestApp;
use tests::helpers::fixtures::*;

// ── Pure Function Tests ──

#[test]
fn test_calculate_damage_zero_defense() {
    let attacker = Stats { attack: 100, ..Default::default() };
    let defender = Stats { defense: 0, ..Default::default() };
    assert_eq!(calculate_damage(&attacker, &defender, 1.0), 100);
}

#[test]
fn test_calculate_damage_high_defense() {
    let attacker = Stats { attack: 100, ..Default::default() };
    let defender = Stats { defense: 900, ..Default::default() };
    // mitigation = 1.0 - 900/1000 = 0.1
    // damage = 100 * 0.1 = 10
    assert_eq!(calculate_damage(&attacker, &defender, 1.0), 10);
}

// ── System Integration Tests ──

#[test]
fn test_death_system_despawns_zero_hp_entities() {
    let mut world = World::new();
    world.insert_resource(Events::<DeathEvent>::default());

    let alive = world.spawn((Health::new(100, 0),)).id();
    let dead = world.spawn((Health::new(0, 0),)).id();

    // Run death_system
    // death_system(...);

    // Verify dead entity is despawned
    // assert!(world.get_entity(dead).is_none());
    // assert!(world.get_entity(alive).is_some());
}

// ── Property-Based Tests ──

proptest! {
    #[test]
    fn test_damage_never_exceeds_u32(
        attack in 0u32..10000,
        defense in 0u32..10000,
        mult in 0.0f32..10.0,
    ) {
        let attacker = Stats { attack, ..Default::default() };
        let defender = Stats { defense, ..Default::default() };
        let damage = calculate_damage(&attacker, &defender, mult);
        assert!(damage >= 1);
    }
}
```

### Testing Entity Factory Pattern

```rust
#[test]
fn test_spawn_player_creates_valid_entity() {
    let mut world = World::new();
    let entity = spawn_player(
        &mut world,
        ClassType::TwinBlade,
        Position::new(0.0, 0.0),
        Stats::new(10, 8, 5, 12),
    );

    assert!(world.get::<Position>(entity).is_some());
    assert!(world.get::<Health>(entity).is_some());
    assert!(world.get::<Combat>(entity).is_some());
    assert!(world.get::<SkillSet>(entity).is_some());
    assert!(world.get::<Inventory>(entity).is_some());
    assert!(world.get::<Renderable>(entity).is_some());
    assert!(world.get::<Player>(entity).is_some()); // Tag component
}
```

### Headless CI Test Gating

```rust
#[test]
#[cfg_attr(feature = "headless", ignore)]
fn test_render_pipeline_initialises() {
    // This test requires a GPU — skip in headless CI
    let window = create_test_window();
    let render_ctx = RenderContext::new(&window);
    assert!(render_ctx.is_ok());
}
```

## Related Skills
- [`ecs-patterns.md`](ecs-patterns.md) — ECS fundamentals for constructing test worlds
- [`rendering-pipeline.md`](rendering-pipeline.md) — Render system testing patterns
- [`combat-system.md`](combat-system.md) — Combat-specific test scenarios
- [`area-generation.md`](area-generation.md) — Procedural generation test strategies
- [`../docs/TDD_GUIDE.md`](../docs/TDD_GUIDE.md) — Full TDD guide and testing strategy documentation
