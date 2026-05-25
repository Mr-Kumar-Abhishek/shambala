//! # Shambala — A story-driven action RPG
//!
//! Shambala is a top-down 2D action RPG built in Rust with a custom
//! Entity-Component-System (ECS) engine. It uses `wgpu` for GPU-accelerated
//! rendering, `winit` for windowing/input, and `rodio` for audio playback.
//!
//! ## Architecture
//!
//! The engine is organised into five layers:
//!
//! - **`core`** — Fundamental types ([`GameState`], [`Class`], [`Element`]),
//!   constants, and the [`GameStateManager`].
//! - **`components`** — ECS data-only structs ([`Position`], [`Stats`],
//!   [`Renderable`], [`Skill`], etc.).
//! - **`systems`** — Stateless logic that operates on components
//!   ([`CombatSystem`], [`PhysicsSystem`], [`RenderSystem`], etc.).
//! - **`entities`** — Factory functions that bundle components into
//!   ready-to-use entities ([`PlayerEntity`], [`EnemyEntity`], [`AreaEntity`]).
//! - **`resources`** — Shared singletons ([`GameTime`], [`Camera`],
//!   [`InputStateResource`], [`AssetManager`], [`AudioManager`]).
//!
//! Higher-level game screens and managers live in **`game`**
//! ([`TitleScreen`], [`ChaosGate`], [`QuestManager`], etc.),
//! and the **`render`** layer wraps raw `wgpu` pipelines, sprite batching,
//! tilemaps, text rendering, and UI.
//!
//! [`GameState`]: core/types/enum.GameState.html
//! [`Class`]: core/types/enum.Class.html
//! [`Element`]: core/types/enum.Element.html
//! [`GameStateManager`]: core/game_state/struct.GameStateManager.html
//! [`Position`]: components/position/struct.Position.html
//! [`Stats`]: components/stats/struct.Stats.html
//! [`Renderable`]: components/render/struct.Renderable.html
//! [`Skill`]: components/skill/struct.Skill.html

pub mod components;
pub mod systems;
pub mod entities;
pub mod resources;
pub mod core;
pub mod game;
pub mod render;