# Skill: Area Generation

## Description
Covers procedural area generation for Shambala, inspired by the `.hack//sign` keyword system. This includes noise-based terrain generation, room/corridor placement, enemy placement rules, and treasure/item placement.

## Prerequisites
- Understanding of [`AreaData`](../src/components/area.rs) component and keyword enums
- Familiarity with the procgen module structure at [`src/procgen/`](../src/procgen/)
- Basic knowledge of Perlin noise and BSP algorithms

## Steps

### 1. Keyword System

Areas are generated from four keyword slots that define properties:

```rust
// src/procgen/mod.rs

/// Complete keyword configuration for an area.
pub struct AreaKeywords {
    pub terrain: TerrainType,       // Visual theme and tile set
    pub difficulty: DifficultyLevel, // Enemy level range and loot quality
    pub weather: WeatherType,       // Environmental effects
    pub modifier: AreaModifier,     // Special gameplay rules
}

/// Generates an area from keyword configuration.
pub fn generate_area(
    keywords: &AreaKeywords,
    seed: u64,
    rng: &mut impl Rng,
) -> GeneratedArea {
    // 1. Create the terrain heightmap from noise
    let heightmap = generate_heightmap(seed, keywords.terrain, MAP_WIDTH, MAP_HEIGHT);

    // 2. Place rooms and corridors
    let rooms = place_rooms(rng, MAP_WIDTH, MAP_HEIGHT, &heightmap);

    // 3. Build the tilemap from terrain + rooms
    let tilemap = build_tilemap(&heightmap, &rooms, &keywords.terrain);

    // 4. Place enemies based on difficulty
    let enemy_spawns = place_enemies(rng, &rooms, &keywords.difficulty, &keywords.terrain);

    // 5. Place treasure and items
    let item_spawns = place_items(rng, &rooms, &keywords.difficulty);

    // 6. Apply weather and modifier effects
    let weather_effects = apply_weather(&keywords.weather);
    let modifier_rules = apply_modifier(&keywords.modifier);

    GeneratedArea {
        tilemap,
        enemy_spawns,
        item_spawns,
        entrance: rooms.first().map(|r| r.center()),
        exit: rooms.last().map(|r| r.center()),
        weather_effects,
        modifier_rules,
        markers: generate_markers(&rooms),
    }
}
```

### 2. Noise-Based Terrain Generation

Uses Perlin noise to create natural-looking terrain heightmaps.

```rust
// src/procgen/terrain.rs

use noise::{Perlin, Seedable};

const MAP_WIDTH: u32 = 100;
const MAP_HEIGHT: u32 = 100;

/// Generates a 2D heightmap using layered Perlin noise.
pub fn generate_heightmap(
    seed: u64,
    terrain: TerrainType,
    width: u32,
    height: u32,
) -> Vec<Vec<f32>> {
    let perlin = Perlin::new(seed);

    // Use multiple octaves for detail
    let octaves = &[(1.0, 0.02), (0.5, 0.05), (0.25, 0.1)];

    let mut heightmap = vec![vec![0.0_f32; height as usize]; width as usize];

    for x in 0..width {
        for y in 0..height {
            let mut value = 0.0;
            for (amplitude, frequency) in octaves {
                let nx = x as f64 * frequency;
                let ny = y as f64 * frequency;
                value += (perlin.get([nx, ny]) as f32) * amplitude;
            }
            // Normalize to 0.0–1.0
            value = (value + 1.0) / 2.0;
            heightmap[x as usize][y as usize] = value;
        }
    }

    // Apply terrain-specific transformations
    match terrain {
        TerrainType::Forest => apply_forest_modifiers(&mut heightmap),
        TerrainType::Desert => apply_desert_modifiers(&mut heightmap),
        TerrainType::Ice => apply_ice_modifiers(&mut heightmap),
        TerrainType::Volcano => apply_volcano_modifiers(&mut heightmap),
        TerrainType::Ruins => apply_ruins_modifiers(&mut heightmap),
        TerrainType::Void => apply_void_modifiers(&mut heightmap),
    }

    heightmap
}

/// Converts heightmap values to tile indices based on thresholds.
pub fn heightmap_to_tiles(
    heightmap: &[Vec<f32>],
    terrain: TerrainType,
) -> (Vec<u32>, Vec<bool>) {
    let width = heightmap.len();
    let height = heightmap[0].len();
    let mut tiles = Vec::with_capacity(width * height);
    let mut collision = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let h = heightmap[x][y];
            let (tile_index, is_wall) = match terrain {
                TerrainType::Forest => {
                    if h < 0.3 { (0, false) }       // Grass (walkable)
                    else if h < 0.5 { (1, false) }    // Dirt path
                    else if h < 0.7 { (2, true) }     // Tree
                    else { (3, true) }                  // Dense foliage (blocked)
                }
                TerrainType::Desert => {
                    if h < 0.2 { (4, false) }         // Sand
                    else if h < 0.5 { (5, false) }     // Hardened sand
                    else if h < 0.7 { (6, true) }      // Rock formation
                    else { (7, true) }                   // Canyon wall
                }
                TerrainType::Ice => {
                    if h < 0.3 { (8, false) }          // Ice floor
                    else if h < 0.5 { (9, false) }      // Snow
                    else if h < 0.6 { (10, true) }      // Ice wall
                    else { (11, true) }                   // Rock
                }
                TerrainType::Volcano => {
                    if h < 0.2 { (12, false) }          // Lava-safe rock
                    else if h < 0.4 { (13, false) }      // Ash floor
                    else if h < 0.6 { (14, true) }       // Lava (hazard)
                    else { (15, true) }                    // Obsidian wall
                }
                TerrainType::Ruins => {
                    if h < 0.3 { (16, false) }           // Stone floor
                    else if h < 0.5 { (17, false) }       // Cracked floor
                    else if h < 0.7 { (18, true) }        // Rubble
                    else { (19, true) }                     // Broken wall
                }
                TerrainType::Void => {
                    if h < 0.4 { (20, false) }            // Stable platform
                    else if h < 0.6 { (21, false) }        // Glowing path
                    else { (22, true) }                      // Void (instant death)
                }
            };
            tiles.push(tile_index);
            collision.push(is_wall);
        }
    }

    (tiles, collision)
}
```

### 3. Room and Corridor Placement

Uses Binary Space Partitioning (BSP) to place rooms, then connects them with corridors.

```rust
// src/procgen/room_placer.rs

use rand::Rng;

/// A rectangular room.
#[derive(Clone, Debug)]
pub struct Room {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Room {
    pub fn center(&self) -> (u32, u32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

/// Places rooms using BSP tree partitioning.
pub fn place_rooms(
    rng: &mut impl Rng,
    map_width: u32,
    map_height: u32,
    heightmap: &[Vec<f32>],
) -> Vec<Room> {
    let min_room_size = 5;
    let max_room_size = 12;

    // Start with one large partition (the whole map)
    let mut partitions = vec![(1, 1, map_width - 2, map_height - 2)];
    let mut rooms = Vec::new();

    // BSP splitting — split partitions recursively
    let mut i = 0;
    while i < partitions.len() {
        let (px, py, pw, ph) = partitions[i];

        // Decide if we split, and on which axis
        let split_horizontal = if pw > ph { rng.gen_bool(0.7) } else { rng.gen_bool(0.3) };

        if split_horizontal && pw >= min_room_size * 2 + 2 {
            // Split vertically
            let split_point = rng.gen_range(min_room_size + 1..pw.saturating_sub(min_room_size + 1));
            partitions.push((px, py, split_point, ph));
            partitions.push((px + split_point, py, pw - split_point, ph));
        } else if !split_horizontal && ph >= min_room_size * 2 + 2 {
            // Split horizontally
            let split_point = rng.gen_range(min_room_size + 1..ph.saturating_sub(min_room_size + 1));
            partitions.push((px, py, pw, split_point));
            partitions.push((px, py + split_point, pw, ph - split_point));
        } else {
            // Leaf partition — try to place a room
            let room_w = rng.gen_range(min_room_size..=pw.min(max_room_size));
            let room_h = rng.gen_range(min_room_size..=ph.min(max_room_size));
            let room_x = px + rng.gen_range(0..=pw.saturating_sub(room_w));
            let room_y = py + rng.gen_range(0..=ph.saturating_sub(room_h));

            rooms.push(Room {
                x: room_x,
                y: room_y,
                width: room_w,
                height: room_h,
            });
        }

        i += 1;
        // Cap iterations to prevent infinite loops
        if i > 50 { break; }
    }

    rooms
}

/// Connects rooms with L-shaped corridors.
pub fn connect_rooms(rooms: &[Room], tiles: &mut [u32], width: u32) {
    for i in 1..rooms.len() {
        let (ax, ay) = rooms[i - 1].center();
        let (bx, by) = rooms[i].center();

        // L-shaped corridor: go horizontal first, then vertical
        carve_hallway(tiles, width, ax, ay, bx, ay);
        carve_hallway(tiles, width, bx, ay, bx, by);
    }
}

fn carve_hallway(tiles: &mut [u32], width: u32, x1: u32, y1: u32, x2: u32, y2: u32) {
    let (start_x, end_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
    let (start_y, end_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

    for x in start_x..=end_x {
        for y in start_y..=end_y {
            let idx = (y * width + x) as usize;
            if idx < tiles.len() {
                tiles[idx] = 0; // Walkable floor tile
            }
        }
    }
}
```

### 4. Enemy Placement Rules

Enemies are placed based on room size, difficulty, and terrain type.

```rust
// src/procgen/enemy_placer.rs

use rand::Rng;

/// Enemy spawn point data.
pub struct EnemySpawn {
    pub x: u32,
    pub y: u32,
    pub enemy_id: String,
    pub level: u32,
    pub patrol_radius: f32,
}

/// Places enemies throughout the generated area.
pub fn place_enemies(
    rng: &mut impl Rng,
    rooms: &[Room],
    difficulty: &DifficultyLevel,
    terrain: &TerrainType,
) -> Vec<EnemySpawn> {
    let (min_enemies, max_enemies, level_boost) = match difficulty {
        DifficultyLevel::Easy => (1, 2, 0),
        DifficultyLevel::Normal => (2, 4, 1),
        DifficultyLevel::Hard => (3, 6, 3),
        DifficultyLevel::Hell => (4, 8, 5),
    };

    let mut spawns = Vec::new();
    let enemy_pool = enemy_pool_for_terrain(terrain);

    // Skip the first room (entrance) and last room (boss)
    for room in rooms.iter().skip(1).take(rooms.len().saturating_sub(2)) {
        let count = rng.gen_range(min_enemies..=max_enemies);
        for _ in 0..count {
            let x = rng.gen_range(room.x + 1..room.x + room.width - 1);
            let y = rng.gen_range(room.y + 1..room.y + room.height - 1);
            let enemy_id = enemy_pool[rng.gen_range(0..enemy_pool.len())];

            spawns.push(EnemySpawn {
                x,
                y,
                enemy_id: enemy_id.to_string(),
                level: 1 + level_boost,
                patrol_radius: 64.0,
            });
        }
    }

    spawns
}

/// Returns enemy types appropriate for each terrain.
fn enemy_pool_for_terrain(terrain: &TerrainType) -> &'static [&'static str] {
    match terrain {
        TerrainType::Forest => &["goblin", "wisp", "skeleton"],
        TerrainType::Desert => &["goblin", "golem", "skeleton"],
        TerrainType::Ice => &["skeleton", "wisp", "golem"],
        TerrainType::Volcano => &["golem", "corrupted_player", "wisp"],
        TerrainType::Ruins => &["skeleton", "golem", "corrupted_player"],
        TerrainType::Void => &["corrupted_player", "wisp", "golem"],
    }
}

/// Places a mid-boss in the transition room between field zones.
pub fn place_mid_boss(
    rng: &mut impl Rng,
    room: &Room,
    difficulty: &DifficultyLevel,
) -> EnemySpawn {
    let level_boost = match difficulty {
        DifficultyLevel::Easy => 1,
        DifficultyLevel::Normal => 3,
        DifficultyLevel::Hard => 5,
        DifficultyLevel::Hell => 8,
    };

    EnemySpawn {
        x: room.center().0,
        y: room.center().1,
        enemy_id: "mid_boss".to_string(),
        level: 5 + level_boost,
        patrol_radius: 128.0,
    }
}
```

### 5. Treasure and Item Placement

Items are placed in rooms based on loot tables and difficulty.

```rust
// src/procgen/loot_placer.rs

use rand::Rng;

/// Item spawn point data.
pub struct ItemSpawn {
    pub x: u32,
    pub y: u32,
    pub item_id: String,
    pub item_type: SpawnItemType,
    pub quantity: u32,
    pub rarity: ItemRarity,
}

pub enum SpawnItemType {
    Chest,
    Pickup,
    LoreStone,
    DataShard,
}

/// Places items throughout the area.
pub fn place_items(
    rng: &mut impl Rng,
    rooms: &[Room],
    difficulty: &DifficultyLevel,
) -> Vec<ItemSpawn> {
    let (chests_per_room, shards_per_room) = match difficulty {
        DifficultyLevel::Easy => (0, 1),
        DifficultyLevel::Normal => (1, 2),
        DifficultyLevel::Hard => (1, 3),
        DifficultyLevel::Hell => (2, 4),
    };

    let mut spawns = Vec::new();

    // Place chests in rooms (skip entrance room)
    for room in rooms.iter().skip(1) {
        for _ in 0..chests_per_room {
            let x = rng.gen_range(room.x + 2..room.x + room.width - 2);
            let y = rng.gen_range(room.y + 2..room.y + room.height - 2);

            // Determine chest loot rarity
            let rarity = roll_chest_rarity(rng, difficulty);

            spawns.push(ItemSpawn {
                x, y,
                item_id: format!("chest_{}", rarity),
                item_type: SpawnItemType::Chest,
                quantity: 1,
                rarity,
            });
        }

        // Place Data Shards
        for _ in 0..shards_per_room {
            let x = rng.gen_range(room.x + 1..room.x + room.width - 1);
            let y = rng.gen_range(room.y + 1..room.y + room.height - 1);
            spawns.push(ItemSpawn {
                x, y,
                item_id: "data_shard".to_string(),
                item_type: SpawnItemType::DataShard,
                quantity: rng.gen_range(1..=3),
                rarity: ItemRarity::Common,
            });
        }
    }

    // Always place a lore stone in the last room before boss
    if let Some(last_room) = rooms.last() {
        spawns.push(ItemSpawn {
            x: last_room.center().0,
            y: last_room.center().1,
            item_id: "lore_stone".to_string(),
            item_type: SpawnItemType::LoreStone,
            quantity: 1,
            rarity: ItemRarity::Uncommon,
        });
    }

    spawns
}

fn roll_chest_rarity(rng: &mut impl Rng, difficulty: &DifficultyLevel) -> ItemRarity {
    let roll: f32 = rng.gen();
    let (common, uncommon, rare, epic, legendary) = match difficulty {
        DifficultyLevel::Easy => (0.60, 0.30, 0.08, 0.02, 0.00),
        DifficultyLevel::Normal => (0.40, 0.35, 0.18, 0.06, 0.01),
        DifficultyLevel::Hard => (0.20, 0.30, 0.30, 0.15, 0.05),
        DifficultyLevel::Hell => (0.05, 0.20, 0.35, 0.25, 0.15),
    };

    if roll < common { ItemRarity::Common }
    else if roll < common + uncommon { ItemRarity::Uncommon }
    else if roll < common + uncommon + rare { ItemRarity::Rare }
    else if roll < common + uncommon + rare + epic { ItemRarity::Epic }
    else { ItemRarity::Legendary }
}
```

### 6. Modifier Effects

Area modifiers change gameplay rules.

```rust
// src/procgen/mod.rs

pub enum ModifierRules {
    None,       // Standard gameplay
    Chaos,      // Reversed controls, random screen effects
    Mirror,     // Map is flipped horizontally
    TimeLost,   // Timer is doubled, enemies speed up over time
    Cursed,     // No healing, HP drains slowly
}

pub fn apply_modifier(modifier: &AreaModifier) -> ModifierRules {
    match modifier {
        AreaModifier::None => ModifierRules::None,
        AreaModifier::Chaos => ModifierRules::Chaos,
        AreaModifier::Mirror => ModifierRules::Mirror,
        AreaModifier::TimeLost => ModifierRules::TimeLost,
        AreaModifier::Cursed => ModifierRules::Cursed,
    }
}
```

## Examples

### Generating a Complete Area
```rust
let keywords = AreaKeywords {
    terrain: TerrainType::Forest,
    difficulty: DifficultyLevel::Normal,
    weather: WeatherType::Rain,
    modifier: AreaModifier::None,
};

let seed = 42;
let mut rng = SeededRng::new(seed);
let area = generate_area(&keywords, seed, &mut rng);

// The area contains:
// - A tilemap (100x100 tiles) with forest terrain
// - 5-8 rooms connected by corridors
// - 10-20 enemies placed in non-entrance rooms
// - 3-5 chests with normal-difficulty loot
// - Rain weather effects
// - An entrance marker in the first room
// - An exit marker in the last room
```

## Related Skills
- [`ecs-patterns.md`](ecs-patterns.md) — ECS patterns for the AreaData component and area generation system
- [`combat-system.md`](combat-system.md) — Enemy placement feeds into combat encounters
- [`testing-patterns.md`](testing-patterns.md) — Property-based testing for procedural generation
- [`../docs/GDD.md`](../docs/GDD.md) — Area keyword system and zone design
- [`../docs/TECHNICAL_DESIGN.md`](../docs/TECHNICAL_DESIGN.md) — Procgen module structure
