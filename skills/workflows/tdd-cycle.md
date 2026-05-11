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
- [`testing-patterns.md`](../game-dev/testing-patterns.md) — How to write tests for ECS systems
- [`ecs-patterns.md`](../game-dev/ecs-patterns.md) — ECS fundamentals for system implementation
- [`git-workflow.md`](git-workflow.md) — Commit message conventions and PR process
- [`../docs/TDD_GUIDE.md`](../docs/TDD_GUIDE.md) — Full TDD guide with detailed examples