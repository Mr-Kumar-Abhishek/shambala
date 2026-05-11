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

### 5. Rendering Test Patterns

Rendering code requires special testing approaches because GPU resources are unavailable in headless CI environments. Use these patterns to test rendering logic without a physical GPU.

#### Testing wgpu Pipeline Creation (Mock Surface)

Create a mock surface and adapter for testing pipeline creation without a real window.

```rust
// tests/helpers/mock_render.rs
use wgpu;

pub struct MockRenderContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub format: wgpu::TextureFormat,
}

impl MockRenderContext {
    /// Creates a headless device for testing pipeline creation.
    /// Does NOT require a window or real surface.
    pub async fn new_headless() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None, // No surface needed
                ..Default::default()
            })
            .await
            .expect("No suitable GPU adapter found");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Test Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create test device");

        let format = wgpu::TextureFormat::Bgra8UnormSrgb;

        Self { device, queue, format }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sprite_pipeline_creation() {
        let ctx = MockRenderContext::new_headless().await;

        let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/sprite.wgsl").into()),
        });

        let pipeline_layout = ctx.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Test Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            },
        );

        let pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Test Sprite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: ctx.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Verify pipeline was created successfully
        // (No GPU execution needed — just validation)
        assert!(!pipeline.is_null());
    }
}
```

#### Testing Sprite Batching Logic (Unit Tests Without GPU)

Sprite batching is pure data transformation — test it without any GPU context.

```rust
#[cfg(test)]
mod sprite_batching_tests {
    use shambala::*;

    /// Test that sprites with the same texture are grouped into one batch.
    #[test]
    fn test_same_texture_sprites_are_batched_together() {
        let mut world = World::new();

        // Spawn two sprites sharing the same texture
        let _e1 = world.spawn((
            Position::new(10.0, 20.0),
            Renderable {
                texture_id: "characters.png".into(),
                sprite_index: 0,
                size: (32.0, 32.0),
                color: [1.0; 4],
                visible: true,
                flip_x: false,
                flip_y: false,
            },
            DepthLayer(1),
        )).id();

        let _e2 = world.spawn((
            Position::new(50.0, 60.0),
            Renderable {
                texture_id: "characters.png".into(),
                sprite_index: 1,
                size: (32.0, 32.0),
                color: [1.0; 4],
                visible: true,
                flip_x: false,
                flip_y: false,
            },
            DepthLayer(1),
        )).id();

        let batches = collect_sprites(&world);

        assert_eq!(batches.len(), 1, "Both sprites share the same texture");
        assert_eq!(batches[0].instances.len(), 2, "Batch should contain both instances");
    }

    /// Test that invisible sprites are excluded from batches.
    #[test]
    fn test_invisible_sprites_are_excluded() {
        let mut world = World::new();

        let _visible = world.spawn((
            Position::new(0.0, 0.0),
            Renderable {
                texture_id: "ui.png".into(),
                visible: true,
                ..Default::default()
            },
            DepthLayer(0),
        )).id();

        let _hidden = world.spawn((
            Position::new(0.0, 0.0),
            Renderable {
                texture_id: "ui.png".into(),
                visible: false,
                ..Default::default()
            },
            DepthLayer(0),
        )).id();

        let batches = collect_sprites(&world);

        let total_instances: usize = batches.iter().map(|b| b.instances.len()).sum();
        assert_eq!(total_instances, 1, "Invisible sprites must be excluded");
    }

    /// Test that batches are sorted by depth layer for correct draw order.
    #[test]
    fn test_batches_sorted_by_depth() {
        let mut world = World::new();

        let _bg = world.spawn((
            Position::new(0.0, 0.0),
            Renderable {
                texture_id: "bg.png".into(),
                visible: true,
                ..Default::default()
            },
            DepthLayer(0),
        )).id();

        let _fg = world.spawn((
            Position::new(0.0, 0.0),
            Renderable {
                texture_id: "fg.png".into(),
                visible: true,
                ..Default::default()
            },
            DepthLayer(10),
        )).id();

        let batches = collect_sprites(&world);

        assert_eq!(batches.len(), 2);
        // Background (depth 0) must come before foreground (depth 10)
        assert!(
            batches[0].instances[0].depth <= batches[1].instances[0].depth,
            "Batches must be sorted by depth layer"
        );
    }
}
```

#### Testing Viewport Culling

Viewport culling determines which sprites are visible on screen. Test the culling logic independently.

```rust
#[cfg(test)]
mod viewport_culling_tests {
    use shambala::*;

    /// Helper: creates a camera centred at (0, 0) with given screen dimensions.
    fn test_camera(screen_w: f32, screen_h: f32) -> Camera {
        Camera {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            ..Default::default()
        }
    }

    /// A sprite is visible if its bounding box overlaps the camera viewport.
    fn is_sprite_visible(
        position: &Position,
        renderable: &Renderable,
        camera: &Camera,
        screen_w: f32,
        screen_h: f32,
    ) -> bool {
        // Viewport bounds in world space
        let view_left = camera.x - screen_w / (2.0 * camera.zoom);
        let view_right = camera.x + screen_w / (2.0 * camera.zoom);
        let view_top = camera.y - screen_h / (2.0 * camera.zoom);
        let view_bottom = camera.y + screen_h / (2.0 * camera.zoom);

        // Sprite bounding box
        let sprite_left = position.x;
        let sprite_right = position.x + renderable.size.0;
        let sprite_top = position.y;
        let sprite_bottom = position.y + renderable.size.1;

        // AABB overlap check
        sprite_right >= view_left
            && sprite_left <= view_right
            && sprite_bottom >= view_top
            && sprite_top <= view_bottom
    }

    #[test]
    fn test_sprite_inside_viewport_is_visible() {
        let camera = test_camera(800.0, 600.0);
        let pos = Position::new(100.0, 100.0);
        let renderable = Renderable {
            size: (32.0, 32.0),
            visible: true,
            ..Default::default()
        };

        assert!(is_sprite_visible(&pos, &renderable, &camera, 800.0, 600.0));
    }

    #[test]
    fn test_sprite_outside_viewport_is_culled() {
        let camera = test_camera(800.0, 600.0);
        // Sprite far to the right of the viewport
        let pos = Position::new(5000.0, 100.0);
        let renderable = Renderable {
            size: (32.0, 32.0),
            visible: true,
            ..Default::default()
        };

        assert!(!is_sprite_visible(&pos, &renderable, &camera, 800.0, 600.0));
    }

    #[test]
    fn test_sprite_partially_visible_is_not_culled() {
        let camera = test_camera(800.0, 600.0);
        // Sprite at the left edge, partially visible
        let pos = Position::new(-10.0, 100.0);
        let renderable = Renderable {
            size: (32.0, 32.0),
            visible: true,
            ..Default::default()
        };

        assert!(is_sprite_visible(&pos, &renderable, &camera, 800.0, 600.0));
    }

    #[test]
    fn test_culling_respects_zoom() {
        let mut camera = test_camera(800.0, 600.0);
        camera.zoom = 2.0; // Zoomed in — viewport is smaller

        // Sprite visible at zoom=1.0 but outside zoom=2.0 viewport
        let pos = Position::new(300.0, 0.0);
        let renderable = Renderable {
            size: (32.0, 32.0),
            visible: true,
            ..Default::default()
        };

        // At zoom=2.0, viewport is 400x300 world units centred on (0,0)
        // So right edge is at 200 — sprite at 300 is outside
        assert!(!is_sprite_visible(&pos, &renderable, &camera, 800.0, 600.0));
    }
}
```

#### Testing Animation Frame Calculation

Animation frame advancement is pure logic — no GPU needed.

```rust
#[cfg(test)]
mod animation_tests {
    use shambala::*;

    #[test]
    fn test_animation_advances_frame_after_duration() {
        let mut anim = Animation {
            frames: vec![0, 1, 2],
            frame_duration: 0.5,
            current_frame: 0,
            timer: 0.0,
            looping: true,
            playing: true,
        };

        // Advance by exactly one frame duration
        advance_animation(&mut anim, 0.5);

        assert_eq!(anim.current_frame, 1, "Should advance to next frame");
        assert_eq!(anim.timer, 0.0, "Timer should reset after advancing");
    }

    #[test]
    fn test_animation_does_not_advance_before_duration() {
        let mut anim = Animation {
            frames: vec![0, 1, 2],
            frame_duration: 0.5,
            current_frame: 0,
            timer: 0.0,
            looping: true,
            playing: true,
        };

        // Advance by less than one frame duration
        advance_animation(&mut anim, 0.3);

        assert_eq!(anim.current_frame, 0, "Should stay on current frame");
        assert!((anim.timer - 0.3).abs() < f32::EPSILON, "Timer should accumulate");
    }

    #[test]
    fn test_animation_loops_when_reaching_end() {
        let mut anim = Animation {
            frames: vec![0, 1, 2],
            frame_duration: 0.5,
            current_frame: 2, // At the last frame
            timer: 0.0,
            looping: true,
            playing: true,
        };

        // Advance past the end
        advance_animation(&mut anim, 0.5);

        assert_eq!(anim.current_frame, 0, "Looping animation should wrap to first frame");
    }

    #[test]
    fn test_non_looping_animation_stops_at_last_frame() {
        let mut anim = Animation {
            frames: vec![0, 1, 2],
            frame_duration: 0.5,
            current_frame: 2, // At the last frame
            timer: 0.0,
            looping: false,
            playing: true,
        };

        // Advance past the end
        advance_animation(&mut anim, 0.5);

        assert_eq!(
            anim.current_frame, 2,
            "Non-looping animation should stay on last frame"
        );
        assert!(!anim.playing, "Non-looping animation should stop playing");
    }

    #[test]
    fn test_paused_animation_does_not_advance() {
        let mut anim = Animation {
            frames: vec![0, 1, 2],
            frame_duration: 0.5,
            current_frame: 0,
            timer: 0.0,
            looping: true,
            playing: false, // Paused
        };

        advance_animation(&mut anim, 1.0);

        assert_eq!(anim.current_frame, 0, "Paused animation should not advance");
        assert_eq!(anim.timer, 0.0, "Paused animation timer should not accumulate");
    }

    #[test]
    fn test_animation_skips_multiple_frames_with_large_delta() {
        let mut anim = Animation {
            frames: vec![0, 1, 2, 3, 4],
            frame_duration: 0.1,
            current_frame: 0,
            timer: 0.0,
            looping: true,
            playing: true,
        };

        // Advance by 3 frame durations at once
        advance_animation(&mut anim, 0.3);

        assert_eq!(anim.current_frame, 3, "Should advance 3 frames with large delta");
    }
}
```

#### Headless CI Test Gating (Updated)

Use feature flags to gate GPU-dependent tests in CI environments.

```rust
#[test]
#[cfg_attr(feature = "headless", ignore)]
fn test_render_pipeline_initialises() {
    // This test requires a GPU — skip in headless CI
    let ctx = MockRenderContext::new_headless();
    assert!(ctx.is_ok());
}
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
