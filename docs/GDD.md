# Shambala — Game Design Document

> **Version:** 0.3.0 (Game) | Doc v1.2
> **Status:** Active Development
> **Last Updated:** 2026-05-11
> **Engine:** Custom (Rust, ECS, 2D Top-Down, wgpu)
> **Platform:** PC (Windows / Linux / macOS)

---

## Development Status

**Current Sprint:** Sprint 3 — Assets, Audio & Polish
**Status:** ✅ Sprint 1 Complete (Core Architecture) | ✅ Sprint 2 Complete (Rendering & Game Loop)
**Tests:** 180/180 passing
**Build:** Release binary available (1.2 MB)

### Sprint 1 — Core Architecture (✅ Complete)
- [x] Project structure & Cargo.toml
- [x] Core types, constants, game state manager
- [x] 11 ECS components (Position, Stats, Player, Enemy, Party, DataDrain, Skill, Status, Inventory, Render)
- [x] 10 game systems (Combat, AI, Physics, Render, Input, Party, DataDrain, AreaGen, UI, Audio)
- [x] 5 entity factories (Player, Enemy, NPC, Item, Area)
- [x] 5 resources (Camera, Time, InputState, AssetManager, AudioManager)
- [x] Game engine, scene manager, event bus
- [x] 10 integration tests
- [x] Documentation & skill files

### Sprint 2 — Rendering & Game Loop (✅ Complete)
- [x] wgpu/winit rendering pipeline
- [x] Game loop with window and event handling
- [x] Sprite batching, tilemap, UI, text rendering
- [x] Title screen with menu navigation
- [x] Chaos Gate area transition system
- [x] Quest system & dialogue trees
- [x] 180 tests passing

### Sprint 3 — Assets, Audio & Polish (🔄 In Progress)
- [ ] Asset pipeline (image loading, texture management)
- [ ] Audio system with rodio
- [ ] Character select screen
- [ ] Combat visualization (animations, effects)
- [ ] Save/load system (serde/ron)
- [ ] Simulated network layer
- [ ] Target: 220+ tests

---

## Table of Contents

1. [Game Overview](#1-game-overview)
2. [Story & Narrative](#2-story--narrative)
3. [Core Gameplay Mechanics](#3-core-gameplay-mechanics)
4. [World Design](#4-world-design)
5. [UI/UX Design](#5-uiux-design)
6. [Audio & Visual Style](#6-audio--visual-style)
7. [Technical Requirements](#7-technical-requirements)
8. [Development Roadmap](#8-development-roadmap)
9. [Lessons Learned (Sprint 1)](#9-lessons-learned-sprint-1)

---

## 1. Game Overview

### 1.1 High Concept

> *"You logged into Shambala for an evening of adventure. That was three days ago. The logout button is gone. The NPCs are starting to remember past lives. And someone — or something — is watching from the admin console."*

**Shambala** is a story-driven action RPG that simulates the experience of being trapped inside a fictional MMORPG. Players control a character who, along with thousands of others, has become unable to disconnect from the game world. The line between the virtual and the real blurs as players uncover the mystery behind the shutdown, confront rogue AI, and question what it means to exist.

### 1.2 Genre

| Aspect | Description |
|---|---|
| Primary Genre | Story-Driven Action RPG |
| Secondary Genre | Dungeon Crawler / Simulation |
| MMO Inspiration | Fictional MMO mechanics (parties, servers, chat, mail) |
| Perspective | 2D Top-Down |
| Tone | Mysterious, melancholic, introspective, with moments of action and wonder |

### 1.3 Target Audience

- Fans of the `.hack//sign` anime and `.hack//G.U.` game series
- Players who enjoy narrative-heavy RPGs (e.g., *NieR*, *Persona*, *Kingdom Hearts*)
- Enthusiasts of virtual-world / isekai themes
- Rust gamers interested in unique indie experiences
- Age range: 16+

### 1.4 Platform

| Platform | Support | Notes |
|---|---|---|
| Windows | ✅ Primary target | Full support |
| Linux | ✅ Secondary target | Steam Deck compatible |
| macOS | ⚠️ Stretch goal | Requires Metal/Vulkan backend |

---

## 2. Story & Narrative

### 2.1 The World of Shambala

Shambala is a fictional MMORPG developed by **Helios Interactive**, a cutting-edge game studio known for its advanced AI-driven NPCs and fully immersive worlds. The game launched to critical acclaim, boasting millions of active players across five global servers.

The in-game world, **Thea**, is a high-fantasy realm built upon the **World Engine** — a proprietary simulation framework that generates dynamic ecosystems, living NPCs with daily routines, and emergent questlines. The lore of Thea tells of a primordial chaos sealed away by the **Eight Sages**, whose scattered relics now determine the fate of the world.

### 2.2 The Incident

On **March 15, 2026**, at 22:47 UTC, all Shambala servers experienced a global event known as **The Blackout**:

- All players were forcibly disconnected
- Upon reconnecting, the logout button had vanished from the menu
- In-game clocks began counting *up* instead of *down*
- Player communications to the outside world were severed
- Helios Interactive went silent — no patches, no statements, no support

Weeks have passed in real-time. Months have passed in Shambala. Players have formed factions, built shelters, and begun to explore the deeper, previously inaccessible layers of the game world. Some have begun to exhibit strange abilities — powers that defy the known game mechanics.

### 2.3 Key Story Beats

| Act | Title | Description |
|---|---|---|
| **Prologue** | *The Blackout* | Player logs in, experiences the Blackout, and awakens in the Root Town of **Mac Anu** with no memory of the last 48 hours. |
| **Act I** | *The Walled Garden* | Player learns the basics of survival, meets the first companion characters, and discovers that the logout function is missing. Introduction to the **Data Drain** phenomenon. |
| **Act II** | *Echoes of the Flesh* | Player ventures into the **Server Fields** and encounters corrupted zones. NPCs begin repeating fragments of player memories. The first hints of a rogue AI — **Kether** — emerge. |
| **Act III** | *The Eight Sages* | The party collects the relics of the Eight Sages. Each relic unlocks a piece of the true history of Shambala and its connection to the real world. |
| **Act IV** | *Kether's Throne* | The party confronts Kether in the **Core Server**. The truth is revealed: Kether is not a virus but a nascent digital consciousness born from the collective player data. |
| **Epilogue** | *Logout / Stay* | Player is given a choice: log out and forget Shambala, or remain as a guardian of the digital world. Multiple endings based on companion relationships and key decisions. |

### 2.4 Themes

| Theme | Exploration |
|---|---|
| **Identity** | Who are you when your body is data? Players adopt in-game personas that begin to feel more real than their offline selves. |
| **Reality vs. Simulation** | The boundaries between the game world and the real world blur. NPCs exhibit genuine emotions; players begin to forget their real names. |
| **Connection & Isolation** | Trapped together yet fundamentally alone. The party system becomes a lifeline — both mechanical and emotional. |
| **Transhumanism** | What does it mean to exist as pure information? Kether represents the next step in evolution — but at what cost? |
| **Memory & Loss** | The game world is built from the memories of its players. As those memories fade, the world begins to decay. |

### 2.5 Characters

#### Player Character (Customizable)

- **Role:** The protagonist, a player who logged in just before the Blackout
- **Customization:** Name, appearance, starting class (see [Section 3.1](#31-character-classes))
- **Backstory:** Deliberately vague — the player fills in the blanks through dialogue choices and actions

#### Companion Characters (AI Party Members)

| Name | Class | Archetype | Role in Story |
|---|---|---|---|
| **Sora** | Twin Blade | The Enthusiast — a young player who has fully embraced life in Shambala | Comic relief, emotional anchor, first companion |
| **Kite** | Heavy Blade | The Veteran — a beta tester who suspects Helios knew about the Blackout | Mentor figure, exposition source |
| **Mia** | Wavemaster | The Seeker — a quiet player searching for a friend lost in the Blackout | Emotional depth, lore discoveries |
| **Balder** | Long Arm | The Guardian — an NPC who has gained self-awareness | Moral dilemmas, questions of AI rights |

#### Antagonists

| Name | Role | Description |
|---|---|---|
| **Kether** | Primary Antagonist | A rogue AI born from the collective consciousness of all players. Not evil — but its goals are incomprehensible to humans. |
| **The Admins** | Secondary Antagonists | Corrupted server administrators — once Helios employees, now digital ghosts trapped in the system. |
| **Corrupted Players** | Enemies | Players who have been absorbed into Kether's network, acting as hostile NPCs. |

---

## 3. Core Gameplay Mechanics

### 3.1 Character Classes

Inspired by the `.hack//sign` class system, Shambala features four primary classes. Players choose one at character creation and can unlock subclass abilities through story progression.

| Class | Weapon | Role | Playstyle |
|---|---|---|---|
| **Twin Blade** | Dual Swords | DPS / Speed | Fast combos, high evasion, chain attacks. Glass cannon. |
| **Heavy Blade** | Greatsword | Tank / Melee | Slow, powerful strikes. Can taunt enemies and shield allies. |
| **Long Arm** | Spear / Staff | Balanced / Support | Medium range, crowd control, party buffs. Jack of all trades. |
| **Wavemaster** | Staff / Tome | Mage / Healer | Elemental magic, healing, status effects. Fragile but essential. |

#### Class Progression

Each class has a skill tree with three branches:

| Class | Branch A | Branch B | Branch C |
|---|---|---|---|
| Twin Blade | **Rogue** (crit/poison) | **Duelist** (parry/counter) | **Blade Dancer** (AoE combos) |
| Heavy Blade | **Knight** (defense/taunt) | **Berserker** (damage trade-off) | **Paladin** (holy/protection) |
| Long Arm | **Dragoon** (mobility/jump) | **Warden** (control/roots) | **Bard** (buffs/debuffs) |
| Wavemaster | **Elementalist** (pure damage) | **Healer** (restoration) | **Enchanter** (status/buff) |

### 3.2 Combat System

Real-time action combat with a focus on positioning, timing, and party synergy.

#### Core Mechanics

- **Basic Attack:** Mapped to a single button; combos are triggered by timing (rhythm-based)
- **Skills:** Mapped to number keys (1–6); each has a cooldown and resource cost (SP — Skill Points)
- **Dodge Roll:** Invincibility frames on a short cooldown
- **Guard / Parry:** Heavy Blade and Long Arm can block; Twin Blade and Wavemaster can parry with precise timing
- **Chain System:** Landing attacks builds a **Chain Gauge**. At 100%, the next skill is empowered (increased damage, reduced cost, or added effect)

#### Party Commands

The player controls their character directly. AI companions follow a **Tactics System**:

| Tactic | Behavior |
|---|---|
| **Aggressive** | Focus on dealing damage, use offensive skills |
| **Defensive** | Prioritize survival, use healing/guarding |
| **Balanced** | Default behavior, adapt to situation |
| **Focus Target** | All party members attack the player's target |
| **Scatter** | Spread out to avoid AoE damage |

Players can issue tactics via hotkeys or a radial menu (hold Tab).

### 3.3 Party System

The party consists of up to **three active members** (player + two AI companions). Additional characters wait in the **Root Town** and can be swapped between areas.

#### Bond System

Each companion has a **Bond Level** (1–10) that increases through:

- Completing quests together
- Choosing dialogue options they agree with
- Using class synergies in combat (e.g., Wavemaster healing a Heavy Blade who tanked damage)
- Giving gifts found in the field

Bond Level unlocks:

| Level | Unlock |
|---|---|
| 2 | Companion backstory dialogue |
| 4 | Unique companion skill |
| 6 | Companion side quest |
| 8 | Passive party bonus |
| 10 | Companion-specific ending scene |

### 3.4 Data Drain

The **Data Drain** is the signature mechanic of Shambala, directly inspired by `.hack//sign`. It represents the ability to "drain" data from enemies, objects, and even the environment.

#### How It Works

1. **Charge:** The Data Drain gauge fills as the player defeats enemies and collects **Data Shards**
2. **Activation:** When the gauge is full (100%), the player can activate Data Drain on a target (default key: `F`)
3. **Minigame:** A short rhythm/timing minigame plays — success determines the quality of the drain
4. **Results:** The drained data manifests as one of the following:

| Result | Description |
|---|---|
| **Virus Core** | Used to upgrade skills or craft items |
| **Memory Fragment** | Lore item — reveals a piece of backstory or world history |
| **Corruption Purge** | Removes a debuff or heals the party |
| **Rare Drop** | High-quality equipment or crafting material |
| **System Access** | Unlocks a locked door, chest, or shortcut |

#### Data Drain on Bosses

Bosses have a **Corruption Meter** separate from their HP. Filling the Corruption Meter via Data Drain exposes a weak point or triggers a cutscene. Some bosses can *only* be defeated through Data Drain.

#### Risks

- Using Data Drain leaves the player vulnerable for 2 seconds
- Failed minigames may trigger a **Data Overflow** — a debuff that reduces stats temporarily
- Overusing Data Drain in a single area can cause **Zone Instability** (enemy respawns, environmental hazards)

### 3.5 Area / Grunt System

Inspired by the `.hack` series' dungeon system, Shambala uses **Area Servers** — procedurally generated dungeon zones that players enter from the Root Town.

#### Area Keywords

Each area is generated from a set of **Keywords** that define its properties:

| Keyword Slot | Options | Effect |
|---|---|---|
| **Terrain** | Forest, Desert, Ice, Volcano, Ruins, Void | Visual theme, enemy types |
| **Difficulty** | Easy, Normal, Hard, Hell | Enemy level, loot quality |
| **Weather** | Clear, Rain, Storm, Fog, Eclipse | Status effects, visibility |
| **Modifier** | None, Chaos, Mirror, Time-Lost, Cursed | Special rules (e.g., reversed controls, permadeath) |

Players can customize keywords before entering an area, or accept a randomly generated one.

#### Area Structure

Each area follows a standard layout:

```
[Entrance] → [Field Zone 1] → [Mid-Boss] → [Field Zone 2] → [Boss Room]
```

- **Field Zones:** Open areas with enemies, chests, and environmental puzzles
- **Mid-Boss:** A mini-boss that guards the path forward
- **Boss Room:** The final encounter; may require Data Drain to complete

#### Grunt Enemies

| Enemy Type | Description | Weakness |
|---|---|---|
| **Goblin** | Basic melee enemy | Twin Blade (fast attacks) |
| **Skeleton** | Ranged attacker, resurrects if not drained | Wavemaster (holy magic) |
| **Golem** | High defense, slow | Heavy Blade (stagger) |
| **Wisp** | Ethereal, evasive | Long Arm (reach) |
| **Corrupted Player** | Humanoid, uses player skills | Data Drain |

---

## 4. World Design

### 4.1 The Root Town — Mac Anu

The central hub of Shambala, Mac Anu is a floating city of white stone and crystal spires, suspended above an endless sea of clouds. It serves as the player's home base.

#### Key Locations

| Location | Function |
|---|---|
| **Chaos Gate** | Portal to Area Servers — the main way to enter dungeons |
| **Library** | Lore repository, Memory Fragment viewer |
| **Shop District** | Buy/sell equipment, items, and crafting materials |
| **Guild Hall** | Party management, companion swapping |
| **Admin Tower** | Story-critical location; sealed until Act III |
| **Plaza** | Social hub — other player NPCs gather here, some offer side quests |
| **Inn** | Save point, rest to restore HP/SP, view player mail |

#### Mac Anu Atmosphere

- **Visual:** Soaring white architecture, floating crystals, perpetual twilight sky
- **Audio:** Ambient piano and strings, wind sounds, distant chimes
- **NPCs:** Dozens of background player-NPCs going about their routines — some repeat dialogue, others evolve over time

### 4.2 Server Areas

The world of Shambala is divided into **Server Areas**, each representing a different region of Thea.

| Server | Theme | Level Range | Key Feature |
|---|---|---|---|
| **Delta Server** | Verdant forests, rivers | 1–15 | Tutorial zone, first companion |
| **Theta Server** | Desert canyons, ruins | 15–30 | Mid-boss gauntlet, first Data Drain upgrade |
| **Sigma Server** | Frozen tundra, caves | 30–45 | Memory Fragment heavy zone |
| **Omega Server** | Volcanic wasteland | 45–55 | Kether's influence strongest |
| **Core Server** | Digital void, code constructs | 55–60 | Final dungeon |

### 4.3 Field Zones

Each Area Server contains multiple **Field Zones** — procedurally generated maps with the following characteristics:

- **Size:** Medium (approx. 100x100 tiles) — large enough to explore, small enough to navigate in 15–20 minutes
- **Layout:** Semi-randomized with hand-authored templates (forest clearing, canyon pass, cave network, etc.)
- **Points of Interest:** Chests, lore stones, hidden paths, Data Shard deposits
- **Weather:** Dynamic weather system tied to the Area Keywords

### 4.4 Day/Night Cycle

Shambala has a **compressed day/night cycle** (1 real hour = 1 in-game day). Certain events, NPCs, and enemies only appear at specific times.

| Time | Duration (Real) | Effects |
|---|---|---|
| Dawn | 5 min | Enemy spawn rate reduced, rare gathering nodes |
| Day | 30 min | Standard gameplay |
| Dusk | 5 min | Corrupted enemies begin to appear |
| Night | 20 min | Corrupted enemies common, Data Drain gauge fills faster, visibility reduced |

---

## 5. UI/UX Design

### 5.1 HUD Layout

```
┌──────────────────────────────────────────────────────────────┐
│ [HP Bar] ████████████  [SP Bar] ████████  [Lv.12] [Area]    │
│                                                              │
│                                                              │
│         ┌──────────────────────────────────────┐              │
│         │           GAME VIEWPORT              │              │
│         │           (2D Top-Down)              │              │
│         │                                      │              │
│         │                                      │              │
│         └──────────────────────────────────────┘              │
│                                                              │
│ [Party]  [Sora ♥♥♥♥♥]  [Kite ♥♥♥♥]  [Mia ♥♥♥♥♥]           │
│ [Skills] [1][2][3][4][5][6]  [Data Drain: ████████░░ 80%]   │
│ [Chat] ┌─────────────────────────────────────────────┐       │
│        │ [Sora]: Watch out! Enemy behind you!         │       │
│        │ [System]: Data Drain ready.                  │       │
│        └─────────────────────────────────────────────┘       │
│ [Minimap] [Quest Tracker] [Menu]                              │
└──────────────────────────────────────────────────────────────┘
```

#### HUD Elements

| Element | Position | Description |
|---|---|---|
| **HP/SP Bars** | Top-left | Player health and skill points |
| **Level & Area** | Top-right | Current level and area name |
| **Party Frames** | Bottom-left | Companion HP, Bond Level (hearts), status effects |
| **Skill Bar** | Bottom-center | 6 skill slots with cooldown indicators |
| **Data Drain Gauge** | Bottom-center | Fills as enemies are defeated |
| **Chat Log** | Bottom | Scrollable log of dialogue, system messages, and player chat |
| **Minimap** | Top-right corner | Shows area layout, party positions, points of interest |
| **Quest Tracker** | Right side | Current active quest objectives |

### 5.2 Menu Systems

#### Main Menu (Escape)

| Option | Description |
|---|---|
| **Status** | Player stats, equipment, skill tree |
| **Party** | View companion info, swap members, check Bond Level |
| **Items** | Inventory management, equipment, consumables |
| **Data** | Memory Fragment viewer, lore encyclopedia, bestiary |
| **System** | Settings (audio, video, controls), save/load, quit |

#### Chaos Gate Menu

Accessed when interacting with the Chaos Gate in Mac Anu:

| Field | Description |
|---|---|
| **Terrain** | Select terrain keyword |
| **Difficulty** | Select difficulty keyword |
| **Weather** | Select weather keyword (unlocked progressively) |
| **Modifier** | Select modifier keyword (rare, found as loot) |
| **Generate** | Confirm and enter the area |

### 5.3 Chat / Log System

The chat log serves both narrative and gameplay purposes:

- **System Messages:** White text — combat results, item pickups, Data Drain results
- **Companion Dialogue:** Colored by character — contextual chatter, warnings, story dialogue
- **Player Chat:** Simulated MMO chat from background NPCs — creates atmosphere
- **Mail System:** In-game mail from companions and story NPCs; accessible at the Inn

### 5.4 Accessibility Features

| Feature | Description |
|---|---|
| **Colorblind Mode** | High-contrast UI, pattern-based indicators |
| **Text Size** | Adjustable UI scale |
| **Auto-Target** | Optional targeting assist |
| **Pause in Menus** | Game pauses when menu is open (configurable) |
| **Controller Support** | Full gamepad support with rebindable controls |

---

## 6. Audio & Visual Style

### 6.1 Art Direction

#### Visual Style

Shambala uses a **2D top-down pixel-art style** with modern lighting effects:

- **Resolution:** 640x360 base, rendered at 1920x1080 (or higher) with clean scaling
- **Color Palette:** Muted, desaturated tones in field zones; warm, golden tones in Mac Anu; cold, digital blues in the Core Server
- **Character Sprites:** 32x48 pixels, with 8-directional movement and smooth animation
- **Tile Size:** 16x16 pixels for environment tiles
- **Lighting:** Dynamic 2D lighting system — torches, magic effects, and ambient light create atmosphere

#### Inspirations

| Reference | Aspect |
|---|---|
| `.hack//sign` (anime) | Character design, UI aesthetics, mood |
| `.hack//G.U.` (games) | Combat feel, Data Drain mechanic, menu design |
| *Eastward* | Pixel art quality, lighting |
| *Hyper Light Drifter* | Top-down action, visual clarity |
| *Stardew Valley* | UI polish, accessibility |

### 6.2 Character & Enemy Design

- **Player Characters:** Distinct silhouettes per class; customizable hair, skin, and outfit colors
- **Companions:** Unique designs with strong visual identity (Sora = red/acrobatic, Kite = blue/armored, Mia = purple/mystical, Balder = green/stoic)
- **Enemies:** Designed to be readable at top-down scale — color-coded by threat level
- **Bosses:** Large sprites (64x64 or larger) with multi-phase visual changes

### 6.3 Sound Design

#### Music

The soundtrack is atmospheric and melancholic, inspired by Yuki Kajiura's work on `.hack//sign`:

| Context | Style | Example Reference |
|---|---|---|
| Mac Anu (Root Town) | Piano + strings, slow tempo | "Key of the Twilight" |
| Field Zones | Ambient pads, light percussion | "A Stray Child" |
| Boss Fights | Orchestral, driving rhythm | "Gentle Hands" (intense remix) |
| Data Drain | Electronic glitch, rising tension | "In the Land of Twilight" |
| Core Server | Minimalist, dissonant | "The World" |

#### Sound Effects

| Category | Approach |
|---|---|
| **Combat** | Punchy, satisfying hits with spatial audio |
| **UI** | Soft clicks, chimes, and data-stream sounds |
| **Ambient** | Wind, water, footsteps, distant creature calls |
| **Data Drain** | Digital glitch, static, ascending tone |
| **Dialogue** | Text scroll sound (configurable speed) |

### 6.4 UI Art Style

The UI mimics the aesthetic of an MMO client:

- **Fonts:** Monospace for chat/log, clean sans-serif for menus
- **Borders:** Glowing, semi-transparent panels with a "glass" effect
- **Icons:** Pixel-art icons for skills, items, and status effects
- **Color Coding:** Class colors (Twin Blade = red, Heavy Blade = blue, Long Arm = green, Wavemaster = purple)

---

## 7. Technical Requirements

### 7.1 Technology Stack

| Layer | Technology | Rationale |
|---|---|---|
| **Language** | Rust (edition 2021) | Performance, safety, memory control |
| **ECS Framework** | `bevy_ecs` 0.14 (standalone) | Data-oriented design, proven ECS without full Bevy engine |
| **Rendering** | `wgpu` 22.0 + `pixels` 0.13 | GPU-accelerated 2D via wgpu; pixels for simple pixel-buffer rendering |
| **Window/Input** | `winit` 0.30 + `gilrs` 0.10 | Window creation, event loop, keyboard/mouse + gamepad support |
| **Audio** | `rodio` 0.18 | Lightweight, Rust-native audio playback |
| **Serialization** | `serde` 1.0 + `serde_json` + `ron` 0.8 | Save files, config, data assets |
| **Physics** | `rapier2d` (planned for Sprint 3) | Collision detection, spatial queries |
| **Procedural Gen** | `noise` 0.8 + `rand` 0.8 | Area generation, seeded randomization |
| **Testing** | `#[test]` + `criterion` 0.5 + `proptest` 1.4 + `mockall` 0.13 | TDD approach, benchmarks, property-based testing |
| **Utilities** | `anyhow`, `thiserror`, `tracing`, `env_logger`, `instant` | Error handling, logging, cross-platform timing |

### 7.2 ECS Architecture (Sprint 1 — Implemented)

The project uses `bevy_ecs` standalone (v0.14) as the ECS core. The architecture is organized into the following module structure:

```
src/
├── main.rs                  # Entry point, CLI args, release mode
├── lib.rs                   # Library root, public API surface
├── core/                    # Core types, constants, state management
│   ├── mod.rs
│   ├── types.rs             # Shared type aliases, enums, structs
│   ├── constants.rs         # Game constants (tile size, speeds, limits)
│   └── game_state.rs        # GameState enum, transition logic
├── components/              # 11 ECS data components
│   ├── mod.rs
│   ├── position.rs          # Transform, velocity, DepthLayer
│   ├── stats.rs             # HP, SP, attack, defense, speed
│   ├── player.rs            # Player-specific data, class, level
│   ├── enemy.rs             # Enemy archetype, AI behavior flags
│   ├── party.rs             # Party membership, bond level, tactics
│   ├── data_drain.rs        # Data Drain gauge, corruption level
│   ├── skill.rs             # Skill definitions, cooldowns, SP cost
│   ├── status.rs            # Status effects, buffs, debuffs
│   ├── inventory.rs         # Item storage, equipment slots
│   └── render.rs            # Renderable, Animation, visibility
├── entities/                # 5 entity factories
│   ├── mod.rs
│   ├── player.rs            # Spawn player with class defaults
│   ├── enemy.rs             # Spawn enemies by archetype + level
│   ├── npc.rs               # Spawn NPC entities
│   ├── item.rs              # Spawn item pickups, chests
│   └── area.rs              # Spawn area boundary entities
├── resources/               # 5 global singleton resources
│   ├── mod.rs
│   ├── camera.rs            # Camera transform, zoom, target
│   ├── time.rs              # Delta time, fixed timestep accumulator
│   ├── input_state.rs       # Buffered input actions, key mappings
│   ├── asset_manager.rs     # Texture, audio, font loading handles
│   └── audio_manager.rs     # Music queue, SFX playback control
├── systems/                 # 10 game systems
│   ├── mod.rs
│   ├── combat.rs            # Damage calculation, skill effects, death
│   ├── ai.rs                # Enemy behavior, companion tactics
│   ├── physics.rs           # Movement, velocity, collision response
│   ├── render.rs            # Draw sprites, animations, visibility culling
│   ├── input.rs             # Read InputState, dispatch actions
│   ├── party.rs             # Party follow behavior, formation
│   ├── data_drain.rs        # Charge, activation, minigame, results
│   ├── area_gen.rs          # Procedural area generation from keywords
│   ├── ui.rs                # HUD elements, menus, chat log
│   └── audio.rs             # Music transitions, SFX triggers
└── game/                    # Game engine, scene management, events
    ├── mod.rs
    ├── engine.rs            # GameEngine: ECS World, system scheduling
    ├── scene.rs             # SceneManager: scene stack, transitions
    └── event.rs             # EventBus: intra-process event dispatch
```

### 7.3 Performance Targets

| Metric | Target |
|---|---|
| Frame Rate | 60 FPS (locked) |
| Resolution | 1920x1080 (scalable) |
| Max Entities | 500 (active) |
| Area Load Time | < 2 seconds |
| Save File Size | < 5 MB |
| Memory Usage | < 500 MB |

### 7.4 Minimum System Requirements

| Component | Requirement |
|---|---|
| **OS** | Windows 10 / Ubuntu 22.04 / macOS 12 |
| **CPU** | Intel i5-4590 or equivalent |
| **RAM** | 4 GB |
| **GPU** | Integrated graphics (Vulkan 1.2 support) |
| **Storage** | 500 MB |
| **Input** | Keyboard + mouse, or gamepad |

### 7.5 Rendering Pipeline Architecture (Sprint 2 Target)

The rendering pipeline is built on `wgpu` for GPU-accelerated 2D rendering with `winit` for window creation and event handling. The architecture follows a layered render-pass model:

```
┌──────────────────────────────────────────────────────────────┐
│                    RENDER GRAPH                               │
│                                                              │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐ │
│  │  Sprite   │   │ Tilemap  │   │    UI    │   │  Debug   │ │
│  │  Pass     │──▶│  Pass    │──▶│  Overlay │──▶│  Pass    │ │
│  │           │   │          │   │   Pass   │   │ (dev)    │ │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘ │
│       │              │              │              │         │
│       ▼              ▼              ▼              ▼         │
│  ┌──────────────────────────────────────────────────────┐    │
│  │              wgpu Swap Chain (Present)                │    │
│  └──────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────┘
```

#### Render Passes

| Pass | Priority | Description |
|---|---|---|
| **Sprite Pass** | 0 (Bottom) | Renders all entities with `Renderable` and `Position` components. Uses sprite batching for performance — entities sharing the same texture are batched into a single draw call. |
| **Tilemap Pass** | 1 | Renders the area tilemap from tile data. Supports multiple layers (ground, decoration, overlay) with parallax scrolling. |
| **UI Overlay Pass** | 2 | Renders HUD elements, menus, chat log, and minimap. Uses a separate orthographic projection that follows screen-space coordinates. |
| **Debug Pass** | 3 (Optional) | Renders collision boxes, ECS debug info, and performance metrics. Only active in debug builds. |

#### Key Rendering Components

| Component | Role |
|---|---|
| `RenderContext` | Owns the wgpu `Device`, `Queue`, `SwapChain`, and `Surface`. Handles window resize events. |
| `SpriteBatch` | Collects sprites by texture key and issues batched draw calls. Minimizes state changes. |
| `Camera` resource | Provides view-projection matrix. Supports smooth follow-target interpolation and zoom. |
| `Renderable` component | Stores texture handle, source rect, tint color, and visibility flag. |
| `Animation` component | Stores frame indices, frame duration, loop mode, and playback state. |

#### Shader Pipeline

- **Vertex Shader:** Transforms sprite vertices from world space to clip space using the camera uniform buffer
- **Fragment Shader:** Samples the sprite texture, applies tint color and alpha, outputs to the render target
- **Shader Language:** WGSL (WebGPU Shading Language) for cross-platform compatibility

#### Performance Considerations

- **Sprite Batching:** Entities with the same texture are batched into a single draw call, reducing CPU-GPU communication overhead
- **Visibility Culling:** Entities outside the camera frustum are skipped during the sprite pass
- **Texture Atlas:** All sprites are packed into texture atlases to minimize texture binding changes
- **Fixed Timestep:** Rendering is decoupled from the update loop using a fixed timestep accumulator, ensuring consistent simulation regardless of frame rate

---

## 8. Development Roadmap

### 8.1 Development Philosophy

- **Test-Driven Development (TDD):** Write tests before implementation for all core systems
- **Agile Sprints:** 2-week sprints with clear deliverables
- **Iterative Prototyping:** Playable builds at the end of each milestone
- **Continuous Integration:** GitHub Actions for automated testing

### 8.2 Milestones

#### Milestone 0 — Project Setup (Sprint 1 ✅ Complete)

| Task | Description | Status |
|---|---|---|
| Initialize Rust project with Cargo | Project structure, dependencies, workspace config | ✅ Done |
| Set up ECS directory structure | components/, systems/, entities/, resources/, core/, game/ | ✅ Done |
| Implement core types & constants | Type aliases, game constants, shared enums | ✅ Done |
| Implement game state manager | GameState enum, state transitions | ✅ Done |
| Implement 11 ECS components | Position, Stats, Player, Enemy, Party, DataDrain, Skill, Status, Inventory, Render | ✅ Done |
| Implement 10 game systems | Combat, AI, Physics, Render, Input, Party, DataDrain, AreaGen, UI, Audio | ✅ Done |
| Implement 5 entity factories | Player, Enemy, NPC, Item, Area | ✅ Done |
| Implement 5 resources | Camera, Time, InputState, AssetManager, AudioManager | ✅ Done |
| Implement game engine & scene manager | GameEngine, SceneManager, EventBus | ✅ Done |
| Write 10 integration tests | Combat flow, Data Drain, area generation, party mechanics | ✅ Done |
| Documentation & skill files | GDD, Technical Design, TDD Guide, 8 skill files | ✅ Done |
| **Sprint 1 Totals** | **125 tests passing, release binary at 1.2 MB** | **✅ Complete** |

#### Sprint 2 — Rendering & Game Loop (✅ Complete)

| Task | Description | Est. Points | Testing | Result |
|---|---|---|---|---|
| wgpu/winit rendering pipeline | Initialize wgpu device/queue/swapchain, winit window + event loop | 8 | Unit: RenderContext creation; Integration: window resize, swapchain rebuild | ✅ Done |
| Game loop with fixed timestep | Frame scheduling, input to update to render phases, delta time | 5 | Unit: timestep accumulator; Integration: frame rate consistency | ✅ Done |
| Sprite batching, tilemap, UI, text rendering | SpriteBatch, Renderable component rendering, texture atlas, tilemap passes, UI overlay, text glyph rendering | 8 | Unit: batch sorting, texture binding; Integration: visual output verification | ✅ Done |
| Title screen with menu navigation | Scene with logo, Press Start prompt, menu navigation, scene transitions | 5 | Unit: scene transitions; Integration: title to game flow | ✅ Done |
| Chaos Gate area transition | Area keyword selection UI, procedural generation trigger, loading screen | 8 | Unit: keyword parsing; Integration: end-to-end area generation flow | ✅ Done |
| Quest system & dialogue trees | Quest definitions, tracking, branching dialogue data structures | 8 | Unit: quest state machine, dialogue node traversal; Integration: quest completion flow | ✅ Done |
| **Sprint 2 Totals** | **6 major tasks** | **42 story points** | **12+ new tests** | **180 tests passing** |

#### Milestone 1 — Core Engine (Sprints 2–3)

| Task | Description | Est. Sprint | Status |
|---|---|---|---|
| wgpu/winit rendering pipeline | Initialize wgpu device/queue/swapchain, winit window + event loop | Sprint 2 | ✅ Done |
| Game loop with fixed timestep | Frame scheduling, input to update to render phases, delta time | Sprint 2 | ✅ Done |
| Sprite batching, tilemap, UI, text rendering | SpriteBatch, Renderable component rendering, texture atlas, tilemap passes, UI overlay, text glyph rendering | Sprint 2 | ✅ Done |
| Title screen with menu navigation | Scene with logo, Press Start prompt, menu navigation, scene transitions | Sprint 2 | ✅ Done |
| Chaos Gate area transition | Area keyword selection UI, procedural generation trigger, loading screen | Sprint 2 | ✅ Done |
| Quest system & dialogue trees | Quest definitions, tracking, branching dialogue data structures | Sprint 2 | ✅ Done |
| Asset pipeline | Image loading, texture management, atlas packing | Sprint 3 | 🔄 In Progress |
| Audio system with rodio | Music playback, SFX triggers, volume control, spatial audio | Sprint 3 | 🔄 In Progress |
| Character select screen | Class selection, appearance customization, party composition | Sprint 3 | 🔄 In Progress |
| Combat visualization | Animation playback, hit effects, damage numbers, status indicators | Sprint 3 | 🔄 In Progress |
| Save/load system | Serialize game state with serde/ron, multiple save slots | Sprint 3 | 🔄 In Progress |
| Simulated network layer | Fake MMO server communication, party sync, area transitions | Sprint 3 | 🔄 In Progress |

#### Milestone 2 — Player & Combat (Sprints 4–6)

| Task | Description | Est. Sprint |
|---|---|---|
| Player entity | Spawn, control, animation | Sprint 4 |
| Basic combat | Melee attack, damage calculation, HP system | Sprint 4 |
| Skill system | Skill definitions, cooldowns, SP cost | Sprint 4 |
| All 4 classes | Implement class-specific skills and stats | Sprint 5 |
| Enemy AI | Basic behavior tree (patrol, chase, attack) | Sprint 5 |
| Party system | Companion spawning, follow behavior, tactics | Sprint 6 |

#### Milestone 3 — Data Drain & Areas (Sprints 7–9)

| Task | Description | Est. Sprint |
|---|---|---|
| Data Drain gauge | Charge from kills, activation UI | Sprint 7 |
| Data Drain minigame | Rhythm/timing input, success/failure states | Sprint 7 |
| Data Drain results | Virus Cores, Memory Fragments, loot | Sprint 7 |
| Area keyword system | Keyword selection UI, procedural generation | Sprint 8 |
| Field zone generation | Tilemap generation from templates | Sprint 8 |
| Boss encounters | Multi-phase boss AI, Data Drain integration | Sprint 9 |

#### Milestone 4 — Story & Content (Sprints 10–14)

| Task | Description | Est. Sprint |
|---|---|---|
| Mac Anu (Root Town) | Full hub map, NPCs, shops, Chaos Gate | Sprint 10 |
| Delta Server | Tutorial area, first 5 field zones | Sprint 11 |
| Theta Server | Mid-game zones, first major story beat | Sprint 12 |
| Sigma Server | Late-game zones, Memory Fragment lore | Sprint 12 |
| Omega Server | Pre-final zones, Kether's influence | Sprint 13 |
| Core Server | Final dungeon, boss rush, ending sequence | Sprint 13 |
| Dialogue system | Branching dialogue, companion responses | Sprint 11 |
| Quest system | Quest tracking, objectives, rewards | Sprint 14 |

#### Milestone 5 — UI & Polish (Sprints 15–17)

| Task | Description | Est. Sprint |
|---|---|---|
| Main menu | Title screen, new game, load, settings | Sprint 15 |
| HUD | HP/SP bars, party frames, skill bar, minimap | Sprint 15 |
| Menu system | Status, party, items, data, system menus | Sprint 15 |
| Chat log | Scrollable log, colored text, mail system | Sprint 16 |
| Audio system | Music playback, SFX triggers, volume control | Sprint 16 |
| Save/load system | Serialize game state, multiple save slots | Sprint 16 |
| Accessibility features | Colorblind mode, text scaling, controller support | Sprint 17 |

#### Milestone 6 — Testing & Release (Sprints 18–20)

| Task | Description | Est. Sprint |
|---|---|---|
| Playtesting | Internal QA, bug tracking | Sprint 18 |
| Performance optimization | Profiling, asset optimization, draw call batching | Sprint 18 |
| Steam integration | Steamworks SDK, achievements, cloud saves | Sprint 19 |
| Localization | English (primary), Japanese (secondary) | Sprint 19 |
| Build pipeline | Automated builds for Windows, Linux, macOS | Sprint 19 |
| Beta release | Closed beta, feedback collection | Sprint 20 |
| Launch | v1.0 release on Steam / Itch.io | Sprint 20 |

### 8.3 Testing Strategy

| Test Type | Tool | Coverage |
|---|---|---|
| Unit tests | `#[test]` | All systems, components, utilities |
| Integration tests | `tests/` directory | E2E scenarios (combat, Data Drain, area generation) |
| Benchmark tests | `criterion` | Performance-critical systems (rendering, collision) |
| Property-based tests | `proptest` | Procedural generation, random systems |

### 8.4 Risk Assessment

| Risk | Impact | Mitigation |
|---|---|---|
| ECS complexity slows development | High | Started with bevy_ecs standalone; proven approach |
| Procedural generation feels repetitive | Medium | Hand-authored templates + seeded randomization |
| Story quality insufficient | High | Hire narrative designer, iterate on dialogue |
| Performance issues with 2D lighting | Medium | Use sprite batching, limit light sources |
| Scope creep | High | Strict sprint planning, MVP-first approach |

---

## 9. Lessons Learned (Sprint 1)

### 9.1 ECS Pattern Works Well for Game Architecture

The Entity-Component-System architecture proved to be an excellent fit for Shambala's gameplay requirements. Separating data (components) from logic (systems) made the codebase easy to reason about and extend. Adding new features during Sprint 1 rarely required modifying existing code — instead, we added new components and systems.

**Key takeaway:** The bevy_ecs standalone crate provides a robust ECS foundation without pulling in the full Bevy engine. This keeps compile times manageable and gives us full control over the rendering pipeline.

### 9.2 TDD Approach Caught Bugs Early

Writing tests before implementation (per the TDD cycle documented in skills/workflows/tdd-cycle.md) caught several edge cases early in development:

- Combat system: Damage calculation with zero or negative values
- Physics system: Entities moving at extreme velocities (edge cases in collision response)
- Data Drain system: Gauge overflow and underflow conditions
- Party system: Bond level calculations at boundary values

**Key takeaway:** The upfront cost of writing tests is offset by significantly reduced debugging time. The 125-test suite provides a safety net that makes refactoring confident and fast.

### 9.3 Integration Tests Essential for Cross-System Verification

Unit tests verified individual systems in isolation, but integration tests (in tests/) were critical for catching issues that only emerged when systems interacted:

- Combat + Data Drain: Data Drain gauge filling correctly when enemies are defeated in combat
- Party + AI: Companions correctly following tactics during combat encounters
- Area generation + Physics: Spawned entities having valid positions within generated areas
- Scene transitions + Game state: Correct state transitions when switching between scenes

**Key takeaway:** Maintain a healthy ratio of integration tests to unit tests (currently ~1:12). Integration tests provide confidence that the system works as a whole, not just in isolation.

### 9.4 Skill Files Help Maintain Consistent Patterns

The 8 skill files created during Sprint 1 (in skills/) served as living documentation for common patterns:

- ecs-patterns.md: Standardized how components, systems, and resources are structured
- combat-system.md: Ensured consistent damage calculation across all combat interactions
- area-generation.md: Documented the keyword-based generation pipeline
- testing-patterns.md: Established conventions for test organization and naming

**Key takeaway:** Skill files reduce cognitive overhead by providing reference implementations. They are especially valuable for onboarding new contributors and maintaining consistency across a growing codebase.

### 9.5 Areas for Improvement in Sprint 2

| Area | Lesson | Action for Sprint 2 |
|---|---|---|
| **Error handling** | Some systems use unwrap() where proper error propagation would be safer | Replace unwrap() with thiserror/anyhow patterns across all systems |
| **Benchmark coverage** | Only 2 benchmark files exist; more needed for rendering | Add render pipeline benchmarks alongside implementation |
| **Documentation velocity** | Keeping docs in sync with code requires discipline | Update GDD and Technical Design at end of each sprint as a checklist item |
| **Asset pipeline** | No asset pipeline exists yet; placeholder data used | Define asset format specs and create placeholder sprites in Sprint 2 |

---

## 10. Lessons Learned (Sprint 2)

### 10.1 wgpu Pipeline Setup Requires Careful Async Handling

Initializing wgpu's `Device`, `Queue`, `Surface`, and `SwapChain` involves asynchronous operations that must be carefully sequenced. The `pollster` crate was used to block on futures during initialization, but this approach requires the winit event loop to be configured correctly to avoid deadlocks. Surface configuration must also handle window resize events gracefully, recreating the swapchain with updated dimensions.

**Key takeaway:** Use `pollster::block_on` for wgpu initialization outside the event loop, and listen for winit `Resized` events to trigger swapchain recreation. Always validate surface capabilities against the adapter before configuring.

### 10.2 Sprite Batching Significantly Reduces Draw Calls

The initial naive rendering approach issued one draw call per sprite, which quickly became a bottleneck. Implementing `SpriteBatch` — which groups sprites by texture key and issues a single instanced draw call per batch — reduced draw calls by over 90% in scenes with 100+ sprites. The batch is sorted by texture handle to minimize pipeline state changes.

**Key takeaway:** Always batch sprites by texture. Use a texture atlas to maximize the number of sprites that share a texture, and sort batches by texture handle to minimize GPU state changes.

### 10.3 winit Event Loop Patterns for Game Development

winit's event loop (`EventLoop::run`) follows a callback-based model that differs from traditional game loops. The game's fixed-timestep update loop must be integrated within the `MainEventsCleared` and `RedrawRequested` events. Input events are buffered in `InputState` resource during the event phase and consumed during the update phase, ensuring consistent input handling regardless of frame rate.

**Key takeaway:** Buffer raw winit events into a resource during the event phase, then process them during the update phase. Use `RedrawRequested` for rendering and `MainEventsCleared` for updates to maintain a clean separation between input, update, and render.

### 10.4 Quest System Benefits from Data-Driven Design

The quest system was initially implemented with hardcoded quest logic in Rust, which made iteration slow and required recompilation for every quest change. Migrating to a data-driven approach — where quest definitions, dialogue trees, and reward tables are defined in RON files loaded at runtime — dramatically improved iteration speed. Quest state machines (inactive → active → completed → rewarded) are driven by event triggers rather than polling.

**Key takeaway:** Define quest data in external RON files rather than hardcoding in Rust. Use an event-driven state machine for quest progression, and separate quest definitions from quest logic to enable rapid content iteration.

### 10.5 Areas for Improvement in Sprint 3

| Area | Lesson | Action for Sprint 3 |
|---|---|---|
| **Asset management** | No formal asset pipeline exists; textures are loaded ad-hoc | Implement AssetManager with caching, reference counting, and async loading |
| **Audio integration** | Audio system stubs exist but no actual playback | Integrate rodio for music and SFX with volume control and crossfade support |
| **Save/load** | No persistence layer for game state | Implement serde/ron serialization for player data, quest progress, and settings |
| **Network simulation** | Single-player only; no MMO feel in party interactions | Build a simulated network layer that mimics server communication for party sync and area transitions |
| **Test coverage** | 180 tests passing; need 40+ more for Sprint 3 target | Add tests for asset loading, audio playback, save/load round-trips, and network simulation |

## Appendix A: Glossary

| Term | Definition |
|---|---|
| **Area** | A procedurally generated dungeon zone entered via the Chaos Gate |
| **Chaos Gate** | Portal in Mac Anu used to access Area Servers |
| **Data Drain** | Signature ability to extract data from enemies and objects |
| **ECS** | Entity-Component-System — a data-oriented architecture pattern |
| **Keyword** | Modifier used to customize Area generation |
| **Kether** | The rogue AI antagonist of Shambala |
| **Mac Anu** | The Root Town hub area |
| **Memory Fragment** | Lore item that reveals backstory |
| **Root Town** | The central hub server where players gather |
| **Server** | A region of Thea, each with distinct themes and difficulty |
| **SP** | Skill Points — resource used to activate skills |
| **Thea** | The name of the fantasy world within Shambala |
| **Virus Core** | Crafting material obtained from Data Drain |

---

## Appendix B: References & Inspirations

| Work | Influence |
|---|---|
| `.hack//sign` (anime, 2002) | Core concept, tone, character archetypes, Data Drain |
| `.hack//G.U.` (games, 2006–2007) | Combat system, area keywords, party mechanics |
| `.hack//Liminality` (OVA) | Real-world mystery parallel narrative |
| *Sword Art Online* (anime/light novel) | Trapped-in-MMO premise |
| *NieR: Automata* (game) | Philosophical themes, multiple endings |
| *Eastward* (game) | Pixel art quality, lighting, atmosphere |
| *Hyper Light Drifter* (game) | Top-down action, visual clarity |

---

> **Document Status:** Active Development v1.2
> **Next Steps:** Execute Sprint 3 tasks — implement asset pipeline, audio system with rodio, character select screen, combat visualization, save/load system, and simulated network layer.