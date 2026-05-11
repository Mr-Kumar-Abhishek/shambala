# Skill: Git Workflow

## Description
Defines the Git workflow for the Shambala project. This skill covers branch naming conventions, commit message format (Conventional Commits), pull request process, code review guidelines, and the release process.

## Prerequisites
- Git installed and configured
- Access to the project repository
- Understanding of the TDD cycle ([`tdd-cycle.md`](tdd-cycle.md))
- Familiarity with the project's skill files and documentation

## Steps

### 1. Branch Naming Conventions

All work must be done on feature branches. Never commit directly to `main`.

#### Branch Name Format

```
<type>/<short-description>
```

#### Types

| Type | Usage | Branches From |
|---|---|---|
| `feat/` | New features | `main` |
| `fix/` | Bug fixes | `main` |
| `refactor/` | Code restructuring | `main` |
| `docs/` | Documentation changes | `main` |
| `perf/` | Performance improvements | `main` |
| `test/` | Adding or modifying tests | `main` |
| `chore/` | Build/config/tooling | `main` |
| `release/` | Release preparation | `main` |

#### Examples

```
feat/combat-damage-formula
fix/health-underflow-on-zero-damage
refactor/extract-sprite-batching
docs/rendering-pipeline-skill
perf/optimise-ecs-query-filtering
test/status-effect-edge-cases
chore/update-wgpu-to-0.19
release/v0.2.0
```

#### Branch Naming Rules

- Use **kebab-case** (lowercase with hyphens)
- Keep descriptions concise but descriptive (2-5 words)
- Include the feature area prefix when relevant (e.g., `combat-`, `render-`, `ui-`)
- Do not include issue numbers in the branch name (reference them in the PR description)

### 2. Commit Message Format (Conventional Commits)

Every commit must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

#### Structure

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

#### Types

| Type | TDD Phase | Description |
|---|---|---|
| `test` | RED | Adding or modifying tests |
| `feat` | GREEN | New feature implementation |
| `fix` | GREEN | Bug fix |
| `refactor` | REFACTOR | Code restructuring without behaviour change |
| `docs` | Any | Documentation changes |
| `perf` | REFACTOR | Performance improvements |
| `chore` | Any | Build/config/tooling changes |
| `style` | REFACTOR | Formatting, whitespace (no logic change) |

#### Scopes

| Scope | Area |
|---|---|
| `combat` | Combat system, damage, skills |
| `render` | Rendering pipeline, sprites, UI |
| `ecs` | ECS core, components, systems |
| `area` | Area generation, procgen |
| `ui` | UI elements, HUD, menus |
| `audio` | Audio system, SFX, music |
| `physics` | Physics, collision |
| `data` | Data formats, save/load |
| `ci` | CI/CD, build scripts |
| `docs` | Documentation, skill files |

#### Commit Examples

```
test(combat): add test for damage clamping to zero

RED phase: verify take_damage saturates at 0
```

```
feat(combat): implement saturating damage subtraction

GREEN phase: minimum code to pass damage clamping test
```

```
refactor(combat): extract damage calculation helper

REFACTOR phase: DRY up damage logic across systems
```

```
fix(render): correct sprite UV calculation for non-square tiles

Tiles with aspect ratio != 1 were rendering with stretched textures.
```

```
docs(render): add rendering-pipeline skill file

Covers wgpu setup, sprite batching, tilemap rendering, UI layer,
camera system, and animation system.
```

```
perf(ecs): add With<T> filter to combat query

Reduces query iteration from 10k to 500 entities per frame.
```

#### Commit Body Rules

- Use the body to explain **why** the change was made, not **what** was changed
- Reference related issues: `Closes #42`, `Related to #17`
- Mention the TDD phase: `RED phase`, `GREEN phase`, `REFACTOR phase`
- Keep lines under 72 characters

#### Breaking Changes

Add `BREAKING CHANGE:` in the footer or append `!` after the type/scope:

```
feat(ecs)!: migrate from hecs to bevy_ecs

BREAKING CHANGE: All component derives must use `#[derive(Component)]`
instead of `#[derive(Component)]` from hecs.
```

### 3. Pull Request Process

Every feature branch must go through a pull request before merging into `main`.

#### PR Creation Checklist

- [ ] Branch is up to date with `main` (rebased, not merged)
- [ ] All tests pass: `cargo t-all`
- [ ] Benchmarks show no regression: `cargo bench`
- [ ] Code follows project conventions (see [`ecs-patterns.md`](../game-dev/ecs-patterns.md))
- [ ] TDD cycle is complete (RED → GREEN → REFACTOR)
- [ ] Commit messages follow Conventional Commits format
- [ ] PR description is filled out (see template below)

#### PR Template

```markdown
## Description
[Brief description of the changes]

## TDD Cycle
- [ ] RED: Tests written
- [ ] GREEN: Implementation complete
- [ ] REFACTOR: Code cleaned up

## Type of Change
- [ ] feat: New feature
- [ ] fix: Bug fix
- [ ] refactor: Code restructuring
- [ ] docs: Documentation
- [ ] perf: Performance
- [ ] test: Tests
- [ ] chore: Build/config

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Property-based tests added/updated
- [ ] Benchmarks added/updated
- [ ] Manual testing performed

## Checklist
- [ ] `cargo t-all` passes
- [ ] `cargo bench` shows no regression
- [ ] Code follows project conventions
- [ ] Documentation updated (if applicable)
- [ ] No `unwrap()` in production code
- [ ] Public API has doc comments

## Related Issues
Closes #[issue_number]
```

#### PR Lifecycle

```
1. Create branch from main
2. Make changes following TDD cycle
3. Push branch to remote
4. Create PR using template
5. Request review from at least one team member
6. Address review feedback (additional commits)
7. Squash merge into main
8. Delete the feature branch
```

### 4. Code Review Guidelines

#### Reviewer Responsibilities

- Review within **24 hours** of assignment
- Focus on correctness, not style (style is enforced by `rustfmt`)
- Verify the TDD cycle was followed (tests written first)
- Check for edge cases and error handling
- Run the code locally if the change is complex

#### What to Look For

| Category | Check |
|---|---|
| **Correctness** | Does the code do what it claims? |
| **Test coverage** | Are there tests for edge cases? |
| **Performance** | Are there unnecessary allocations in hot paths? |
| **Safety** | Are there unsafe blocks without SAFETY comments? |
| **Conventions** | Does it follow [`ecs-patterns.md`](../game-dev/ecs-patterns.md) naming? |
| **Documentation** | Are public items documented? |
| **TDD compliance** | Was the RED phase committed before GREEN? |

#### Review Comment Format

Use the [Conventional Comments](https://conventionalcomments.org/) format:

```
suggestion(non-blocking): Consider using saturating_sub here

`self.current_hp -= amount` could underflow if amount > current_hp.
Use `self.current_hp = self.current_hp.saturating_sub(amount)` instead.
```

| Label | Meaning |
|---|---|
| `suggestion` | Improvement idea (author may accept or decline) |
| `issue` | Must be addressed before merge |
| `question` | Clarification needed |
| `praise` | Positive feedback |
| `nitpick` | Trivial preference |

#### Approval Rules

- **At least one** approval required for merge
- All `issue` comments must be resolved before merge
- Author may merge after all issues are addressed (no need for re-review on trivial changes)

### 5. Release Process

#### Versioning

Follow [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH
```

| Increment | When |
|---|---|
| MAJOR | Breaking changes to public API or save format |
| MINOR | New features, no breaking changes |
| PATCH | Bug fixes, performance improvements, no new features |

#### Release Branch

```bash
# Create release branch
git checkout -b release/v0.2.0 main

# Update version in Cargo.toml
# Update version in docs/GDD.md header

# Commit the version bump
git commit -m "chore(release): bump version to 0.2.0"

# Push and create PR
git push origin release/v0.2.0
```

#### Release Checklist

- [ ] Version bumped in [`Cargo.toml`](../Cargo.toml)
- [ ] Version bumped in [`docs/GDD.md`](../docs/GDD.md) header
- [ ] `CHANGELOG.md` updated with all changes since last release
- [ ] All tests pass: `cargo t-all`
- [ ] Benchmarks run and recorded: `cargo bench`
- [ ] Release branch PR approved and merged to `main`
- [ ] Git tag created: `git tag v0.2.0 && git push origin v0.2.0`
- [ ] Release notes written (summary of changes, known issues)

#### Changelog Format

```markdown
# Changelog

## [0.2.0] - 2026-06-01

### Added
- Combat damage calculation with defense mitigation (#42)
- Skill execution system with cooldowns (#45)
- Status effect system (poison, burn, freeze) (#48)

### Changed
- Refactored sprite batching to use instanced rendering (#40)
- Optimised ECS query filtering with With<T> (#41)

### Fixed
- Health underflow when damage exceeds current HP (#39)
- Sprite UV calculation for non-square tiles (#43)

### Performance
- Combat system query: 10k → 500 entities per frame (-95%)
- Sprite batching: 500 draw calls → 12 draw calls (-97%)
```

#### Hotfix Process

For critical bugs in production:

```bash
# Create hotfix branch from main
git checkout -b fix/critical-bug-description main

# Fix and follow TDD cycle
# Commit with fix type
git commit -m "fix(area): correct crash on void tile generation"

# Create PR targeting main directly
# After approval, squash merge
# Tag immediately
git tag v0.2.1 && git push origin v0.2.1
```

## Examples

### Complete Feature Lifecycle

```bash
# 1. Create feature branch
git checkout -b feat/status-effect-system main

# 2. RED phase: write tests
git add tests/unit/status_effects.rs
git commit -m "test(combat): add tests for status effect tick system"

# 3. GREEN phase: implement
git add src/systems/status_effects.rs
git commit -m "feat(combat): implement status effect tick system"

# 4. REFACTOR phase: clean up
git add src/systems/status_effects.rs
git commit -m "refactor(combat): extract status effect tick into helper method"

# 5. Push and create PR
git push origin feat/status-effect-system

# 6. After PR approval, squash merge
# (Done via GitHub UI with squash merge option)

# 7. Delete branch
git branch -d feat/status-effect-system
```

### PR Review Example

```markdown
## Reviewer Comments

praise: Clean test-first approach — the RED phase tests clearly
define the expected behaviour.

issue: The `status_effect_system` uses `unwrap()` on line 47.
Please replace with proper error handling or early continue.

suggestion(non-blocking): Consider adding a `max_effects` constant
to prevent unbounded status effect accumulation on entities.
```

## Related Skills
- [`tdd-cycle.md`](tdd-cycle.md) — TDD workflow that drives commit phases
- [`agile-sprint.md`](agile-sprint.md) — Sprint planning and task management
- [`../docs/TDD_GUIDE.md`](../docs/TDD_GUIDE.md) — Full TDD guide with commit conventions