# Shambala — Game Design Document

> **Version:** 1.0  
> **Status:** Draft  
> **Last Updated:** 2026-05-11  
> **Engine:** Custom (Rust, ECS, 2D Top-Down)  
> **Platform:** PC (Windows / Linux / macOS)

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
| **Language** | Rust (edition 2024) | Performance, safety, memory control |
| **ECS Framework** | Custom (or `bevy_ecs` / `hecs`) | Data-oriented design, cache-friendly |
| **Rendering** | `wgpu` (Vulkan/DX12/Metal) | Cross-platform, modern GPU API |
| **Audio** | `kira` or `rodio` | Lightweight, Rust-native audio |
| **Input** | `winit` + `gilrs` | Window management + gamepad support |
| **Serialization** | `serde` + `ron` | Save files, config, data assets |
| **Physics** | `rapier2d` | Collision detection, spatial queries |
| **Testing** | Built-in `#[test]` + `criterion` | TDD approach, benchmarks |

### 7.2 ECS Architecture

The project already follows an ECS (Entity-Component-System) structure:

```
src/
├── main.rs                  # Entry point, game loop
├── components/              # Data components
│   ├── mod.rs
│   ├── player.rs            # Player-specific data
│   ├── enemy.rs             # Enemy AI data
│   ├── position.rs          # Transform, velocity
│   ├── health.rs            # HP, SP, status effects
│   ├── combat.rs            # Attack, defense, skills
│   ├── party.rs             # Party membership, bond level
│   ├── data_drain.rs        # Data Drain gauge, corruption
│   └── render.rs            # Sprite, animation, visibility
├── entities/                # Entity factories
│   ├── mod.rs
│   ├── player.rs            # Spawn player entity
│   ├── companion.rs         # Spawn companion entities
│   ├── enemy.rs             # Spawn enemy entities
│   └── item.rs              # Spawn item/chest entities
├── resources/               # Global singleton resources
│   ├── mod.rs
│   ├── game_state.rs        # GameState enum, transition logic
│   ├── input.rs             # Input mapping, buffered actions
│   ├── time.rs              # Game clock, delta time
│   ├── asset_manager.rs     # Texture, audio, font loading
│   └── world.rs             # Area generation, keyword state
└── systems/                 # System logic
    ├── mod.rs
    ├── input.rs             # Read input, dispatch actions
    ├── movement.rs          # Apply velocity, collision
    ├── combat.rs            # Damage calculation, skill effects
    ├── ai.rs                # Companion and enemy AI
    ├── data_drain.rs        # Data Drain charge, activation, results
    ├── rendering.rs         # Draw sprites, UI, particles
    ├── audio.rs             # Play music, SFX
    └── ui.rs                # HUD, menus, chat log
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

---

## 8. Development Roadmap

### 8.1 Development Philosophy

- **Test-Driven Development (TDD):** Write tests before implementation for all core systems
- **Agile Sprints:** 2-week sprints with clear deliverables
- **Iterative Prototyping:** Playable builds at the end of each milestone
- **Continuous Integration:** GitHub Actions for automated testing

### 8.2 Milestones

#### Milestone 0 — Project Setup (Sprint 1)

| Task | Description |
|---|---|
| Initialize Rust project with Cargo | Done |
| Set up ECS directory structure | Done |
| Configure CI pipeline | GitHub Actions, `cargo test`, `cargo clippy` |
| Set up `wgpu` rendering window | Window creation, clear color |
| Implement basic game loop | Fixed timestep, delta time |

#### Milestone 1 — Core Engine (Sprints 2–3)

| Task | Description |
|---|---|
| ECS framework integration | Register components, systems, resources |
| Input system | Keyboard + mouse input, key rebinding |
| Movement system | Velocity, acceleration, collision with `rapier2d` |
| Camera system | Follow player, smooth interpolation |
| Tilemap rendering | Load and render Tiled `.tmx` maps |
| Sprite animation | Spritesheet loading, frame-based animation |

#### Milestone 2 — Player & Combat (Sprints 4–6)

| Task | Description |
|---|---|
| Player entity | Spawn, control, animation |
| Basic combat | Melee attack, damage calculation, HP system |
| Skill system | Skill definitions, cooldowns, SP cost |
| All 4 classes | Implement class-specific skills and stats |
| Enemy AI | Basic behavior tree (patrol, chase, attack) |
| Party system | Companion spawning, follow behavior, tactics |

#### Milestone 3 — Data Drain & Areas (Sprints 7–9)

| Task | Description |
|---|---|
| Data Drain gauge | Charge from kills, activation UI |
| Data Drain minigame | Rhythm/timing input, success/failure states |
| Data Drain results | Virus Cores, Memory Fragments, loot |
| Area keyword system | Keyword selection UI, procedural generation |
| Field zone generation | Tilemap generation from templates |
| Boss encounters | Multi-phase boss AI, Data Drain integration |

#### Milestone 4 — Story & Content (Sprints 10–14)

| Task | Description |
|---|---|
| Mac Anu (Root Town) | Full hub map, NPCs, shops, Chaos Gate |
| Delta Server | Tutorial area, first 5 field zones |
| Theta Server | Mid-game zones, first major story beat |
| Sigma Server | Late-game zones, Memory Fragment lore |
| Omega Server | Pre-final zones, Kether's influence |
| Core Server | Final dungeon, boss rush, ending sequence |
| Dialogue system | Branching dialogue, companion responses |
| Quest system | Quest tracking, objectives, rewards |

#### Milestone 5 — UI & Polish (Sprints 15–17)

| Task | Description |
|---|---|
| Main menu | Title screen, new game, load, settings |
| HUD | HP/SP bars, party frames, skill bar, minimap |
| Menu system | Status, party, items, data, system menus |
| Chat log | Scrollable log, colored text, mail system |
| Audio system | Music playback, SFX triggers, volume control |
| Save/load system | Serialize game state, multiple save slots |
| Accessibility features | Colorblind mode, text scaling, controller support |

#### Milestone 6 — Testing & Release (Sprints 18–20)

| Task | Description |
|---|---|
| Playtesting | Internal QA, bug tracking |
| Performance optimization | Profiling, asset optimization, draw call batching |
| Steam integration | Steamworks SDK, achievements, cloud saves |
| Localization | English (primary), Japanese (secondary) |
| Build pipeline | Automated builds for Windows, Linux, macOS |
| Beta release | Closed beta, feedback collection |
| Launch | v1.0 release on Steam / Itch.io |

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
| ECS complexity slows development | High | Start with `bevy_ecs` crate, migrate to custom if needed |
| Procedural generation feels repetitive | Medium | Hand-authored templates + seeded randomization |
| Story quality insufficient | High | Hire narrative designer, iterate on dialogue |
| Performance issues with 2D lighting | Medium | Use sprite batching, limit light sources |
| Scope creep | High | Strict sprint planning, MVP-first approach |

---

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

> **Document Status:** Draft v1.0  
> **Next Steps:** Review with team, refine mechanics, begin Milestone 0 implementation.