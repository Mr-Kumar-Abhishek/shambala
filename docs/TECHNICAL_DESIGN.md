# Shambala — Technical Design Document

> **Version:** 1.0  
> **Status:** Draft  
> **Last Updated:** 2026-05-11  
> **Engine:** Custom (Rust, ECS, 2D Top-Down, wgpu)  
> **Platform:** PC (Windows / Linux / macOS)

---

## Sprint Status

| Sprint | Focus | Tests | Build | Status |
|--------|-------|-------|-------|--------|
| Sprint 1 | Core Architecture | 125 | Release binary (1.2 MB) | ✅ Complete |
| Sprint 2 | Rendering & Game Loop | 180 | Windowed app (wgpu/winit) | ✅ Complete |
| Sprint 3 | Assets, Audio & Polish | Target: 220+ | Full game features | 🔄 In Progress |

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [ECS Architecture](#2-ecs-architecture)
3. [Rendering Pipeline](#3-rendering-pipeline)
4. [Physics & Collision](#4-physics--collision)
5. [Data-Driven Design](#5-data-driven-design)
6. [Module Structure](#6-module-structure)
7. [State Management](#7-state-management)
8. [Networking Layer](#8-networking-layer)
9. [Testing Strategy](#9-testing-strategy)
10. [Performance Targets](#10-performance-targets)
11. [Dependencies](#11-dependencies)

---

## 1. Architecture Overview

### 1.1 High-Level System Architecture

Shambala follows a **layered architecture** built on an ECS (Entity-Component-System) core. The engine is structured as a pipeline where data flows from input through simulation to output.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         GAME LOOP (fixed timestep)                   │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────────┐  │
│  │  INPUT    │───▶│  UPDATE  │───▶│  RENDER  │───▶│   PRESENT    │  │
│  │  PHASE    │    │  PHASE   │    │  PHASE   │    │   PHASE      │  │
│  └──────────┘    └──────────┘    └──────────┘    └──────────────┘  │
│       │               │               │               │            │
│       ▼               ▼               ▼               ▼            │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────────┐  │
│  │  winit   │    │   ECS    │    │  wgpu    │    │  Swap Chain  │  │
│  │  gilrs   │    │  World   │    │  Passes  │    │              │  │
│  └──────────┘    └──────────┘    └──────────┘    └──────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
         │                │                    │
         ▼                ▼                    ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────────────┐
│  Event Bus      │ │  Asset Manager  │ │  Audio Manager (kira)   │
│  (Intra-process)│ │  (Texture, Map, │ │  (Music, SFX, spatial)  │
│                 │ │   Font, Config) │ │                         │
└─────────────────┘ └─────────────────┘ └─────────────────────────┘
```

### 1.2 Crate Dependency Graph

```
┌─────────────────────────────────────────────────────────────────┐
│                        shambala (binary)                         │
│                         src/main.rs                              │
└───────────────────────────┬─────────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────────┐
│                     shambala_core (library)                       │
│                        src/lib.rs                                 │
│                                                                   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐ │
│  │components/ │  │ systems/   │  │ entities/  │  │resources/  │ │
│  │  mod.rs    │  │  mod.rs    │  │  mod.rs    │  │  mod.rs    │ │
│  │  position  │  │  render    │  │  player    │  │  game_state│ │
│  │  stats     │  │  physics   │  │  enemy     │  │  input     │ │
│  │  health    │  │  combat    │  │  companion │  │  camera    │ │
│  │  ...       │  │  ...       │  │  ...       │  │  ...       │ │
│  └────────────┘  └────────────┘  └────────────┘  └────────────┘ │
└───────────────────────────┬─────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────────┐
        │                   │                       │
        ▼                   ▼                       ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────────────┐
│  wgpu         │  │  rapier2d     │  │  kira / rodio         │
│  (Graphics)   │  │  (Physics)    │  │  (Audio)              │
│  + wgpu-glyph │  │  + parry      │  │                       │
│  + image      │  │               │  │                       │
└───────────────┘  └───────────────┘  └───────────────────────┘
        │                   │                       │
        ▼                   ▼                       ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────────────┐
│  winit         │  │  serde        │  │  tokio                │
│  (Window/Input)│  │  + ron + toml │  │  (Async runtime)      │
│  + gilrs       │  │  (Data)       │  │  + bytes              │
└───────────────┘  └───────────────┘  └───────────────────────┘
        │
        ▼
┌───────────────┐
│  bevy_ecs     │
│  (or hecs)    │
│  (ECS Core)   │
└───────────────┘
```

### 1.3 Data Flow Pipeline

```
[User Input] ──▶ [InputSystem] ──▶ [Action Queue] ──▶ [ECS Commands]
                                                           │
              ┌────────────────────────────────────────────┤
              │                    │                       │
              ▼                    ▼                       ▼
     [PhysicsSystem]      [CombatSystem]           [AISystem]
              │                    │                       │
              ▼                    ▼                       ▼
     [PartySystem]       [DataDrainSystem]      [AreaGenSystem]
              │                    │                       │
              └────────────────────┼───────────────────────┘
                                   │
                                   ▼
                           [ECS World State]
                                   │
                                   ▼
                      [RenderSystem / UISystem]
                                   │
                                   ▼
                              [wgpu Frame]
                                   │
                                   ▼
                              [Swap Chain]
```

---

## 2. ECS Architecture

Shambala uses [`bevy_ecs`](https://crates.io/crates/bevy_ecs) (extracted from the Bevy engine) as its core ECS framework. This provides a proven, high-performance, data-oriented architecture without pulling in Bevy's full renderer or scheduler.

The ECS core consists of three fundamental types:

| Type | Purpose | Lifetime |
|---|---|---|
| **Component** | Plain data structs attached to entities | Per-entity |
| **System** | Functions that operate on component data | Per-frame execution |
| **Resource** | Singleton data accessible from any system | Global |

### 2.1 Components

#### 2.1.1 Transform & Spatial Components

```rust
// components/position.rs

/// World-space position in 2D
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Velocity vector for movement
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub max_speed: f32,
}

/// Axis-aligned bounding box for collision
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct AABB {
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

/// Rendering Z-order (higher = closer to camera)
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct DepthLayer(pub u8);

/// Facing direction for sprite flipping
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum Facing {
    Up, Down, Left, Right,
    UpLeft, UpRight, DownLeft, DownRight,
}
```

#### 2.1.2 Renderable Component

```rust
// components/render.rs

/// A renderable sprite entity
#[derive(Component, Clone, Debug)]
pub struct Renderable {
    pub texture_id: String,
    pub sprite_index: usize,        // Index into spritesheet
    pub size: (f32, f32),           // Source rect size in pixels
    pub color: [f32; 4],            // RGBA tint
    pub flip_x: bool,
    pub flip_y: bool,
    pub blend_mode: BlendMode,
    pub visible: bool,
}

/// Animation state for spritesheet animations
#[derive(Component, Clone, Debug)]
pub struct Animation {
    pub frames: Vec<usize>,          // Frame indices into spritesheet
    pub frame_duration: f32,         // Seconds per frame
    pub current_frame: usize,
    pub timer: f32,
    pub looping: bool,
    pub playing: bool,
}

#[derive(Clone, Debug)]
pub enum BlendMode {
    Normal,
    Additive,
    Multiply,
}

/// Particle emitter attached to an entity
#[derive(Component, Clone, Debug)]
pub struct ParticleEmitter {
    pub particle_texture: String,
    pub spawn_rate: f32,         // Particles per second
    pub lifetime: f32,           // Seconds
    pub speed_range: (f32, f32),
    pub color_start: [f32; 4],
    pub color_end: [f32; 4],
    pub size_start: f32,
    pub size_end: f32,
    pub active: bool,
}
```

#### 2.1.3 Stats & Combat Components

```rust
// components/stats.rs

/// Core character statistics
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub level: u32,
    pub experience: u64,
    pub experience_to_next: u64,

    // Primary attributes
    pub strength: u32,
    pub vitality: u32,
    pub intelligence: u32,
    pub agility: u32,

    // Derived attributes (auto-calculated)
    pub attack: u32,
    pub defense: u32,
    pub magic_attack: u32,
    pub magic_defense: u32,
    pub speed: u32,
    pub crit_rate: f32,     // 0.0 - 1.0
    pub crit_damage: f32,   // Multiplier

    // Stat points available to allocate
    pub unspent_points: u32,
}

/// Health and Mana (SP) with regen
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    pub current_hp: u32,
    pub max_hp: u32,
    pub current_sp: u32,
    pub max_sp: u32,
    pub hp_regen: f32,      // Per second
    pub sp_regen: f32,      // Per second
    pub invulnerable: bool,
    pub invuln_timer: f32,
}

/// Combat-specific data
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Combat {
    pub base_damage: u32,
    pub attack_range: f32,
    pub attack_speed: f32,       // Attacks per second
    pub attack_timer: f32,
    pub combo_count: u32,
    pub combo_window: f32,       // Time window for combo chaining
    pub last_hit_time: f32,
}

/// Equipment slots and stat modifiers
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: Option<ItemData>,
    pub armor: Option<ItemData>,
    pub accessory_1: Option<ItemData>,
    pub accessory_2: Option<ItemData>,

    // Cached stat modifiers from equipped items
    pub stat_modifiers: StatsModifiers,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StatsModifiers {
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub magic_attack_bonus: i32,
    pub magic_defense_bonus: i32,
    pub speed_bonus: i32,
    pub hp_bonus: i32,
    pub sp_bonus: i32,
}

/// Inventory — a fixed-size container
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: Vec<Option<ItemData>>,
    pub max_slots: usize,
    pub gold: u64,
}

/// A single item instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemData {
    pub id: String,
    pub item_type: ItemType,
    pub quantity: u32,
    pub max_stack: u32,
    pub rarity: ItemRarity,
    pub stats: Option<StatsModifiers>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    KeyItem,
    Material,
    VirusCore,
    MemoryFragment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}
```

#### 2.1.4 Skill Component

```rust
// components/skill.rs

/// Skill definitions and cooldown tracking
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct SkillSet {
    pub skills: [Option<Skill>; 6],   // Hotbar slots
}

/// A single skill instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub skill_type: SkillType,
    pub damage_multiplier: f32,
    pub sp_cost: u32,
    pub cooldown: f32,             // Seconds
    pub current_cooldown: f32,
    pub cast_time: f32,            // Seconds
    pub range: f32,
    pub area_of_effect: f32,       // 0.0 = single target
    pub status_effect: Option<StatusEffectData>,
    pub animation_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SkillType {
    Melee,
    Ranged,
    Magic,
    Heal,
    Buff,
    Debuff,
    Utility,
}
```

#### 2.1.5 Status Effect Component

```rust
// components/status.rs

/// Active status effects on an entity
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct StatusEffects {
    pub effects: Vec<StatusEffectInstance>,
}

/// A single active status effect instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusEffectInstance {
    pub effect_id: String,
    pub effect_type: StatusEffectType,
    pub remaining_duration: f32,
    pub tick_interval: f32,
    pub tick_timer: f32,
    pub magnitude: f32,            // e.g., damage per tick, slow percentage
    pub source: String,            // Effect source for stacking rules
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusEffectType {
    Poison,
    Burn,
    Freeze,
    Stun,
    Slow,
    Haste,
    Regen,
    Barrier,
    AtkUp,
    DefUp,
    AtkDown,
    DefDown,
    DataOverflow,      // Data Drain failure debuff
    Corruption,        // Zone corruption
    Invincible,
}
```

#### 2.1.6 Party & Bond Components

```rust
// components/party.rs

/// Party membership marker
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct PartyMember {
    pub party_id: u32,
    pub member_index: u32,         // 0 = leader (player), 1-2 = companions
    pub bond_level: u32,           // 1-10
    pub bond_experience: u32,
    pub bond_threshold: u32,       // XP needed for next level
    pub tactics: PartyTactic,
}

/// Companion AI behavior
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct CompanionAI {
    pub companion_id: String,
    pub personality: CompanionPersonality,
    pub unlocked_skills: Vec<String>,
    pub unique_skill: Option<String>,
    pub side_quest_active: bool,
    pub dialogue_flags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PartyTactic {
    Aggressive,
    Defensive,
    Balanced,
    FocusTarget,
    Scatter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CompanionPersonality {
    Enthusiast,     // Sora — chatty, optimistic
    Veteran,        // Kite — serious, protective
    Seeker,         // Mia — quiet, perceptive
    Guardian,       // Balder — wise, philosophical
}
```

#### 2.1.7 Data Drain Component

```rust
// components/data_drain.rs

/// Data Drain capability
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct DataDrain {
    pub gauge: f32,                // 0.0 - 1.0
    pub max_gauge: f32,
    pub charge_rate: f32,          // Per enemy defeated
    pub active: bool,
    pub vulnerable: bool,
    pub vulnerable_timer: f32,
    pub minigame_active: bool,
    pub minigame_phase: DrainPhase,
    pub minigame_score: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DrainPhase {
    Idle,
    Charging,
    Targeting,
    Minigame,
    Draining,
    Complete,
    Failed,
}

/// Corruption meter on bosses/enemies (separate from HP)
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct CorruptionMeter {
    pub current: f32,
    pub max: f32,
    pub drain_resistance: f32,
    pub corrupted: bool,
}
```

#### 2.1.8 AI State Component

```rust
// components/ai.rs

/// Behavior state for AI-controlled entities
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct AIState {
    pub behavior: AIBehavior,
    pub alert_radius: f32,
    pub attack_radius: f32,
    pub patrol_path: Vec<(f32, f32)>,
    pub patrol_index: usize,
    pub wait_timer: f32,
    pub aggro_target: Option<u32>,    // Entity ID
    pub last_known_position: Option<(f32, f32)>,
    pub state_timer: f32,
}

/// Enemy type tag for AI dispatch
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct EnemyType {
    pub enemy_id: String,
    pub category: EnemyCategory,
    pub threat_level: u32,
    pub drops: Vec<DropTableEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AIBehavior {
    Idle,
    Patrol,
    Investigate,
    Chase,
    Attack,
    Retreat,
    Flee,
    Guard,
    Berserk,
    Corrupted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnemyCategory {
    Goblin,
    Skeleton,
    Golem,
    Wisp,
    CorruptedPlayer,
    Boss,
    MidBoss,
}
```

#### 2.1.9 Area & Environment Components

```rust
// components/area.rs

/// Area keyword configuration for procedural generation
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct AreaData {
    pub terrain: TerrainType,
    pub difficulty: DifficultyLevel,
    pub weather: WeatherType,
    pub modifier: AreaModifier,
    pub seed: u64,
    pub zone_index: u32,
    pub is_boss_room: bool,
    pub is_completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TerrainType {
    Forest,
    Desert,
    Ice,
    Volcano,
    Ruins,
    Void,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Easy,
    Normal,
    Hard,
    Hell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Rain,
    Storm,
    Fog,
    Eclipse,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AreaModifier {
    None,
    Chaos,
    Mirror,
    TimeLost,
    Cursed,
}

/// Tilemap component for rendering
#[derive(Component, Clone, Debug)]
pub struct Tilemap {
    pub texture_id: String,
    pub tileset_columns: u32,
    pub tileset_rows: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub map_width: u32,
    pub map_height: u32,
    pub tiles: Vec<u32>,            // Tile indices, row-major
    pub collision_tiles: Vec<bool>,  // Per-tile collision flag
}

/// Map marker for points of interest
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct MapMarker {
    pub marker_type: MarkerType,
    pub label: String,
    pub discovered: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MarkerType {
    Entrance,
    Exit,
    Chest,
    LoreStone,
    SavePoint,
    BossDoor,
    HiddenPath,
}
```

### 2.2 Systems

Systems are executed in ordered stages. Shambala uses `bevy_ecs` stages to ensure deterministic execution order.

#### 2.2.1 System Execution Order

```
Stage 0:  InputSystem        — Read raw input, dispatch actions
Stage 1:  AISystem           — Update AI behaviors, set velocities
Stage 2:  PhysicsSystem      — Apply velocity, resolve collisions
Stage 3:  CombatSystem       — Process damage, healing, skill effects
Stage 4:  DataDrainSystem    — Update gauge, process minigame results
Stage 5:  PartySystem        — Update bond levels, companion state
Stage 6:  AreaGenSystem      — Manage area transitions, LOD
Stage 7:  AnimationSystem    — Advance sprite animations
Stage 8:  AudioSystem        — Trigger music/SFX, update spatial audio
Stage 9:  RenderSystem       — Collect renderables, draw to screen
Stage 10: UISystem           — Draw HUD, menus, overlays
```

#### 2.2.2 System Descriptions

```rust
// systems/input.rs
/// Reads from winit (keyboard/mouse) and gilrs (gamepad).
/// Converts raw inputs into a buffered ActionQueue resource.
/// Supports key rebinding via a Bindings resource.
pub struct InputSystem {
    // Queries: none (reads resources)
    // Resources: InputState (mut), Bindings
}

// systems/physics.rs
/// Applies Velocity to Position. Resolves collisions via rapier2d.
/// Handles spatial queries (detection, overlap checks).
/// Queries: (Position, Velocity, AABB), optionally Acceleration
/// Resources: CollisionWorld, Time
pub struct PhysicsSystem;

// systems/combat.rs
/// Processes Attack, Skill, and Damage events.
/// Calculates damage using Stats, applies to Health.
/// Spawns status effects, updates combo chains.
/// Queries: (Stats, Health, Combat, Equipment, StatusEffects)
/// Events: AttackEvent, SkillEvent, DamageEvent, DeathEvent
pub struct CombatSystem;

// systems/ai.rs
/// Behavior tree executor for enemies and companions.
/// Evaluates conditions, selects actions, updates AIState.
/// Queries: (AIState, Position, Health, Stats, EnemyType or CompanionAI)
/// Resources: GameState, Time
pub struct AISystem;

// systems/party.rs
/// Tracks party composition, updates bond XP.
/// Handles companion swapping and tactics changes.
/// Queries: (PartyMember, CompanionAI, Position)
/// Resources: PartyState, Time
pub struct PartySystem;

// systems/data_drain.rs
/// Manages Data Drain gauge charging and activation.
/// Executes the drain minigame and resolves results.
/// Queries: (DataDrain, CorruptionMeter, Position)
/// Events: DataDrainEvent, DrainResultEvent
pub struct DataDrainSystem;

// systems/area_generation.rs
/// Handles procedural area generation on zone entry.
/// Builds tilemaps, spawns enemies, places items.
/// Queries: AreaData (on area root entity)
/// Resources: AreaGenerator, AssetManager, RngSeed
pub struct AreaGenerationSystem;

// systems/render.rs
/// Collects all visible renderable entities.
/// Builds sprite batches, draws tilemaps, submits wgpu commands.
/// Queries: (Position, Renderable, Animation, DepthLayer, optionally Tilemap)
/// Resources: Camera, AssetManager, RenderState
pub struct RenderSystem;

// systems/ui.rs
/// Draws HUD elements, menus, chat log, minimap.
/// Processes UI interaction events.
/// Resources: GameState, InputState, PartyState, UILayout
pub struct UISystem;

// systems/audio.rs
/// Manages music tracks and SFX playback.
/// Handles crossfade, volume, spatial audio.
/// Events: PlayMusicEvent, PlaySFXEvent, StopAudioEvent
/// Resources: AudioManager, Time
pub struct AudioSystem;
```

### 2.3 Resources

```rust
// resources/game_state.rs

/// Top-level game state machine
pub struct GameState {
    pub current: GamePhase,
    pub previous: GamePhase,
    pub transition_timer: f32,
    pub paused: bool,
    pub tick_count: u64,
}

pub enum GamePhase {
    Boot,
    Title,
    Connecting,
    CharacterSelect,
    Loading,
    Exploring,
    Combat,
    Menu,
    Cutscene,
    Disconnected,
}

// resources/input.rs

/// Buffered input state for the current frame
pub struct InputState {
    pub keyboard: HashMap<VirtualKeyCode, KeyState>,
    pub mouse: MouseState,
    pub gamepad: Option<GamepadState>,
    pub actions: Vec<GameAction>,
    pub cursor_world: (f32, f32),     // Mouse position in world coords
}

pub struct MouseState {
    pub position: (f32, f32),          // Screen coords
    pub left_click: bool,
    pub right_click: bool,
    pub scroll_delta: f32,
}

pub enum GameAction {
    MoveUp, MoveDown, MoveLeft, MoveRight,
    BasicAttack, Skill1, Skill2, Skill3, Skill4, Skill5, Skill6,
    Dodge, Guard, Interact, DataDrain,
    Menu, PartyMenu, Cancel, Confirm,
    TacticAggressive, TacticDefensive, TacticBalanced,
    TacticFocus, TacticScatter,
}

pub struct Bindings {
    pub key_map: HashMap<GameAction, Vec<VirtualKeyCode>>,
    pub gamepad_map: HashMap<GameAction, GamepadButton>,
}

// resources/camera.rs

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub target: Option<(f32, f32)>,         // Smooth follow target
    pub lerp_speed: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub world_bounds: Option<(f32, f32, f32, f32)>,  // MinX, MinY, MaxX, MaxY
}

// resources/asset_manager.rs

pub struct AssetManager {
    pub textures: HashMap<String, TextureHandle>,
    pub spritesheets: HashMap<String, SpritesheetData>,
    pub tilemaps: HashMap<String, TilemapData>,
    pub fonts: HashMap<String, FontHandle>,
    pub audio_buffers: HashMap<String, AudioHandle>,
    pub configs: HashMap<String, ConfigData>,
}

pub struct TextureHandle {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

pub struct SpritesheetData {
    pub texture_id: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub columns: u32,
    pub rows: u32,
}

// resources/audio_manager.rs

pub struct AudioManager {
    pub music_player: MusicPlayer,      // kira instance
    pub sfx_player: SfxPlayer,
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub current_track: Option<String>,
    pub crossfade_duration: f32,
}

// resources/network_state.rs

pub struct NetworkState {
    pub connected: bool,
    pub server_tick: u64,
    pub latency: f32,                   // Simulated ping
    pub players_online: Vec<PlayerBrief>,
    pub event_queue: VecDeque<NetworkEvent>,
    pub connection_id: u32,
    pub simulated: bool,                // True = local emulation mode
}

// resources/time.rs

pub struct Time {
    pub delta: f32,                     // Seconds since last frame
    pub fixed_delta: f32,               // Fixed timestep (1/60)
    pub elapsed: f32,                   // Total elapsed gameplay seconds
    pub frame_count: u64,
    pub scale: f32,                     // Time scale (slow-motion, pause)
}
```

---

## 3. Rendering Pipeline (Sprint 2)

### 3.1 Architecture Overview
The rendering pipeline uses wgpu (Vulkan/DX12/Metal backend) with winit for window management.

### 3.2 Window & Surface Creation
- winit EventLoop for window events
- wgpu Surface for rendering
- Swap chain with vsync
- Resize handling

### 3.3 Render Pipeline
- Vertex shader: 2D sprite vertex processing
- Fragment shader: Texture sampling with alpha blending
- Uniform buffers: Camera projection matrix
- Sprite batching: Single draw call for visible sprites

### 3.4 Sprite System
- Texture atlas for sprite sheets
- SpriteBatch: Collects visible sprites, sorts by layer, batches by texture
- Animation system: Frame-based animation with configurable FPS
- Tilemap rendering: Chunk-based tile rendering for areas

### 3.5 UI Rendering
- Separate render pass for UI overlay
- 9-slice scaling for panels
- Text rendering (bitmap font)
- Progress bar rendering

### 3.6 Render Graph
```
Frame Start
  ├─ Clear screen (dark blue/black)
  ├─ Sprite Pass (Background layer)
  ├─ Sprite Pass (Floor layer)
  ├─ Sprite Pass (Items layer)
  ├─ Sprite Pass (Characters layer)
  ├─ Sprite Pass (Effects layer)
  ├─ UI Pass (HUD elements)
  └─ Present to surface
```

### 3.7 Performance Targets
- 60 FPS at 1280x720
- < 1000 draw calls per frame
- < 16ms frame time
- < 256 MB GPU memory

### 3.8 Texture Loading Pipeline
- Load PNG via `image` crate
- Convert to `wgpu::Texture` with mipmaps
- Create `wgpu::Sampler` with bilinear filtering
- Texture atlas packing for sprite sheets
- Async loading with progress tracking

---

## 4. Physics & Collision

### 4.1 Physics Engine

Shambala uses [`rapier2d`](https://crates.io/crates/rapier2d) for physics and collision detection. rapier2d was chosen for its:

- Deterministic simulation (important for replay / networking)
- Efficient broad-phase (SAP + spatial grid)
- Built-in collision groups and query pipeline
- No external dependencies (pure Rust)

```rust
// Physics setup
struct PhysicsResources {
    pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    query_pipeline: QueryPipeline,
    physics_hooks: PhysicsHooks,
    event_handler: EventHandler,
}
```

### 4.2 Collision Layers

```
Layer Bitmask Layout (u32):
Bit 0: Player
Bit 1: PlayerProjectile
Bit 2: Companion
Bit 3: Enemy
Bit 4: EnemyProjectile
Bit 5: Terrain (walls, obstacles)
Bit 6: Interactable (chests, doors, NPCs)
Bit 7: TriggerZones (area transitions, traps)
Bit 8: Sensor (proximity detection)

Collision Matrix:
                    Player  ProjP  Comp  Enemy  ProjE  Terrain  Interact  Trigger  Sensor
Player              -       -      -     ✔     -      ✔        ✔         ✔        -
PlayerProjectile    -       -      -     ✔     -      ✔        -         -        -
Companion           -       -      -     ✔     -      ✔        ✔         ✔        -
Enemy               ✔       ✔      ✔     ✔     -      ✔        ✔         ✔        -
EnemyProjectile     ✔       -      ✔     -     -      ✔        -         -        -
Terrain             ✔       ✔      ✔     ✔     ✔      -        -         -        -
Interactable        ✔       -      ✔     ✔     -      -        -         -        -
TriggerZones        -       -      -     -     -      -        -         -        -
Sensor              -       -      -     -     -      -        -         -        -
```

### 4.3 Spatial Partitioning

rapier2d provides a **dynamic broad-phase** using a combination of:

1. **Sweep-and-Prune (SAP)** — default broad phase for dynamic objects
2. **Spatial Grid** — for static geometry (tilemap collision tiles)

```rust
// For tilemap collision, we use a simpler grid lookup:
struct CollisionGrid {
    cell_size: f32,                // Tile size in world units
    width: u32,                    // Number of cells horizontally
    height: u32,                   // Number of cells vertically
    cells: Vec<Vec<Entity>>,       // Entities per cell
}

impl CollisionGrid {
    // Query all entities in cells overlapping a rectangle
    fn query_rect(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<Entity>;
    // Update entity position in grid
    fn update_entity(&mut self, entity: Entity, old_x: f32, old_y: f32, new_x: f32, new_y: f32);
}
```

### 4.4 Physics Query Pipeline

```rust
// Common queries used by systems:

// 1. Raycasting (line of sight, targeting)
let ray = Ray::new(point, direction);
let filter = QueryFilter::new().groups(InteractionGroups::new(0b0001, 0b1000));
let hit = query_pipeline.cast_ray(&rigid_body_set, &collider_set, &ray, max_toi, true, filter);

// 2. Point intersection (mouse picking)
let filter = QueryFilter::new().groups(InteractionGroups::new(0b0001, 0b0100));
let hits = query_pipeline.points_projections(&rigid_body_set, &collider_set, &point, true, filter);

// 3. Area overlap (AoE attacks, Data Drain range)
let shape = Ball::new(radius);
let filter = QueryFilter::new().groups(InteractionGroups::new(0b0001, 0b0100));
let hits = query_pipeline.collisions_with_shape(&rigid_body_set, &collider_set, &shape, &pose, filter);
```

### 4.5 Movement Model

```rust
// Player movement uses acceleration-based physics:
//   1. InputSystem sets desired velocity direction
//   2. PhysicsSystem applies acceleration toward desired velocity
//   3. Friction decelerates when no input
//   4. Collision resolution handles wall sliding

// Parameters:
const PLAYER_ACCELERATION: f32 = 500.0;    // pixels/s²
const PLAYER_FRICTION: f32 = 800.0;         // pixels/s²
const PLAYER_MAX_SPEED: f32 = 200.0;        // pixels/s
const PLAYER_DODGE_SPEED: f32 = 400.0;      // pixels/s (i-frames active)
const PLAYER_DODGE_DURATION: f32 = 0.3;     // seconds
```

---

## 5. Data-Driven Design

### 5.1 Asset Loading Pipeline

```
┌──────────┐    ┌──────────────┐    ┌────────────┐    ┌──────────────┐
│  Raw      │───▶│  Asset       │───▶│  Runtime   │───▶│  GPU/Engine  │
│  Files    │    │  Loader      │    │  Cache     │    │  Resource    │
└──────────┘    └──────────────┘    └────────────┘    └──────────────┘

Asset Directory Structure:
assets/
├── textures/
│   ├── spritesheets/           # .png + .ron metadata
│   │   ├── player_twin_blade.png
│   │   ├── player_twin_blade.ron
│   │   ├── player_heavy_blade.png
│   │   ├── enemies.png
│   │   └── effects.png
│   ├── tilesets/               # .png + .ron tileset definitions
│   │   ├── delta_forest.png
│   │   ├── theta_desert.png
│   │   └── mac_anu.png
│   └── ui/                     # .png UI elements
│       ├── hud_panels.png
│       ├── icons.png
│       └── fonts/              # .ttf or .otf
├── audio/
│   ├── music/                  # .ogg or .flac
│   ├── sfx/                    # .ogg or .wav
│   └── ambient/                # .ogg looping ambience
├── maps/
│   ├── mac_anu.tmx             # Tiled editor maps (root town)
│   ├── templates/              # Area generation templates
│   │   ├── forest_clearing.tmx
│   │   ├── canyon_pass.tmx
│   │   └── cave_network.tmx
│   └── tilesets/               # .tsx tileset files
├── config/
│   ├── skills.ron              # Skill definitions
│   ├── items.ron               # Item definitions
│   ├── enemies.ron             # Enemy stat tables
│   ├── classes.ron             # Class starting stats
│   ├── dialogues.ron           # Dialogue trees
│   └── quests.ron              # Quest definitions
├── save/                       # Runtime save data directory
│   └── (auto-created)
└── shaders/
    ├── sprite.wgsl             # Sprite vertex + fragment
    ├── tilemap.wgsl            # Tilemap vertex + fragment
    ├── post_process.wgsl       # Screen-space effects
    └── ui.wgsl                 # UI vertex + fragment
```

### 5.2 Asset Loading System

```rust
// Asset loading uses a configurable loader with progress tracking
struct AssetLoader {
    load_queue: Vec<AssetLoadTask>,
    loaded: HashMap<String, AssetHandle>,
    progress: LoadProgress,
    thread_pool: Arc<rayon::ThreadPool>,
}

struct AssetLoadTask {
    path: PathBuf,
    asset_type: AssetType,
    priority: LoadPriority,
    callback: Option<Box<dyn FnOnce(AssetHandle) + Send>>,
}

enum LoadPriority {
    Critical,       // Must load before game starts (UI, player sprites)
    High,           // Needed for current area
    Medium,         // May be needed soon
    Low,            // Background load (future areas)
    Streaming,      // Load on-demand (large audio files)
}

// Loading pipeline per asset type:
impl TextureLoader {
    // 1. Read .png file via image crate
    // 2. Decode to RGBA8 byte buffer
    // 3. Upload to wgpu texture via queue.write_texture()
    // 4. Create TextureView and Sampler
    // 5. Store in AssetManager.textures
}

impl SpritesheetLoader {
    // 1. Load associated .ron metadata
    // 2. Load texture via TextureLoader
    // 3. Store frame dimensions and layout
}

impl AudioLoader {
    // 1. Read .ogg file
    // 2. Decode via kira's decoder
    // 3. Store as AudioHandle (pre-decoded buffer)
}

impl ConfigLoader {
    // 1. Read .ron or .toml file
    // 2. Deserialize via serde into typed struct
    // 3. Store in AssetManager.configs
}
```

### 5.3 Configuration File Formats

Shambala uses **RON** (Rusty Object Notation) for game data and **TOML** for user-facing settings.

```ron
// assets/config/skills.ron
SkillsConfig(
    skills: [
        SkillDef(
            id: "tb_sword_flurry",
            name: "Sword Flurry",
            skill_type: Melee,
            damage_multiplier: 1.5,
            sp_cost: 15,
            cooldown: 3.0,
            cast_time: 0.2,
            range: 48.0,
            area_of_effect: 0.0,
            status_effect: None,
            animation_key: "tb_attack_3",
            unlock_level: 1,
            class: TwinBlade,
            description: "A rapid three-hit combo that builds chain gauge quickly.",
        ),
        SkillDef(
            id: "wm_heal",
            name: "Heal",
            skill_type: Heal,
            damage_multiplier: 0.0,
            sp_cost: 25,
            cooldown: 8.0,
            cast_time: 0.8,
            range: 100.0,
            area_of_effect: 0.0,
            status_effect: Some(StatusEffectData(
                effect_type: Regen,
                duration: 5.0,
                magnitude: 30.0,
                tick_interval: 1.0,
            )),
            animation_key: "wm_cast_heal",
            unlock_level: 2,
            class: Wavemaster,
            description: "Restores a moderate amount of HP to a single ally.",
        ),
    ],
)
```

```toml
# assets/config/settings.toml (user-facing, defaults)
[graphics]
resolution = [1920, 1080]
fullscreen = false
vsync = true
scale_factor = 3
particle_quality = "high"  # "low", "medium", "high"

[audio]
master_volume = 0.8
music_volume = 0.7
sfx_volume = 1.0

[gameplay]
pause_on_menu = true
auto_target = false
difficulty = "normal"

[controls]
move_up = ["W", "Up"]
move_down = ["S", "Down"]
move_left = ["A", "Left"]
move_right = ["D", "Right"]
basic_attack = ["MouseLeft"]
dodge = ["Space"]
data_drain = ["F"]
menu = ["Escape"]
```

### 5.4 Serialization (Save/Load)

```rust
// Save file format: binary RON (.save)
// Each save file contains:

struct SaveData {
    version: u32,                          // Schema version for migration
    timestamp: u64,                        // Unix timestamp
    play_time: f32,                        // Total play time in seconds

    // Player state
    player_name: String,
    player_class: ClassType,
    player_stats: Stats,
    player_health: Health,
    player_equipment: Equipment,
    player_inventory: Inventory,
    player_skills: SkillSet,
    player_position: (f32, f32),
    player_area: String,

    // Party state
    party_members: Vec<PartySaveData>,

    // Progress
    completed_quests: Vec<String>,
    active_quests: Vec<String>,
    unlocked_areas: Vec<String>,
    collected_memory_fragments: Vec<String>,
    story_flags: HashMap<String, bool>,
    bond_levels: HashMap<String, u32>,

    // Settings
    settings: SettingsConfig,
}

// Save manager
struct SaveManager {
    save_directory: PathBuf,
    current_save: Option<SaveData>,
    auto_save_timer: f32,
    auto_save_interval: f32,        // 120 seconds
    max_save_slots: u8,             // 10 slots
}

impl SaveManager {
    fn save_to_slot(&self, slot: u8, data: &SaveData) -> Result<()>;
    fn load_from_slot(&self, slot: u8) -> Result<SaveData>;
    fn auto_save(&mut self, data: &SaveData) -> Result<()>;
    fn list_saves(&self) -> Vec<SaveMeta>;
    fn migrate_save(&self, old_data: &[u8]) -> Result<SaveData>;
}
```

---

## 6. Module Structure

### 6.1 Complete Module Tree

```
shambala/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── assets/                              # Game assets (not in src/)
│   ├── textures/
│   │   ├── spritesheets/
│   │   ├── tilesets/
│   │   └── ui/
│   ├── audio/
│   │   ├── music/
│   │   ├── sfx/
│   │   └── ambient/
│   ├── maps/
│   │   ├── templates/
│   │   └── tilesets/
│   ├── config/
│   ├── save/
│   └── shaders/
├── docs/
│   ├── GDD.md                           # Game Design Document
│   └── TECHNICAL_DESIGN.md              # This document
├── src/
│   ├── main.rs                          # Entry point, winit event loop
│   ├── lib.rs                           # Library root, module declarations
│   │
│   ├── components/                      # ECS Component definitions
│   │   ├── mod.rs                       # Re-exports all components
│   │   ├── position.rs                  # Position, Velocity, AABB, Facing, DepthLayer
│   │   ├── render.rs                    # Renderable, Animation, ParticleEmitter, BlendMode
│   │   ├── stats.rs                     # Stats, Health, Combat, Equipment, Inventory, ItemData
│   │   ├── skill.rs                     # SkillSet, Skill, SkillType
│   │   ├── status.rs                    # StatusEffects, StatusEffectInstance, StatusEffectType
│   │   ├── party.rs                     # PartyMember, CompanionAI, PartyTactic, CompanionPersonality
│   │   ├── data_drain.rs               # DataDrain, CorruptionMeter, DrainPhase
│   │   ├── ai.rs                        # AIState, EnemyType, AIBehavior, EnemyCategory
│   │   └── area.rs                      # AreaData, Tilemap, MapMarker, TerrainType, etc.
│   │
│   ├── systems/                         # ECS System implementations
│   │   ├── mod.rs                       # Re-exports all systems
│   │   ├── input.rs                     # InputSystem — raw input → game actions
│   │   ├── physics.rs                   # PhysicsSystem — movement, collision resolution
│   │   ├── combat.rs                    # CombatSystem — damage, skills, healing
│   │   ├── ai.rs                        # AISystem — behavior trees for enemies/companions
│   │   ├── party.rs                     # PartySystem — bond XP, tactics, companion management
│   │   ├── data_drain.rs               # DataDrainSystem — gauge, minigame, results
│   │   ├── area_generation.rs           # AreaGenerationSystem — procgen on zone entry
│   │   ├── animation.rs                 # AnimationSystem — sprite frame advancement
│   │   ├── audio.rs                     # AudioSystem — music/SFX triggers
│   │   ├── render.rs                    # RenderSystem — sprite batching, tilemap drawing
│   │   └── ui.rs                        # UISystem — HUD, menus, chat, minimap
│   │
│   ├── entities/                        # Entity factories / spawners
│   │   ├── mod.rs                       # Re-exports spawn functions
│   │   ├── player.rs                    # spawn_player() — construct player entity
│   │   ├── companion.rs                 # spawn_companion(id) — construct companion entity
│   │   ├── enemy.rs                     # spawn_enemy(type, level) — construct enemy entity
│   │   ├── item.rs                      # spawn_item_drop(data, pos) — item/chest entities
│   │   ├── projectile.rs                # spawn_projectile(source, skill) — attack projectiles
│   │   └── effects.rs                   # spawn_effect(type, pos) — particle effects
│   │
│   ├── resources/                       # ECS Resource singletons
│   │   ├── mod.rs                       # Re-exports all resources
│   │   ├── game_state.rs                # GameState, GamePhase enum, state transitions
│   │   ├── input.rs                     # InputState, MouseState, GameAction, Bindings
│   │   ├── camera.rs                    # Camera — viewport, follow, shake
│   │   ├── time.rs                      # Time — delta, fixed timestep, scale
│   │   ├── asset_manager.rs             # AssetManager — texture/font/audio cache
│   │   ├── audio_manager.rs             # AudioManager — music/SFX players, volumes
│   │   ├── network_state.rs             # NetworkState — connection, events, player list
│   │   ├── party_state.rs               # PartyState — roster, active members
│   │   ├── dialog_state.rs              # DialogState — active conversation, choices
│   │   └── config.rs                    # GameConfig — runtime settings from settings.toml
│   │
│   ├── render/                          # Rendering internals (wgpu)
│   │   ├── mod.rs                       # Renderer struct, initialization
│   │   ├── pipeline.rs                  # wgpu pipeline setup
│   │   ├── sprite.rs                    # Sprite batching & rendering
│   │   ├── tilemap.rs                   # Tilemap rendering
│   │   ├── ui_render.rs                 # UI overlay rendering
│   │   ├── text.rs                      # Text rendering
│   │   └── shaders/                     # WGSL shader files
│   │       ├── sprite.wgsl
│   │       ├── tilemap.wgsl
│   │       └── ui.wgsl
│   │
│   ├── physics/                         # Physics abstractions
│   │   ├── mod.rs                       # PhysicsWorld wrapper
│   │   ├── collision_grid.rs            # Spatial grid for tilemap collision
│   │   └── layers.rs                    # CollisionGroup bitmask constants
│   │
│   ├── audio/                           # Audio abstractions (kira)
│   │   ├── mod.rs                       # AudioEngine wrapper
│   │   ├── music.rs                     # MusicPlayer — track queue, crossfade
│   │   └── sfx.rs                       # SfxPlayer — one-shots, spatial audio
│   │
│   ├── network/                         # Simulated MMO networking
│   │   ├── mod.rs                       # NetworkManager
│   │   ├── server_emulator.rs           # Local server process simulation
│   │   ├── events.rs                    # NetworkEvent enum, event bus
│   │   └── player_sync.rs              # Player state synchronization
│   │
│   ├── data/                            # Data-driven configuration
│   │   ├── mod.rs                       # DataManager, load_all()
│   │   ├── skills.rs                    # SkillDef deserialization from .ron
│   │   ├── items.rs                     # ItemDef deserialization from .ron
│   │   ├── enemies.rs                   # EnemyDef deserialization from .ron
│   │   ├── classes.rs                   # ClassDef deserialization from .ron
│   │   ├── dialogues.rs                 # DialogueNode deserialization from .ron
│   │   ├── quests.rs                    # QuestDef deserialization from .ron
│   │   └── save.rs                      # SaveData serialization/deserialization
│   │
│   ├── ai/                              # AI behavior tree framework
│   │   ├── mod.rs                       # BehaviorTree trait
│   │   ├── nodes.rs                     # BehaviorNode types (Sequence, Selector, Condition, Action)
│   │   ├── trees.rs                     # Predefined behavior trees per enemy/companion type
│   │   └── evaluator.rs                 # BehaviorTreeEvaluator — runtime execution
│   │
│   ├── procgen/                         # Procedural generation
│   │   ├── mod.rs                       # AreaGenerator
│   │   ├── terrain.rs                   # Terrain generation using Perlin noise
│   │   ├── room_placer.rs               # Room/path placement algorithms
│   │   ├── enemy_placer.rs              # Enemy spawn point distribution
│   │   ├── loot_placer.rs               # Chest/item placement logic
│   │   └── templates.rs                 # Hand-authored template loading
│   │
│   ├── ui/                              # UI framework internals
│   │   ├── mod.rs                       # UIManager, layout system
│   │   ├── layout.rs                    # Box model, flex-like layout
│   │   ├── widgets.rs                   # Widget types (Button, Panel, Label, etc.)
│   │   ├── hud.rs                       # HUD layout and data binding
│   │   ├── menus.rs                     # Menu screens (main menu, pause, status, etc.)
│   │   ├── chat.rs                      # Chat log rendering and management
│   │   └── minimap.rs                   # Minimap rendering
│   │
│   ├── events/                          # Event system
│   │   ├── mod.rs                       # EventBus, EventHandler trait
│   │   ├── combat_events.rs             # AttackEvent, DamageEvent, DeathEvent, SkillEvent
│   │   ├── drain_events.rs              # DataDrainEvent, DrainResultEvent
│   │   ├── audio_events.rs              # PlayMusicEvent, PlaySFXEvent, StopAudioEvent
│   │   ├── area_events.rs               # AreaTransitionEvent, ZoneCompleteEvent
│   │   ├── quest_events.rs              # QuestProgressEvent, QuestCompleteEvent
│   │   └── ui_events.rs                 # MenuOpenEvent, DialogueAdvanceEvent
│   │
│   └── util/                            # Shared utilities
│       ├── mod.rs
│       ├── math.rs                      # Vector2D, lerp, clamp, angle helpers
│       ├── rng.rs                       # Seeded RNG wrapper (rand crate)
│       ├── timer.rs                     # Timer, Stopwatch utility structs
│       ├── id_generator.rs              # Entity ID generation
│       └── error.rs                     # Custom error types, Result aliases
│
├── tests/                               # Integration tests
│   ├── integration/
│   │   ├── combat_tests.rs              # Full combat scenario: player vs enemy
│   │   ├── data_drain_tests.rs          # Data Drain activation → result cycle
│   │   ├── party_tests.rs               # Party formation, bond XP, tactics
│   │   ├── area_gen_tests.rs            # Area generation from keywords
│   │   ├── physics_tests.rs             # Movement, collision, wall sliding
│   │   ├── save_load_tests.rs           # Save → serialize → deserialize → verify
│   │   └── network_tests.rs             # Simulated server event sync
│   └── helpers/
│       ├── mod.rs
│       ├── test_ecs.rs                  # Test ECS world builder utilities
│       └── fixtures.rs                  # Common test data (sample entities, configs)
│
└── benches/                             # Benchmark tests
    ├── render_bench.rs                  # Sprite batching throughput
    ├── collision_bench.rs               # Physics query performance
    ├── ai_bench.rs                      # Behavior tree evaluation speed
    └── ecs_bench.rs                     # ECS query/iteration overhead
```

### 6.2 Module Dependency Graph

```
main.rs
  └── lib.rs
        ├── components/    (no internal deps — pure data)
        ├── entities/      → components/
        ├── resources/     → components/, events/
        ├── events/        → components/
        ├── util/          → (external crates only)
        ├── data/          → components/, util/
        ├── render/        → resources/camera, resources/asset_manager, util/
        ├── physics/       → components/position, util/
        ├── audio/         → resources/audio_manager
        ├── ai/            → components/ai, components/position, util/
        ├── procgen/       → data/, util/, components/area
        ├── ui/            → resources/game_state, resources/input, events/
        ├── network/       → events/, resources/network_state
        └── systems/       → ALL of the above
              ├── input/             → resources/input, events/
              ├── physics/           → physics/, components/position
              ├── combat/            → components/stats, components/skill, events/
              ├── ai/                → ai/, components/ai
              ├── party/             → components/party, resources/party_state
              ├── data_drain/        → components/data_drain, events/
              ├── area_generation/   → procgen/, components/area
              ├── animation/         → components/render
              ├── audio/             → audio/, events/
              ├── render/            → render/, components/render
              └── ui/                → ui/, resources/
```

---

## 7. State Management

### 7.1 Game State Machine

Shambala uses a **hierarchical state machine** with explicit transitions. The current state determines which systems run, what is rendered, and what input is processed.

```
                    ┌─────────────────────────────────────┐
                    │               Boot                   │
                    │   Initialize engine, load core       │
                    │   assets, check save files           │
                    └──────────────┬──────────────────────┘
                                   │ init complete
                                   ▼
                    ┌─────────────────────────────────────┐
          ┌────────▶│              Title                   │◀────────────────┐
          │         │   Main menu: New Game, Load,         │                │
          │         │   Settings, Quit                     │                │
          │         └──────┬──────────────┬────────────────┘                │
          │                │              │                                 │
          │         new game        load game                               │
          │                │              │                                 │
          │                ▼              ▼                                 │
          │         ┌─────────────────────────────────────┐                │
          │         │           Connecting                  │                │
          │         │   "Connecting to server..."           │                │
          │         │   Simulated handshake + version check │                │
          │         └──────────────┬──────────────────────┘                │
          │                        │ connected                              │
          │                        ▼                                       │
          │         ┌─────────────────────────────────────┐                │
          │         │         Character Select              │                │
          │         │   Create/choose character, class      │                │
          │         │   selection                            │                │
          │         └──────────────┬──────────────────────┘                │
          │                        │ confirmed                              │
          │                        ▼                                       │
          │         ┌─────────────────────────────────────┐                │
          │         │             Loading                   │                │
          │         │   "Loading area..."                   │                │
          │         │   Show loading screen + tips          │                │
          │         └──────────────┬──────────────────────┘                │
          │                        │ loaded                                │
          │                        ▼                                       │
          │    ┌────────────────────────────────────────────────────────┐  │
          │    │                                                        │  │
          │    │              ┌──────────────────┐                      │  │
          │    │              │    Exploring      │                     │  │
          │    │              │  Free roam,       │   combat start      │  │
          │    │              │  movement,        │──────────────────▶  │  │
          │    │              │  interaction       │                   │  │
          │    │              └────────┬─────────┘                    │  │
          │    │                       │      ▲                       │  │
          │    │              escape   │      │ combat end             │  │
          │    │                       ▼      │                       │  │
          │    │              ┌──────────────────┐                    │  │
          │    │              │      Menu        │   open menu        │  │
          │    │              │  Pause, status,   │◀──────────────────  │  │
          │    │              │  inventory, etc.  │                    │  │
          │    │              └──────────────────┘                    │  │
          │    │                       │                               │  │
          │    │              cutscene trigger  │                    │  │
          │    │                       │                               │  │
          │    │              ┌──────────────────┐                    │  │
          │    │              │    Cutscene       │                    │  │
          │    │              │  Dialogue,        │                    │  │
          │    │              │  scripted events  │──── continue ───▶ │  │
          │    │              └──────────────────┘                    │  │
          │    └──────────────────────────┬─────────────────────────────┘  │
          │                               │                                │
          │                      disconnected                              │
          │                               │                                │
          │         ┌──────────────────────▼────────────────────────┐      │
          │         │              Disconnected                      │      │
          │         │   "Connection lost. Reconnect?"               │      │
          │         │   Yes → Connecting | No → Title                │──────┘
          │         └───────────────────────────────────────────────┘
          │
          └────────────────────── quit ───────────────────────────┘
```

### 7.2 State Transition Logic

```rust
impl GameState {
    pub fn transition(&mut self, next: GamePhase) -> GameTransition {
        let prev = self.current.clone();
        self.previous = prev.clone();
        self.current = next.clone();

        GameTransition {
            from: prev,
            to: next,
            on_exit: Self::exit_actions(&prev),
            on_enter: Self::enter_actions(&next),
        }
    }

    fn enter_actions(phase: &GamePhase) -> Vec<SystemAction> {
        match phase {
            GamePhase::Boot => vec![
                SystemAction::InitRenderer,
                SystemAction::LoadCoreAssets,
                SystemAction::InitAudio,
            ],
            GamePhase::Title => vec![
                SystemAction::ShowTitleScreen,
                SystemAction::PlayMusic("title_theme"),
            ],
            GamePhase::Connecting => vec![
                SystemAction::ShowConnectingScreen,
                SystemAction::StartConnectionSimulation,
            ],
            GamePhase::CharacterSelect => vec![
                SystemAction::ShowCharacterSelect,
                SystemAction::StopMusic,
                SystemAction::PlayMusic("character_select"),
            ],
            GamePhase::Loading => vec![
                SystemAction::ShowLoadingScreen,
                SystemAction::LoadSaveData,
                SystemAction::LoadAreaAssets,
            ],
            GamePhase::Exploring => vec![
                SystemAction::ResumeGameLoop,
                SystemAction::ShowHUD,
                SystemAction::PlayAreaMusic,
                SystemAction::EnablePlayerInput,
            ],
            GamePhase::Combat => vec![
                SystemAction::EnterCombatMode,
                SystemAction::PlayMusic("combat"),
                SystemAction::ShowCombatUI,
            ],
            GamePhase::Menu => vec![
                SystemAction::PauseGameLoop,
                SystemAction::ShowMenu,
                SystemAction::PlaySFX("menu_open"),
            ],
            GamePhase::Cutscene => vec![
                SystemAction::PauseGameLoop,
                SystemAction::HideHUD,
                SystemAction::LockPlayerInput,
                SystemAction::StartCutscene,
            ],
            GamePhase::Disconnected => vec![
                SystemAction::PauseGameLoop,
                SystemAction::ShowDisconnectDialog,
                SystemAction::PlaySFX("disconnect"),
            ],
        }
    }

    fn exit_actions(phase: &GamePhase) -> Vec<SystemAction> {
        match phase {
            GamePhase::Exploring => vec![
                SystemAction::DisablePlayerInput,
            ],
            GamePhase::Combat => vec![
                SystemAction::ExitCombatMode,
            ],
            GamePhase::Menu => vec![
                SystemAction::PlaySFX("menu_close"),
                SystemAction::ResumeGameLoop,
            ],
            GamePhase::Cutscene => vec![
                SystemAction::UnlockPlayerInput,
                SystemAction::ShowHUD,
                SystemAction::ResumeGameLoop,
            ],
            _ => vec![],
        }
    }
}
```

### 7.3 System Enable/Disable Per State

| System | Boot | Title | Connect | CharSelect | Loading | Explore | Combat | Menu | Cutscene | DC |
|---|---|---|---|---|---|---|---|---|---|---|
| InputSystem | - | ✔ | - | ✔ | - | ✔ | ✔ | ✔ | - | ✔ |
| PhysicsSystem | - | - | - | - | - | ✔ | ✔ | - | - | - |
| CombatSystem | - | - | - | - | - | ✔ | ✔ | - | - | - |
| AISystem | - | - | - | - | - | ✔ | ✔ | - | - | - |
| PartySystem | - | - | - | - | - | ✔ | ✔ | ✔ | - | - |
| DataDrainSystem | - | - | - | - | - | ✔ | ✔ | - | - | - |
| AreaGenSystem | - | - | - | - | ✔ | ✔ | - | - | - | - |
| AnimationSystem | - | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ |
| AudioSystem | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ |
| RenderSystem | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ |
| UISystem | - | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ |

---

## 8. Networking Layer

### 8.1 Architecture

Shambala is a **single-player game** that simulates an MMO experience. The "network" layer emulates a local game server within the same process, providing:

- Realistic connection/disconnection flow
- Simulated latency and packet loss
- Emulated other player NPCs
- Server-authoritative event validation (cheat resistance framework)

```
┌─────────────────────────────────────────────────────────────────┐
│                     Shambala Process                              │
│                                                                   │
│  ┌──────────────────────┐      ┌──────────────────────────────┐  │
│  │    Game Client        │      │    Local Server Emulator      │  │
│  │  (ECS World + UI)    │      │  (Simulated server process)   │  │
│  │                      │      │                               │  │
│  │  ┌────────────────┐  │      │  ┌─────────────────────────┐  │  │
│  │  │ Event Queue    │──┼──────┼─▶│ Event Bus              │  │  │
│  │  └────────────────┘  │      │  │ - Validate actions      │  │  │
│  │         ▲            │      │  │ - Process server tick   │  │  │
│  │         │            │      │  │ - Broadcast events      │  │  │
│  │  ┌────────────────┐  │      │  └─────────────────────────┘  │  │
│  │  │ Render Systems │  │      │  ┌─────────────────────────┐  │  │
│  │  └────────────────┘  │      │  │ World State Manager     │  │  │
│  │                      │      │  │ - Authoritative state   │  │  │
│  │                      │      │  │ - Player sync data      │  │  │
│  └──────────────────────┘      │  │ - AI NPC simulation     │  │  │
│                                 │  └─────────────────────────┘  │  │
│                                 │  ┌─────────────────────────┐  │  │
│                                 │  │ Chat/Message Relay      │  │  │
│                                 │  │ - System messages       │  │  │
│                                 │  │ - NPC chatter           │  │  │
│                                 │  │ - Simulated player chat │  │  │
│                                 │  └─────────────────────────┘  │  │
│                                 └──────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 Event System

Events are the primary communication mechanism between systems, the client, and the simulated server.

```rust
// events/mod.rs

/// The central event bus using a channel-based dispatcher
pub struct EventBus {
    subscribers: HashMap<EventType, Vec<Box<dyn EventHandler>>>,
    queue: VecDeque<GameEvent>,
    pending: Vec<GameEvent>,         // Events for next frame dispatch
}

/// All game events are defined as an enum for exhaustive matching
#[derive(Clone, Debug)]
pub enum GameEvent {
    // Input Events
    InputAction(GameAction),
    InputBindingsChanged,

    // Combat Events
    AttackEntity(AttackEvent),
    EntityDamaged(DamageEvent),
    EntityHealed(HealEvent),
    EntityDied(DeathEvent),
    SkillUsed(SkillEvent),
    ComboChainUpdated(ComboEvent),

    // Data Drain Events
    DataDrainActivated(DataDrainEvent),
    DataDrainMinigameResult(DrainResultEvent),
    DataDrainComplete(DrainCompleteEvent),

    // Party Events
    PartyMemberJoined(PartyMemberEvent),
    PartyMemberLeft(PartyMemberEvent),
    TacticChanged(TacticChangeEvent),
    BondLevelUp(BondEvent),

    // Area Events
    AreaEntered(AreaTransitionEvent),
    ZoneCompleted(ZoneCompleteEvent),
    BossDefeated(BossDefeatEvent),

    // Quest Events
    QuestAccepted(QuestEvent),
    QuestProgressed(QuestProgressEvent),
    QuestCompleted(QuestEvent),

    // Audio Events
    PlayMusic(PlayMusicEvent),
    PlaySFX(PlaySFXEvent),
    StopAudio(StopAudioEvent),

    // Network Events
    ServerConnected(ConnectionEvent),
    ServerDisconnected(DisconnectEvent),
    PlayerJoined(PlayerSyncEvent),
    PlayerLeft(PlayerSyncEvent),
    ChatMessage(ChatMessageEvent),

    // System Events
    GameStateChanged(GamePhase),
    SaveTriggered,
    LoadTriggered,
}

// Event data structures
#[derive(Clone, Debug)]
pub struct AttackEvent {
    pub source: Entity,
    pub target: Entity,
    pub skill_id: Option<String>,
    pub position: (f32, f32),
}

#[derive(Clone, Debug)]
pub struct DamageEvent {
    pub target: Entity,
    pub source: Entity,
    pub damage: u32,
    pub damage_type: DamageType,
    pub is_critical: bool,
}

#[derive(Clone, Debug)]
pub enum DamageType {
    Physical,
    Magical,
    True,
    DataDrain,
}

#[derive(Clone, Debug)]
pub struct DeathEvent {
    pub entity: Entity,
    pub killer: Option<Entity>,
    pub position: (f32, f32),
}

#[derive(Clone, Debug)]
pub struct PlayMusicEvent {
    pub track_id: String,
    pub fade_in_duration: f32,
    pub loop_start: Option<f32>,
    pub loop_end: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct PlaySFXEvent {
    pub sfx_id: String,
    pub position: Option<(f32, f32)>,  // None = 2D (UI), Some = 3D spatial
    pub volume: f32,
}

#[derive(Clone, Debug)]
pub struct ChatMessageEvent {
    pub sender: String,
    pub message: String,
    pub message_type: ChatMessageType,
}

#[derive(Clone, Debug)]
pub enum ChatMessageType {
    System,
    PlayerChat,
    Companion,
    Party,
    Global,
    Mail,
}
```

### 8.3 Server Emulator

```rust
// network/server_emulator.rs

/// Simulates a remote game server running in-process
pub struct ServerEmulator {
    // Server state
    pub server_tick: u64,
    pub tick_rate: f32,                    // 20 ticks/second
    pub tick_timer: f32,
    pub uptime: f32,                       // Simulated server uptime

    // Connection simulation
    pub connection_state: ServerConnectionState,
    pub connect_progress: f32,             // 0.0 - 1.0
    pub simulated_latency: f32,            // 50-200ms random delay
    pub packet_loss_chance: f32,           // 0.0 - 0.05

    // World simulation
    pub online_players: Vec<PlayerNPC>,    // Emulated other players
    pub global_chat_messages: VecDeque<String>,
    pub server_events: VecDeque<String>,   // "Server-wide announcements"

    // Event bridge
    pub outbound_events: VecDeque<GameEvent>,  // Server → Client
    pub inbound_events: VecDeque<GameEvent>,   // Client → Server
}

impl ServerEmulator {
    /// Process one server tick
    pub fn tick(&mut self, dt: f32) {
        self.server_tick += 1;
        self.tick_timer += dt;

        if self.tick_timer >= 1.0 / self.tick_rate {
            self.tick_timer -= 1.0 / self.tick_rate;
            self.process_inbound_events();
            self.simulate_players();
            self.generate_chat();
            self.generate_server_events();
            self.send_outbound_events();
        }
    }

    /// Simulate other players moving around the world
    fn simulate_players(&mut self) {
        for player in &mut self.online_players {
            player.tick();
        }
    }

    /// Generate background NPC chat messages
    fn generate_chat(&mut self) {
        // Every 5-15 seconds, emit a simulated chat message
        // Messages are drawn from a pool of pre-written lines
    }

    /// Apply simulated latency to event delivery
    fn apply_latency(&self, event: GameEvent) -> DelayedEvent {
        DelayedEvent {
            event,
            deliver_at: self.uptime + self.simulated_latency,
        }
    }
}

pub struct PlayerNPC {
    pub name: String,
    pub class: ClassType,
    pub level: u32,
    pub position: (f32, f32),
    pub area: String,
    pub online: bool,
    pub last_seen: f32,
    pub is_corrupted: bool,              // Story-relevant transformation
    pub patrol_path: Vec<(f32, f32)>,
}
```

### 8.4 Simulated Network Flow

```
Client                          Server Emulator
  │                                    │
  │  ConnectRequest                    │
  │───────────────────────────────────▶│
  │                                    │  Validate version
  │                                    │  Generate connection_id
  │                                    │  Start player sync
  │                                    │
  │  ConnectResponse(conn_id)          │
  │◀───────────────────────────────────│
  │                                    │
  │  Send CharacterData                │
  │───────────────────────────────────▶│
  │                                    │  Validate character
  │                                    │  Load save data
  │                                    │  Broadcast PlayerJoined
  │                                    │
  │  WorldState(entities, area)        │
  │◀───────────────────────────────────│
  │                                    │
  │  ─── Gameplay Loop ───             │
  │                                    │
  │  PlayerAction(move_to)             │
  │───────────────────────────────────▶│
  │                                    │  Validate position
  │                                    │  Update server state
  │                                    │
  │  EntityUpdate(id, pos, anim)       │
  │◀───────────────────────────────────│
  │                                    │
  │  PlayerAction(use_skill)           │
  │───────────────────────────────────▶│
  │                                    │  Validate cooldown
  │                                    │  Calculate damage
  │                                    │  Broadcast combat event
  │                                    │
  │  CombatResult(damage, effects)     │
  │◀───────────────────────────────────│
  │                                    │
  │  ─── Disconnect ───                │
  │                                    │
  │  DisconnectRequest                 │
  │───────────────────────────────────▶│
  │                                    │  Save state
  │                                    │  Broadcast PlayerLeft
  │                                    │
  │  DisconnectConfirmed               │
  │◀───────────────────────────────────│
```

---

## 9. Testing Strategy

### 9.1 Test Pyramid

```
          ╱╲
         ╱  ╲              Manual Playtesting
        ╱    ╲
       ╱ E2E  ╲            Integration tests (tests/)
      ╱────────╲
     ╱          ╲
    ╱ Integration╲          Module integration tests
   ╱──────────────╲
  ╱                ╲
 ╱   Unit + Prop    ╲       Unit tests per module + property-based
╱────────────────────╲
╱   Compile-time       ╲    Type system, cargo check, clippy
╱──────────────────────╲
```

### 9.2 Unit Tests

Every module has a `#[cfg(test)] mod tests` block at the bottom of the source file.

```rust
// components/position.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_addition() {
        let a = Position { x: 10.0, y: 20.0 };
        let b = Velocity { x: 5.0, y: -3.0, max_speed: 100.0 };
        let result = Position {
            x: a.x + b.x,
            y: a.y + b.y,
        };
        assert_eq!(result.x, 15.0);
        assert_eq!(result.y, 17.0);
    }
}
```

```rust
// systems/combat.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_calculation() {
        let attacker_stats = Stats {
            strength: 10,
            attack: 25,
            ..Default::default()
        };
        let defender_stats = Stats {
            vitality: 8,
            defense: 15,
            ..Default::default()
        };
        let damage = calculate_damage(&attacker_stats, &defender_stats, 1.0);
        // Expected: base_damage * (attack / defense) * multiplier
        assert!(damage > 0);
        assert!(damage < 100); // Sanity check
    }

    #[test]
    fn test_skill_cooldown() {
        let mut skill = Skill {
            cooldown: 5.0,
            current_cooldown: 0.0,
            ..Default::default()
        };
        skill.activate();
        assert!(skill.current_cooldown > 0.0);
        skill.tick(3.0);
        assert!(skill.current_cooldown > 0.0);
        skill.tick(2.1);
        assert_eq!(skill.current_cooldown, 0.0);
    }
}
```

### 9.3 Integration Tests

Integration tests in `tests/` directory test full scenarios across multiple systems.

```rust
// tests/integration/combat_tests.rs
use shambala::*;

#[test]
fn test_player_vs_goblin_combat_flow() {
    let mut world = World::new();
    let mut app = TestApp::new(&mut world);

    // Spawn player entity
    let player = app.spawn_player(ClassType::TwinBlade, Position { x: 0.0, y: 0.0 });

    // Spawn goblin enemy
    let goblin = app.spawn_enemy("goblin", Position { x: 50.0, y: 0.0 });

    // Run physics — player approaches goblin
    app.send_input(GameAction::MoveRight);
    app.update(2.0); // 2 seconds of movement

    // Player should be closer to goblin
    let player_pos = world.get::<Position>(player).unwrap();
    assert!(player_pos.x > 100.0);

    // Use basic attack
    app.send_input(GameAction::BasicAttack);
    app.update(0.5);

    // Goblin should have taken damage
    let goblin_health = world.get::<Health>(goblin).unwrap();
    assert!(goblin_health.current_hp < goblin_health.max_hp);
}

#[test]
fn test_data_drain_complete_cycle() {
    let mut world = World::new();
    let mut app = TestApp::new(&mut world);

    let player = app.spawn_player(ClassType::Wavemaster, Position { x: 0.0, y: 0.0 });
    let enemy = app.spawn_enemy("skeleton", Position { x: 30.0, y: 0.0 });

    // Fill data drain gauge by defeating enemies
    app.kill_entity(enemy);
    app.update(0.1);

    let drain = world.get::<DataDrain>(player).unwrap();
    assert!(drain.gauge > 0.0);

    // Activate data drain
    app.send_input(GameAction::DataDrain);
    app.update(0.1);

    // Minigame phase should be active
    let drain = world.get::<DataDrain>(player).unwrap();
    assert!(drain.minigame_active);

    // Complete minigame successfully
    app.complete_drain_minigame(true);
    app.update(0.5);

    // Should have received a Virus Core or Memory Fragment
    let inventory = world.get::<Inventory>(player).unwrap();
    let has_reward = inventory.slots.iter().any(|slot| {
        matches!(slot, Some(ItemData { item_type: ItemType::VirusCore, .. }))
    });
    assert!(has_reward);
}
```

### 9.4 Property-Based Testing

Using [`proptest`](https://crates.io/crates/proptest) for randomized testing of procedural systems.

```rust
// tests/property/area_generation.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_area_generation_always_produces_valid_map(
        seed in any::<u64>(),
        terrain in prop_oneof![
            Just(TerrainType::Forest),
            Just(TerrainType::Desert),
            Just(TerrainType::Ice),
            Just(TerrainType::Volcano),
            Just(TerrainType::Ruins),
            Just(TerrainType::Void),
        ],
        difficulty in prop_oneof![
            Just(DifficultyLevel::Easy),
            Just(DifficultyLevel::Normal),
            Just(DifficultyLevel::Hard),
            Just(DifficultyLevel::Hell),
        ],
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(terrain, difficulty);

        // All generated areas must have:
        assert!(area.map_width >= 50 && area.map_width <= 200);
        assert!(area.map_height >= 50 && area.map_height <= 200);
        assert!(area.entrance.is_some(), "Area must have an entrance");
        assert!(area.exit.is_some(), "Area must have an exit");
        assert!(!area.tiles.is_empty(), "Area must have tiles");

        // Entrance and exit must be within bounds
        let (ex, ey) = area.entrance.unwrap();
        assert!(ex < area.map_width);
        assert!(ey < area.map_height);

        let (xx, xy) = area.exit.unwrap();
        assert!(xx < area.map_width);
        assert!(xy < area.map_height);

        // At least some walkable tiles must exist
        let walkable = area.tiles.iter().filter(|&&t| t == 0).count();
        assert!(walkable > (area.tiles.len() / 4),
                "At least 25% of tiles must be walkable");
    }

    #[test]
    fn test_damage_calculation_is_bounded(
        attack in 1..999u32,
        defense in 0..999u32,
        multiplier in 0.1f32..10.0,
    ) {
        let stats = Stats { attack, ..Default::default() };
        let def_stats = Stats { defense, ..Default::default() };
        let damage = calculate_damage(&stats, &def_stats, multiplier);

        // Damage must always be at least 1
        assert!(damage >= 1,
            "Damage must be at least 1, got {} with attack={} def={} mult={}",
            damage, attack, defense, multiplier);

        // Damage should not exceed reasonable bounds
        let max_possible = (attack as f32 * multiplier * 2.0) as u32;
        assert!(damage <= max_possible,
            "Damage {} exceeds max possible {}",
            damage, max_possible);
    }

    #[test]
    fn test_path_between_entrance_and_exit(
        seed in any::<u64>(),
    ) {
        let generator = AreaGenerator::new(seed);
        let area = generator.generate(TerrainType::Forest, DifficultyLevel::Normal);

        // A path must exist between entrance and exit
        let path = find_path(
            &area.tiles,
            area.map_width,
            area.map_height,
            area.entrance.unwrap(),
            area.exit.unwrap(),
        );
        assert!(path.is_some(), "A path must exist between entrance and exit");
        // Path should not be unreasonably long
        let path = path.unwrap();
        let map_area = (area.map_width * area.map_height) as f32;
        assert!((path.len() as f32) < map_area * 0.8,
                "Path should not cover >80% of map area");
    }
}
```

### 9.5 Benchmark Tests

Using [`criterion`](https://crates.io/crates/criterion) for performance regression tracking.

```rust
// benches/render_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sprite_batching_benchmark(c: &mut Criterion) {
    c.bench_function("sprite_batch_500_sprites", |b| {
        b.iter_batched(
            || {
                // Setup: 500 sprite entities with random positions
                let sprites: Vec<SpriteInstance> = (0..500).map(|i| {
                    SpriteInstance {
                        position: [fastrand::f32() * 1000.0, fastrand::f32() * 1000.0],
                        size: [32.0, 32.0],
                        source_rect: [0.0, 0.0, 32.0, 32.0],
                        color: [1.0, 1.0, 1.0, 1.0],
                        flip: [1.0, 1.0],
                    }
                }).collect();
                (sprites, SpriteBatcher::new())
            },
            |(sprites, mut batcher)| {
                black_box(batcher.batch(&sprites));
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, sprite_batching_benchmark);
criterion_main!(benches);
```

### 9.6 Test Coverage Requirements

| Module | Minimum Coverage | Critical Paths |
|---|---|---|
| `components/` | 90% | Serialization round-trips, default constructors |
| `systems/combat.rs` | 95% | Damage calc, crits, healing, status effects, death |
| `systems/physics.rs` | 90% | Movement, collision, wall sliding, queries |
| `systems/ai.rs` | 85% | Behavior transitions, patrol path following |
| `systems/data_drain.rs` | 95% | Gauge charge, activation, minigame, results |
| `procgen/` | 90% | Valid map generation, path existence, seeding |
| `data/` | 90% | Config deserialization, validation, error handling |
| `render/` | 70% | Pipeline creation, buffer updates (complex to unit test) |
| `ui/` | 60% | Widget layout, event propagation |
| `network/` | 85% | Event serialization, server state consistency |

---

## 10. Performance Targets

### 10.1 Frame Rate & Timing

| Metric | Target | Measurement Method |
|---|---|---|
| **Locked Frame Rate** | 60 FPS (16.67ms per frame) | `wgpu` present timing |
| **Frame Budget — Input** | < 0.5ms | System timers |
| **Frame Budget — Physics** | < 3ms | System timers |
| **Frame Budget — AI** | < 2ms | System timers |
| **Frame Budget — Combat** | < 1ms | System timers |
| **Frame Budget — Render (CPU)** | < 4ms | System timers |
| **Frame Budget — Render (GPU)** | < 6ms | GPU query timestamps |
| **Frame Budget — UI** | < 2ms | System timers |
| **Frame Budget — Audio** | < 0.5ms | System timers |
| **Frame Budget — Misc/Overhead** | < 3ms | System timers |
| **Total Budget** | 16.67ms | Sum of all budgets |

### 10.2 Memory Budget

| Category | Budget | Details |
|---|---|---|
| **Textures (GPU)** | < 150 MB | Spritesheets, tilesets, UI elements |
| **Audio Buffers** | < 100 MB | Music (streaming), SFX (preloaded) |
| **ECS World** | < 50 MB | Entities, components, resources |
| **Asset Cache** | < 50 MB | Metadata, configs, deserialized data |
| **Physics World** | < 30 MB | rapier2d colliders, rigid bodies, grid |
| **UI State** | < 20 MB | Layout, text buffers, atlases |
| **Network State** | < 10 MB | Player list, event queues, chat log |
| **Misc (Particles, etc.)** | < 40 MB | Particle buffers, temporary allocations |
| **Save Data** | < 5 MB | Per-save file size |
| **Total (RAM)** | < 450 MB | Sum of all categories |
| **Total (VRAM)** | < 250 MB | GPU-dedicated resources |

### 10.3 Loading Times

| Operation | Target | Optimization Strategy |
|---|---|---|
| **Game Boot** | < 3 seconds | Load critical assets only, defer non-critical |
| **Area Transition** | < 2 seconds | Asset streaming, loading screen with tips |
| **Save Game Load** | < 1 second | Binary RON format, no parsing overhead |
| **Save Game Write** | < 0.5 seconds | Async write via `tokio::task::spawn_blocking` |
| **Settings Apply** | Instant | Hot-reload config, no restart needed |
| **Texture Load** | < 0.1s per texture | Pre-converted RGBA, no runtime format conversion |
| **Area Generation** | < 1 second | Seeded RNG, template-based, no pathfinding at gen time |

### 10.4 Entity & Draw Call Limits

| Metric | Target | Notes |
|---|---|---|
| **Max Active Entities** | 500 | Combined enemies, NPCs, party, items, particles |
| **Max Particle Systems** | 32 | Each system can have up to 200 particles |
| **Max Draw Calls (Total)** | < 100 | Batched as much as possible |
| **Max Draw Calls (Sprites)** | < 50 | Assuming 1024 instances per batch |
| **Max Draw Calls (Tilemap)** | 6 | 4 tile layers + 2 for animated tiles |
| **Max Draw Calls (UI)** | < 20 | One batch per texture atlas |
| **Max Lights** | 16 | 2D point lights with distance culling |
| **Max Physics Bodies** | 300 | Dynamic + static combined |
| **Max AI Agents** | 50 | Active behavior tree evaluations per frame |

### 10.5 Optimization Techniques

```rust
// 1. Sprite Atlas: Pack multiple sprites into a single atlas texture
//    to reduce texture binds and draw calls.
//
// 2. Instance Rendering: Use wgpu instance buffers for all sprites
//    sharing the same texture. One draw call per unique texture.
//
// 3. Frustum Culling: Skip sprites outside camera viewport.
//    Use fast AABB vs camera frustum check.
//
// 4. Tilemap LOD: Render distant tiles at lower resolution
//    (merge 2x2 tiles into one quad beyond threshold).
//
// 5. Component Storage: Use SoA (Struct of Arrays) layout via
//    bevy_ecs Table storage. Avoid Vec<Component> iteration.
//
// 6. Query Filters: Use bevy_ecs With/Without filters to
//    minimize component access per system.
//
// 7. Event Pooling: Pre-allocate event objects to reduce
//    allocation pressure during combat.
//
// 8. Parallel Systems: Run independent systems (Audio, AI, Physics)
//    in parallel using bevy_ecs parallel executor.
//
// 9. Asset Streaming: Load area assets in background during
//    gameplay using a separate thread and atomic progress flag.
//
// 10. Object Pooling: Reuse particle entities, projectile entities,
//     and temporary effect entities via object pool resource.
```

---

## 11. Dependencies

### 11.1 Complete Cargo.toml

```toml
[package]
name = "shambala"
version = "0.1.0"
edition = "2024"
description = "A story-driven action RPG inspired by .hack//sign, built in Rust with ECS architecture."
authors = ["Shambala Dev Team"]
license = "MIT"
repository = "https://github.com/shambala-game/shambala"
readme = "README.md"

[lib]
name = "shambala_core"
path = "src/lib.rs"

[[bin]]
name = "shambala"
path = "src/main.rs"

[dependencies]
# ── ECS Framework ──
bevy_ecs = { version = "0.14", features = ["bevy_reflect"] }
# Alternative: hecs = "0.11" (lighter weight, if bevy_ecs is too heavy)

# ── Graphics & Rendering ──
wgpu = "22.0"                         # Modern GPU abstraction (Vulkan/DX12/Metal)
winit = "0.30"                        # Window creation, event loop
pixels = "0.13"                       # Pixel buffer for 2D rendering
image = "0.25"                        # Image loading (PNG, JPEG)
guillotiere = "0.6"                   # Texture atlas packing

# ── Windowing & Input ──
gilrs = "0.10"                        # Gamepad input

# ── Physics ──
rapier2d = "0.23"                     # 2D physics engine
parry2d = "0.23"                      # 2D collision detection (rapier's math crate)

# ── Audio ──
kira = { version = "0.10", features = ["ogg"] }      # Audio playback with OGG support
rodio = "0.20"                                        # Alternative audio playback (Sprint 3)

# ── Serialization & Data ──
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"                    # JSON support for debug utilities
ron = "0.8"                           # Rusty Object Notation (game data files)
toml = "0.8"                          # TOML for user-facing settings
bincode = "1.3"                       # Binary serialization for save files

# ── Async Runtime ──
tokio = { version = "1", features = ["rt-multi-thread", "fs", "io-util"] }

# ── Procedural Generation ──
noise = "0.9"                         # Perlin/Simplex noise for terrain generation
rand = { version = "0.8", features = ["std_rng"] }   # Seeded RNG

# ── AI & Behavior Trees ──
# Custom behavior tree implementation (no external crate needed)

# ── Utilities ──
log = "0.4"                            # Logging facade
env_logger = "0.11"                    # Log implementation
anyhow = "1.0"                         # Error handling
thiserror = "2.0"                      # Derive Error trait
rayon = "1.10"                         # Parallel iteration for asset loading
uuid = { version = "1.0", features = ["v4"] }  # Unique IDs
crossbeam-channel = "0.5"             # Event bus channels
hashlink = "0.9"                      # LinkedHashMap for LRU caches
once_cell = "1.19"                    # Lazy statics
dashmap = "6.0"                       # Concurrent HashMap for asset cache
paste = "1.0"                         # Macro utilities
derivative = "2.2"                    # Custom derive helpers
strum = { version = "0.26", features = ["derive"] }  # Enum iteration

# ── Time & Profiling ──
instant = "0.1"                       # Cross-platform time
tracing = "0.1"                       # Structured diagnostics
tracing-subscriber = "0.3"            # Tracing output

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
fastrand = "2.1"                      # Fast RNG for benchmarks (not cryptographic)

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
# Native-only dependencies
fs_extra = "1.3"                      # Filesystem utilities for save management
notify = "6.1"                        # File watcher for hot-reload assets

[profile.dev]
opt-level = 1                         # Faster dev builds with some optimization
debug = true                          # Full debug info
incremental = true                    # Faster recompilation
codegen-units = 256                   # More parallel compilation

[profile.release]
opt-level = 3                         # Max optimization
debug = false                         # No debug info
strip = "symbols"                     # Remove debug symbols
lto = "fat"                           # Link-time optimization
codegen-units = 1                     # Single codegen unit for better optimization
panic = "abort"                       # Smaller binary, no unwinding
overflow-checks = false               # Disable overflow checks in release

[profile.test]
opt-level = 0
debug = true
incremental = true

[features]
default = ["audio", "physics"]
audio = ["kira"]                      # Enable audio support
physics = ["rapier2d"]               # Enable physics engine
profiling = []                        # Enable performance profiling tools
debug_render = []                     # Enable debug rendering overlays
headless = []                         # Run without graphics (CI testing)
editor = []                           # In-game editor mode (dev tool)
```

### 11.2 Feature Flags

| Feature | Default | Description |
|---|---|---|
| `audio` | Yes | Enable audio playback via kira. Disable for headless CI runs. |
| `physics` | Yes | Enable rapier2d physics. Disable for editor/level design mode. |
| `profiling` | No | Enable `tracing` spans for all systems. Impacts performance. |
| `debug_render` | No | Render collision boxes, nav mesh, and debug overlays. |
| `headless` | No | No graphics or audio. Run integration/benchmark tests. |
| `editor` | No | Enable in-game editor for area layouts and dialogue. |

### 11.3 MSRV (Minimum Supported Rust Version)

**Rust edition 2024**, requiring **Rust 1.85+** or later.

The `edition = "2024"` in `Cargo.toml` indicates the project uses the Rust 2024 edition features including:
- `impl Trait` in return position
- `gen` blocks (if stabilized)
- Enhanced `match` ergonomics
- Better `unsafe` hygiene

---

## Appendix A: Key Design Decisions

| Decision | Rationale | Alternative Considered |
|---|---|---|
| **bevy_ecs over hecs** | Proven in production, parallel scheduler, query filters, change detection | `hecs` / `specs` / `legion` |
| **wgpu over OpenGL** | Cross-platform Vulkan/DX12/Metal, future-proof, safer API | `gfx-rs` / `rend3` / `pixels` |
| **rapier2d over ncollide** | Full physics pipeline (rigid bodies, joints, sensors), active maintenance | `ncollide2d` / `nphysics2d` / `heron` |
| **kira over rodio** | Better spatial audio, mixer groups, effects (reverb, pitch) | `rodio` / `oddio` |
| **RON for game data** | Rust-friendly syntax, serde integration, human-readable | `JSON` / `YAML` / `TOML` |
| **TOML for settings** | Standard Rust config format, less verbose than RON for user editing | `RON` / `JSON` |
| **Local server emulation** | No networking complexity while delivering MMO feel | Full client-server / P2P |
| **Custom AI behavior trees** | Full control, no external dependency, tailored to game needs | `bonsai-bt` / `behave` |
| **Fixed timestep game loop** | Determinism, stable physics, easier debugging | Variable timestep |
| **Template-based procgen** | Quality control over pure procedural, hand-authored templates | Pure noise-based generation |

---

## Appendix B: Glossary

| Term | Definition |
|---|---|
| **AABB** | Axis-Aligned Bounding Box — rectangular collision volume |
| **ECS** | Entity-Component-System — data-oriented architecture pattern |
| **LOD** | Level of Detail — reducing render complexity at distance |
| **MSRV** | Minimum Supported Rust Version |
| **RON** | Rusty Object Notation — serde-compatible data format |
| **SAP** | Sweep-and-Prune — broad-phase collision detection algorithm |
| **SoA** | Struct of Arrays — memory layout optimizing cache usage |
| **TDD** | Technical Design Document (this document) |
| **wgpu** | WebGPU implementation in Rust; cross-platform graphics API |

---

> **Document Status:** Draft v1.0
> **Next Steps:** Sprint 3 implementation in progress — asset pipeline, audio system, character select, combat visualization, save/load, simulated network layer.

---

## 14. Sprint 2 Implementation Plan

### Task 2.1: wgpu/winit Setup (8 SP)
- Create render module structure
- Implement window creation with winit
- Set up wgpu instance, surface, device, queue
- Create swap chain and render pipeline
- Tests: Window creation, pipeline compilation

### Task 2.2: Game Loop (5 SP)
- Implement event loop with winit
- Frame timing with fixed timestep
- Input event handling
- Resize handling
- Tests: Frame timing, input processing

### Task 2.3: Sprite Rendering (8 SP)
- Implement SpriteBatch
- Texture loading and atlas management
- Layer-based sorting and rendering
- Animation system
- Tests: Sprite batching, layer sorting, animation

### Task 2.4: Title Screen (5 SP)
- Title screen scene
- Menu options (New Game, Continue, Options, Quit)
- Basic UI rendering
- Tests: Menu navigation

### Task 2.5: Chaos Gate (8 SP)
- Area transition system
- Keyword selection UI
- Area generation on transition
- Loading screen
- Tests: Area transitions, keyword validation

### Task 2.6: Quest System & Dialogue (8 SP)
- Quest data structures
- Quest tracking and progression
- Dialogue trees
- NPC interaction
- Tests: Quest lifecycle, dialogue flow

---

## 15. Sprint 3 Implementation Plan

### Task 3.1: Asset Pipeline (8 SP)
- Implement texture loading from PNG files using `image` crate
- Create texture atlas management system
- Implement async asset loading with progress tracking
- Add asset caching and hot-reloading support
- Tests: Texture loading, atlas packing, cache hit/miss

### Task 3.2: Audio System (5 SP)
- Implement rodio-based audio playback
- BGM streaming with crossfade support
- SFX playback with spatial audio
- Volume control per audio type
- Tests: Audio playback, volume control, mute toggle

### Task 3.3: Character Select Screen (5 SP)
- Class selection UI with 4 classes
- Character preview with stats display
- Name input field
- Confirm/cancel navigation
- Tests: Class selection, name validation, state transitions

### Task 3.4: Combat Visualization (8 SP)
- Attack animations (slash, magic, etc.)
- Damage number popups
- HP/MP bar animations
- Status effect indicators
- Screen shake on heavy hits
- Tests: Animation timing, damage display, effect rendering

### Task 3.5: Save/Load System (5 SP)
- Serialize game state with serde/ron
- Save file management (create, load, delete)
- Auto-save on area transitions
- Save slot selection UI
- Tests: Save/load roundtrip, data integrity, error handling

### Task 3.6: Simulated Network Layer (5 SP)
- Simulated MMO server connection
- Event-based network messages
- Latency simulation
- Disconnect/reconnect handling
- Tests: Message serialization, connection states, event routing
