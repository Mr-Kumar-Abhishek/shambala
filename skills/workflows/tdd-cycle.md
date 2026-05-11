# Skill: TDD Cycle

## Description
Defines the Test-Driven Development workflow for the Shambala project. This skill covers the step-by-step Red-Green-Refactor cycle, when to write which type of test, commit message conventions, code review checklist, and the Definition of Done.

## Prerequisites
- Familiarity with Rust testing (`cargo test`, `#[test]`)
- Understanding of the project's testing patterns ([`testing-patterns.md`](../game-dev/testing-patterns.md))
- Knowledge of the project's ECS architecture ([`ecs-patterns.md`](../game-dev/ecs-patterns.md))
- Reference [`../docs/TDD_GUIDE.md`](../docs/TDD_GUIDE.md) for the full TDD guide

## Steps

### 1. Red-Green-Refactor Cycle

Follow this cycle for every new feature or bug fix.

#### 🔴 RED — Write a Failing Test

Before writing any implementation code, write a test that expresses the desired behaviour.

```rust
// This test will FAIL because the feature doesn't exist yet
#[test]
fn test_health_take_damage_clamps_to_zero() {
    let mut health = Health::new(100, 50);
    health.take_damage(200); // More damage than remaining HP
    assert_eq!(health.current_hp, 0, "HP should not go below 0");
}
```

**Rules of RED:**
- The test must fail for the *right reason* (missing feature, not a compile error)
- Write the simplest test that captures the requirement
- Test one behaviour per test function
- Use descriptive names: `test_<system>_<behaviour>_<condition>`
- Run `cargo tdd` to confirm the test fails

#### 🟢 GREEN — Write the Minimum Code to Pass

Write the simplest possible implementation to make the test pass.

```rust
impl Health {
    pub fn take_damage(&mut self, amount: u32) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }
}
```

**Rules of GREEN:**
- Write the *minimum* code to pass — no gold-plating
- It is okay to hardcode values temporarily
- Do not optimise yet — that comes in REFACTOR
- Run `cargo tdd` to confirm all tests pass

#### 🔵 REFACTOR — Clean Up While Staying Green

Improve code quality without changing behaviour.

```rust
impl Health {
    /// Reduces current HP by `amount`, clamped to 0.
    pub fn take_damage(&mut self, amount: u32) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    /// Returns true if the entity is alive (HP > 0).
    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}
```

**Rules of REFACTOR:**
- Only change code structure, not behaviour
- Run the full test suite after each refactor step
- Extract duplicated logic into helper functions
- Add documentation comments (`///`)
- Improve naming
- **Never refactor without a green test suite**

### 2. When to Write Which Type of Test

| Test Type | When to Write | Scope | Command |
|---|---|---|---|
| **Unit test** | First (RED phase) | Single function or component method | `cargo tdd` |
| **System test** | After unit tests pass | Single ECS system in isolation | `cargo tdd` |
| **Integration test** | After system tests pass | Multiple systems working together | `cargo t-all` |
| **Property-based test** | When invariants exist | Validates across random inputs | `cargo t-all` |
| **Benchmark** | Before optimisation | Performance baseline and regression check | `cargo bench` |
| **Doc test** | During REFACTOR | Verifies code examples in docs | `cargo t-doc` |

#### Decision Flow

```
New feature or bug fix?
    │
    ├── Pure data logic? → Write unit test (RED)
    │
    ├── ECS system behaviour? → Write system test with TestApp (RED)
    │
    ├── Cross-system interaction? → Write integration test (RED)
    │
    └── Performance-sensitive? → Write benchmark first (baseline)
```

### 3. Commit Message Conventions

Use [Conventional Commits](https://www.conventionalcommits.org/) format. Every commit must follow the TDD cycle.

#### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

#### Types

| Type | Usage | TDD Phase |
|---|---|---|
| `test` | Adding or modifying tests | RED |
| `feat` | New feature implementation | GREEN |
| `fix` | Bug fix | GREEN |
| `refactor` | Code restructuring | REFACTOR |
| `docs` | Documentation changes | Any |
| `perf` | Performance improvements | REFACTOR |
| `chore` | Build/config changes | Any |

#### Examples

```
test(combat): add test for damage clamping to zero

RED phase: test that take_damage saturates at 0
```

```
feat(combat): implement saturating damage subtraction

GREEN phase: minimum code to pass damage clamping test
```

```
refactor(combat): extract damage calculation helper

REFACTOR phase: DRY up damage logic across systems
```

### 4. Code Review Checklist

Before submitting a pull request, verify each item:

#### Test Quality
- [ ] Every new function has at least one unit test
- [ ] Every ECS system has a system-level test
- [ ] Edge cases are covered (zero values, max values, empty collections)
- [ ] Property-based tests exist for critical invariants (damage formulas, state transitions)
- [ ] All tests pass: `cargo t-all`
- [ ] No `#[ignore]` tests without a documented reason

#### Code Quality
- [ ] Code follows project naming conventions (see [`ecs-patterns.md`](../game-dev/ecs-patterns.md))
- [ ] No `unwrap()` or `expect()` in production code (use proper error handling)
- [ ] Public items have doc comments (`///`)
- [ ] No dead code or commented-out code
- [ ] No magic numbers — use named constants

#### Performance
- [ ] No unnecessary allocations in hot paths (frame-by-frame systems)
- [ ] ECS queries use the most specific filter (`With<T>`, `Without<T>`)
- [ ] Benchmarks show no regression compared to baseline

#### Safety
- [ ] No unsafe code without a `// SAFETY:` comment
- [ ] Component data uses saturating arithmetic where appropriate
- [ ] Event readers are drained each frame (no stale events)

### 5. Definition of Done

A feature or bug fix is considered **Done** when all of the following are true:

| Criterion | Verification |
|---|---|
| **Tests pass** | `cargo t-all` exits with code 0 |
| **Tests written** | RED phase tests exist for all new behaviour |
| **Code implemented** | GREEN phase implementation is complete |
| **Code refactored** | REFACTOR phase completed, no technical debt introduced |
| **No regressions** | `cargo bench` shows no performance regression |
| **Documented** | Public API has doc comments; skill files updated if needed |
| **Reviewed** | PR approved by at least one other team member |
| **Committed** | Commit messages follow conventional commits format |
| **Merged** | PR merged into the main branch |

### 6. Sprint 2: Rendering & Game Loop Testing

Sprint 2 introduces the rendering pipeline and game loop, which require specialised testing approaches beyond standard ECS system tests.

#### Rendering Code Requires Special Testing Approaches

GPU-dependent code cannot run in headless CI environments. Use these strategies:

| Approach | When to Use | How |
|---|---|---|
| **Mock GPU (headless device)** | Testing pipeline creation, shader compilation, bind group layout validation | Create a [`MockRenderContext`](../game-dev/testing-patterns.md#testing-wgpu-pipeline-creation-mock-surface) with `wgpu::Instance::new_headless()` — no window or surface required |
| **Pure function extraction** | Testing sprite batching, viewport culling, animation frame calculation | Extract data transformation logic from render systems into pure functions testable without any GPU context |
| **Feature-gated integration tests** | Full render pipeline smoke tests that require a real GPU | Gate with `#[cfg_attr(feature = "headless", ignore)]` — run locally, skip in CI |
| **Snapshot testing** | Verifying visual output structure (vertex counts, batch order, draw call parameters) | Assert on the data structures produced by render systems, not on pixel output |

**RED phase for rendering:**
```rust
// Pure function test — no GPU needed
#[test]
fn test_collect_sprites_filters_by_viewport() {
    let mut world = World::new();
    let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0, ..Default::default() };

    // Sprite inside viewport
    world.spawn((
        Position::new(100.0, 100.0),
        Renderable { texture_id: "a.png".into(), visible: true, size: (32.0, 32.0), ..Default::default() },
        DepthLayer(0),
    ));
    // Sprite outside viewport
    world.spawn((
        Position::new(5000.0, 5000.0),
        Renderable { texture_id: "b.png".into(), visible: true, size: (32.0, 32.0), ..Default::default() },
        DepthLayer(0),
    ));

    let visible = collect_sprites_in_viewport(&world, &camera, 800.0, 600.0);
    assert_eq!(visible.len(), 1, "Only sprites inside the viewport should be collected");
}
```

**GREEN phase:**
```rust
pub fn collect_sprites_in_viewport(
    world: &World,
    camera: &Camera,
    screen_w: f32,
    screen_h: f32,
) -> Vec<SpriteBatch> {
    let all_sprites = collect_sprites(world);
    all_sprites.into_iter().filter(|batch| {
        batch.instances.iter().any(|inst| {
            let left = inst.position[0];
            let right = inst.position[0] + inst.size[0];
            let top = inst.position[1];
            let bottom = inst.position[1] + inst.size[1];
            let view_left = camera.x - screen_w / (2.0 * camera.zoom);
            let view_right = camera.x + screen_w / (2.0 * camera.zoom);
            let view_top = camera.y - screen_h / (2.0 * camera.zoom);
            let view_bottom = camera.y + screen_h / (2.0 * camera.zoom);
            right >= view_left && left <= view_right && bottom >= view_top && top <= view_bottom
        })
    }).collect()
}
```

#### Integration Tests for Game Loop Timing

The game loop drives frame updates at a fixed timestep. Test timing correctness without a real window.

```rust
#[test]
fn test_fixed_timestep_accumulates_correctly() {
    let mut game_loop = GameLoop::new(60.0); // 60 FPS target
    let mut update_count = 0;

    // Simulate a frame with variable real time
    game_loop.begin_frame(0.0);       // First frame at t=0
    game_loop.begin_frame(0.008);     // 8ms real time — less than one tick (16.67ms)
    assert_eq!(game_loop.pending_updates(), 0, "Should not accumulate a full tick yet");

    game_loop.begin_frame(0.025);     // 25ms real time — total 33ms, ~2 ticks
    assert_eq!(game_loop.pending_updates(), 2, "Should accumulate 2 fixed updates");
}

#[test]
fn test_game_loop_spiral_of_death_prevention() {
    let mut game_loop = GameLoop::new(60.0);
    game_loop.set_max_frame_time(0.1); // Cap at 100ms to prevent spiral of death

    // Simulate a very long frame (500ms)
    game_loop.begin_frame(0.5);

    // Should cap at max_frame_time / fixed_delta = 0.1 / 0.01667 ≈ 6 updates
    assert!(
        game_loop.pending_updates() <= 6,
        "Should cap updates to prevent spiral of death"
    );
}

#[test]
fn test_render_interpolation_factor() {
    let mut game_loop = GameLoop::new(60.0);

    game_loop.begin_frame(0.005); // 5ms into a 16.67ms tick
    let alpha = game_loop.interpolation_factor();

    assert!(
        (alpha - 0.3).abs() < 0.05,
        "Interpolation factor should be ~0.3 for 5ms into a 16.67ms tick"
    );
}
```

#### Visual Testing Considerations

Visual correctness cannot be fully automated. Use these complementary approaches:

| Method | Purpose | Tool / Technique |
|---|---|---|
| **Manual visual review** | Verify colours, layout, animation smoothness | Run the game locally and observe |
| **Structure assertions** | Verify draw call count, batch composition, sprite ordering | Unit tests on render data structures |
| **Shader compilation tests** | Verify WGSL shaders compile without errors | Headless device + `device.create_shader_module()` |
| **Performance benchmarks** | Detect FPS regressions, draw call spikes | `cargo bench` with sprite batching benchmarks |
| **Logging-based verification** | Capture render pass structure in logs for manual review | `tracing` or `log` crate with structured events |

**Checklist for rendering PRs:**
- [ ] All pure render logic has unit tests (batching, culling, animation)
- [ ] Pipeline creation tests pass with headless device
- [ ] Shader compilation does not produce errors or warnings
- [ ] Sprite batching benchmark shows no regression
- [ ] Game loop timing tests pass (fixed timestep, interpolation)
- [ ] Manual visual check performed: sprites render at correct positions
- [ ] UI overlay renders on top of game world correctly
- [ ] Camera follow and shake behave as expected
- [ ] No GPU-only tests run in headless CI (feature-gated)

## Examples

### Complete TDD Session: Adding a Status Effect System

#### RED — Write the Test

```rust
#[test]
fn test_poison_effect_deals_tick_damage() {
    let mut app = TestApp::new();
    let player = app.spawn_player(ClassType::TwinBlade, Position::new(0.0, 0.0), default_player_stats());

    // Apply poison status effect
    app.world_mut().get_mut::<StatusEffects>(player).unwrap().effects.push(
        StatusEffectInstance {
            effect_id: "poison".into(),
            effect_type: StatusEffectType::DamageOverTime,
            remaining_duration: 10.0,
            tick_interval: 2.0,
            tick_timer: 0.0,
            magnitude: 5.0,
            source: "enemy_poison".into(),
        }
    );

    app.add_system(status_effect_system);
    app.update(2.0); // One tick interval passes

    let health = app.world().get::<Health>(player).unwrap();
    assert!(health.current_hp < health.max_hp, "Poison should deal damage over time");
}
```

#### GREEN — Implement the System

```rust
pub fn status_effect_system(
    time: Res<Time>,
    mut query: Query<(&mut StatusEffects, &mut Health)>,
) {
    for (mut effects, mut health) in query.iter_mut() {
        effects.effects.retain_mut(|effect| {
            effect.tick_timer += time.delta;

            if effect.tick_timer >= effect.tick_interval {
                effect.tick_timer -= effect.tick_interval;
                match effect.effect_type {
                    StatusEffectType::DamageOverTime => {
                        health.take_damage(effect.magnitude as u32);
                    }
                    StatusEffectType::HealOverTime => {
                        health.heal(effect.magnitude as u32);
                    }
                    _ => {}
                }
            }

            effect.remaining_duration -= time.delta;
            effect.remaining_duration > 0.0
        });
    }
}
```

#### REFACTOR — Clean Up

```rust
impl StatusEffectInstance {
    /// Returns true if the effect is still active.
    fn tick(&mut self, health: &mut Health, dt: f32) -> bool {
        self.tick_timer += dt;
        while self.tick_timer >= self.tick_interval {
            self.tick_timer -= self.tick_interval;
            match self.effect_type {
                StatusEffectType::DamageOverTime => health.take_damage(self.magnitude as u32),
                StatusEffectType::HealOverTime => health.heal(self.magnitude as u32),
                _ => {}
            }
        }
        self.remaining_duration -= dt;
        self.remaining_duration > 0.0
    }
}

pub fn status_effect_system(
    time: Res<Time>,
    mut query: Query<(&mut StatusEffects, &mut Health)>,
) {
    for (mut effects, mut health) in query.iter_mut() {
        effects.effects.retain_mut(|effect| effect.tick(&mut health, time.delta));
    }
}
```

## Related Skills
- [`testing-patterns.md`](../game-dev/testing-patterns.md) — How to write tests for ECS systems and rendering code
- [`rendering-pipeline.md`](../game-dev/rendering-pipeline.md) — Rendering pipeline implementation patterns
- [`ecs-patterns.md`](../game-dev/ecs-patterns.md) — ECS fundamentals for system implementation
- [`git-workflow.md`](git-workflow.md) — Commit message conventions and PR process
- [`../docs/TDD_GUIDE.md`](../docs/TDD_GUIDE.md) — Full TDD guide with detailed examples