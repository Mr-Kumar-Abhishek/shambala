# Skill: ECS Patterns

## Description
Defines the Entity-Component-System (ECS) patterns used in the Shambala project. This skill covers how to define new Components, create Systems, define Resources, and register everything in the app builder using `bevy_ecs`.

## Prerequisites
- Familiarity with Rust and the `bevy_ecs` crate
- Understanding of ECS architecture (Entities, Components, Systems, Resources)
- Project structure follows [`src/components/`](../src/components/), [`src/systems/`](../src/systems/), [`src/resources/`](../src/resources/), [`src/entities/`](../src/entities/)

## Steps

### 1. Define a New Component

Components are plain data structs with the `#[derive(Component)]` macro. They should be pure data with no logic.

#### Naming Conventions
- File: `snake_case.rs` (e.g., `position.rs`, `data_drain.rs`)
- Struct: `PascalCase` (e.g., `Position`, `DataDrain`, `StatusEffects`)
- Enum: `PascalCase` (e.g., `DrainPhase`, `AIBehavior`)
- Fields: `snake_case` (e.g., `current_hp`, `attack_range`)

#### Basic Component
```rust
// src/components/position.rs
use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

/// World-space position in 2D.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
```

#### Tag Component (no data)
```rust
/// Marker component — no data, used for query filtering.
#[derive(Component)]
pub struct Player;
```

#### Enum Component
```rust
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum Facing {
    Up, Down, Left, Right,
    UpLeft, UpRight, DownLeft, DownRight,
}
```

#### Component with Complex Types
```rust
use std::collections::HashMap;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct StatusEffects {
    pub effects: Vec<StatusEffectInstance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusEffectInstance {
    pub effect_id: String,
    pub effect_type: StatusEffectType,
    pub remaining_duration: f32,
    pub tick_interval: f32,
    pub tick_timer: f32,
    pub magnitude: f32,
    pub source: String,
}
```

#### Required Derive Macros
| Macro | Purpose |
|---|---|
| `#[derive(Component)]` | Required for all ECS components |
| `Clone` | For cloning entities and components |
| `Debug` | For debug printing and test assertions |
| `Serialize, Deserialize` | For save/load functionality |
| `Default` | Optional, for quick construction in tests |

### 2. Create a New System

Systems are functions that operate on component data. They query the ECS world for entities with specific components and process them.

#### System Function Signature
```rust
// src/systems/movement.rs
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_ecs::world::World;

/// Systems can be plain functions.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x * time.delta;
        position.y += velocity.y * time.delta;
    }
}
```

#### Query Patterns
| Pattern | Description |
|---|---|
| `Query<&Position>` | Read-only access to a single component |
| `Query<&mut Position>` | Mutable access to a single component |
| `Query<(&Position, &Velocity)>` | Read multiple components |
| `Query<(&mut Position, &Velocity)>` | Mix of read and write |
| `Query<&Position, With<Player>>` | Filter entities with a tag component |
| `Query<&Position, Without<Enemy>>` | Exclude entities with a component |
| `Query<Entity, &Position>` | Include the Entity ID in the query |
| `Query<&Position, Or<(With<Player>, With<Companion>)>>` | Union filter |

#### System with Event Reading
```rust
use bevy_ecs::event::EventReader;

pub fn combat_event_system(
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.take_damage(event.damage);
        }
    }
}
```

#### System with Commands (Entity Spawning/Despawning)
```rust
use bevy_ecs::system::Commands;

pub fn death_system(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if !health.is_alive() {
            commands.entity(entity).despawn();
        }
    }
}
```

### 3. Define a Resource

Resources are singleton data structures accessible from any system.

```rust
// src/resources/game_state.rs
use bevy_ecs::system::Resource;

#[derive(Resource, Clone, Debug)]
pub struct GameState {
    pub current: GamePhase,
    pub previous: GamePhase,
    pub transition_timer: f32,
    pub paused: bool,
    pub tick_count: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current: GamePhase::Boot,
            previous: GamePhase::Boot,
            transition_timer: 0.0,
            paused: false,
            tick_count: 0,
        }
    }
}
```

#### Resource Access in Systems
```rust
// Read-only access
pub fn my_system(time: Res<Time>) { ... }

// Mutable access
pub fn my_system(mut game_state: ResMut<GameState>) { ... }

// Optional resource (may not exist)
pub fn my_system(audio: Option<Res<AudioManager>>) { ... }
```

### 4. Register Everything in the App Builder

Systems, resources, and events must be registered before they can be used.

```rust
// src/main.rs or src/lib.rs
use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
use bevy_ecs::world::World;
use shambala::*;

pub struct App {
    pub world: World,
    pub schedule: Schedule,
}

impl App {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut schedule = Schedule::default();

        // ── Register Resources ──
        world.insert_resource(GameState::new());
        world.insert_resource(Time::new());
        world.insert_resource(InputState::new());
        world.insert_resource(Camera::new());
        world.insert_resource(AssetManager::new());
        world.insert_resource(AudioManager::new());

        // ── Register Events ──
        world.insert_resource(Events::<DamageEvent>::default());
        world.insert_resource(Events::<DeathEvent>::default());
        world.insert_resource(Events::<SkillEvent>::default());

        // ── Register Systems in Stages ──
        let mut stage = SystemStage::parallel();

        // Stage order matters — see TECHNICAL_DESIGN.md Section 2.2.1
        stage.add_system(input_system);
        stage.add_system(ai_system);
        stage.add_system(physics_system);
        stage.add_system(combat_system);
        stage.add_system(data_drain_system);
        stage.add_system(party_system);
        stage.add_system(area_generation_system);
        stage.add_system(animation_system);
        stage.add_system(audio_system);
        stage.add_system(render_system);
        stage.add_system(ui_system);

        schedule.add_stage("update", stage);

        Self { world, schedule }
    }

    pub fn run(&mut self) {
        self.schedule.run(&mut self.world);
    }
}
```

### 5. Entity Factories

Entity spawning logic lives in [`src/entities/`](../src/entities/). Use factory functions to construct entities with all required components.

```rust
// src/entities/player.rs
use bevy_ecs::system::Commands;
use bevy_ecs::world::Entity;

pub fn spawn_player(
    commands: &mut Commands,
    class: ClassType,
    position: Position,
    stats: Stats,
) -> Entity {
    commands
        .spawn((
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
            Player, // Tag component
        ))
        .id()
}
```

## Examples

### Complete Example: Adding a New Component + System

```rust
// 1. Define the component
// src/components/stamina.rs
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32, // per second
}

impl Stamina {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen_rate: 10.0,
        }
    }

    pub fn consume(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }
}

// 2. Create the system
// src/systems/stamina.rs
pub fn stamina_regen_system(
    time: Res<Time>,
    mut query: Query<&mut Stamina>,
) {
    for mut stamina in query.iter_mut() {
        stamina.current = (stamina.current + stamina.regen_rate * time.delta)
            .min(stamina.max);
    }
}

// 3. Register in App
// world.insert_resource(Events::<StaminaEvent>::default());
// stage.add_system(stamina_regen_system);
```

### Query Filtering Example
```rust
/// Heal only party members (not enemies).
pub fn heal_party_system(
    mut health_query: Query<&mut Health, With<PartyMember>>,
    mut heal_events: EventReader<HealEvent>,
) {
    for event in heal_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.heal(event.amount);
        }
    }
}
```

## Related Skills
- [`combat-system.md`](combat-system.md) — Combat-specific ECS patterns
- [`testing-patterns.md`](testing-patterns.md) — How to test ECS systems
- [`rendering-pipeline.md`](rendering-pipeline.md) — Render system patterns
- [`../docs/TECHNICAL_DESIGN.md`](../docs/TECHNICAL_DESIGN.md) — Full ECS architecture documentation
