# Shambala

[![CI](https://github.com/your-username/shambala/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/shambala/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![Version](https://img.shields.io/badge/version-0.4.0-blue)
![Tests](https://img.shields.io/badge/tests-307_passing-brightgreen)

> A story-driven action RPG built in Rust, inspired by the .hack//sign universe.

**Shambala** is a top-down 2D action RPG featuring a custom Entity-Component-System (ECS) engine, real-time combat, procedural area generation via a Chaos Gate system, party management, quests, NPC dialogue, and a fully offline single-player experience.

---

## Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Controls](#controls)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Development Status](#development-status)
- [Building for Production](#building-for-production)
- [Testing](#testing)
- [CI / CD](#ci--cd)
- [Dependencies](#dependencies)
- [License](#license)

---

## Features

### Core Gameplay

| Feature | Status |
|---------|--------|
| Title screen with animated menu | ✅ |
| Character selection (4 classes) | ✅ |
| Top-down exploration with camera | ✅ |
| Real-time combat system | ✅ |
| Skill trees per class | ✅ |
| Level-up progression | ✅ |
| Chaos Gate keyword system | ✅ |
| Procedural area generation | ✅ |
| Data Drain mechanic | ✅ |
| Party system with bonds | ✅ |
| Quest system (tutorial quest included) | ✅ |
| NPC dialogue with branching choices | ✅ |
| Options menu (audio/controls/display) | ✅ |
| Save/load to RON files (3 slots) | ✅ |
| Network simulator | ✅ |
| BGM/SFX audio system | ✅ |

### Playable Classes

| Class | Role | Stats |
|-------|------|-------|
| **TwinBlade** | Fast melee DPS | High agility, medium HP |
| **HeavyBlade** | Tank | High HP/defense, slow |
| **LongArm** | Hybrid fighter | Balanced melee + magic |
| **Wavemaster** | Pure mage | High magic, fragile |

### Visual & Audio

- Custom `wgpu` render pipeline with sprite batching
- Tilemap rendering from procedurally generated areas
- Sprite atlas / texture management
- Damage number floating effects
- Screen shake on critical hits
- Damage/attack animations (frame-based)
- BGM switching per game state
- Spatial audio volume falloff

---

## Quick Start

### Prerequisites

- **Rust** 1.85+ (install via [rustup](https://rustup.rs/))
- **Cargo** (included with Rust)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/your-username/shambala.git
cd shambala

# Run in debug mode (fast iteration)
cargo run

# Run in release mode (optimised)
cargo run --release
```

### Linux Dependencies

On Linux, install the following system libraries for graphics, audio and windowing:

```bash
sudo apt-get install -y \
  libvulkan-dev \
  libx11-dev \
  libxrandr-dev \
  libxcursor-dev \
  libxi-dev \
  libxext-dev \
  libasound2-dev \
  libudev-dev \
  pkg-config
```

---

## Controls

| Action | Key |
|--------|-----|
| Move Up | `W` / `↑` |
| Move Down | `S` / `↓` |
| Move Left | `A` / `←` |
| Move Right | `D` / `→` |
| Confirm / Interact | `Enter` / `Space` |
| Cancel / Back | `Escape` |
| Attack | `Z` |
| Skill 1-4 | `1`–`4` |
| Data Drain | `X` |
| Open Menu | `M` |

---

## Architecture

Shambala is built on a **layered ECS architecture** with a fixed-timestep game loop:

```
┌─────────────────────────────────────────────────────┐
│                  GAME LOOP (fixed timestep)          │
│  ┌──────────┐    ┌──────────┐    ┌──────────────┐  │
│  │  INPUT    │───▶│  UPDATE  │───▶│   RENDER     │  │
│  │  PHASE    │    │  PHASE   │    │   PHASE      │  │
│  └──────────┘    └──────────┘    └──────────────┘  │
│       │               │               │            │
│       ▼               ▼               ▼            │
│  ┌──────────┐    ┌──────────┐    ┌──────────────┐  │
│  │  winit   │    │   ECS    │    │   wgpu       │  │
│  │  input   │    │  World   │    │  pipelines   │  │
│  └──────────┘    └──────────┘    └──────────────┘  │
└─────────────────────────────────────────────────────┘
```

### ECS Pattern

- **Components** — pure data structs (`Position`, `Stats`, `Renderable`, `Player`, `Enemy`, etc.)
- **Systems** — stateless logic (`InputSystem`, `PhysicsSystem`, `CombatSystem`, `RenderSystem`, etc.)
- **Entities** — factory functions that bundle components (`PlayerEntity`, `EnemyEntity`, `AreaEntity`)
- **Resources** — shared singletons (`GameTime`, `Camera`, `InputState`, `AssetManager`, `AudioManager`)

### Engine

[`GameEngine`](src/game/engine.rs) owns all resources and orchestrates the update loop. State transitions are managed by [`GameStateManager`](src/core/game_state.rs).

---

## Project Structure

```
shambala/
├── .github/workflows/ci.yml    # GitHub Actions CI
├── src/
│   ├── main.rs                 # Entry point, event loop
│   ├── lib.rs                  # Module declarations
│   ├── core/                   # Types, constants, state machine
│   ├── components/             # ECS component structs
│   ├── systems/                # ECS system implementations
│   ├── entities/               # Entity factory functions
│   ├── resources/              # Shared resources (time, camera, assets)
│   ├── game/                   # Game screens & managers
│   └── render/                 # Rendering pipeline
├── tests/                      # Integration tests
├── benches/                    # Criterion benchmarks
├── docs/                       # Game & technical design docs
└── skills/                     # Developer skill references
```

### Key Modules

| Module | Purpose |
|--------|---------|
| [`src/core/`](src/core/) | [`GameState`](src/core/types.rs), [`constants`](src/core/constants.rs), [`GameStateManager`](src/core/game_state.rs) |
| [`src/components/`](src/components/) | [`Position`](src/components/position.rs), [`Stats`](src/components/stats.rs), [`Skill`](src/components/skill.rs), [`DataDrain`](src/components/data_drain.rs) |
| [`src/systems/`](src/systems/) | [`CombatSystem`](src/systems/combat.rs), [`ExplorationSystem`](src/systems/exploration.rs), [`AISystem`](src/systems/ai.rs) |
| [`src/game/`](src/game/) | [`TitleScreen`](src/game/title_screen.rs), [`ChaosGate`](src/game/chaos_gate.rs), [`QuestManager`](src/game/quest.rs), [`ProgressionSystem`](src/game/progression.rs) |
| [`src/render/`](src/render/) | [`RenderPipeline`](src/render/pipeline.rs), [`SpriteBatch`](src/render/sprite.rs), [`Tilemap`](src/render/tilemap.rs) |
| [`tests/`](tests/) | [`sprint4_tests.rs`](tests/sprint4_tests.rs) — full gameplay integration tests |

---

## Development Status

| Sprint | Focus | Tests | Status |
|--------|-------|-------|--------|
| Sprint 1 | Core Architecture | 125 | ✅ Complete |
| Sprint 2 | Rendering & Game Loop | ~180 | ✅ Complete |
| Sprint 3 | Assets, Audio & Polish | ~255 | ✅ Complete |
| Sprint 4 | Gameplay Integration | **307** | ✅ Complete |

**Current test count: 307** (270 unit + 37 integration)

---

## Building for Production

```bash
# Release build with LTO and optimisations
cargo build --release

# The binary is at:
#   target/release/shambala
```

### Benchmarks

```bash
cargo bench
```

Results are written to `target/criterion/` as HTML reports.

---

## Testing

```bash
# Run all tests
cargo test

# Run only Sprint 4 integration tests
cargo test --test sprint4_tests

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

The project uses:
- **Unit tests** — embedded in each module via `#[cfg(test)]`
- **Integration tests** — in `tests/` (combat flow, sprint scenarios)
- **Property-based tests** — via `proptest` (dev-dependency)
- **Mock objects** — via `mockall` (dev-dependency)
- **Benchmarks** — via `criterion` (dev-dependency)

---

## CI / CD

The project includes a [GitHub Actions workflow](.github/workflows/ci.yml) that runs on every push/PR:

1. **Check** — `cargo check` + `clippy` + `rustfmt` (fast lint gate)
2. **Test** — `cargo test` (full suite)
3. **Build** — `cargo build --release` (produces binary artifact)
4. **Bench** — `cargo bench` (nightly, main/master only)

See [`.github/workflows/ci.yml`](.github/workflows/ci.yml) for configuration.

---

## Dependencies

### Runtime

| Crate | Purpose |
|-------|---------|
| `wgpu` 22.0 | GPU-accelerated rendering |
| `winit` 0.30 | Window creation & event loop |
| `bevy_ecs` / `bevy_app` / `bevy_math` 0.14 | ECS framework |
| `rodio` 0.20 | Audio playback |
| `serde` / `ron` | Save serialization |
| `noise` / `rand` | Procedural generation |
| `image` 0.25 | Texture loading |

### Dev-Only

| Crate | Purpose |
|-------|---------|
| `criterion` 0.5 | Benchmarks |
| `proptest` 1.4 | Property-based testing |
| `mockall` 0.13 | Mock objects |

---

## License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

*Built with ❤️ using Rust, wgpu, and the Bevy ECS ecosystem.*
