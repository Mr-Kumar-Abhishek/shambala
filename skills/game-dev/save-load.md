# Skill: Save/Load System

## Description
How to implement game state serialization with `serde` and `ron` for the Shambala game.

## Prerequisites
- Understanding of `serde` (Serialize, Deserialize derives)
- Familiarity with `ron` (Rusty Object Notation) for human-readable save files
- Knowledge of `std::fs` and `std::path` for file management
- Understanding of the game state structure (components, resources)

## Steps

### 1. Define SaveData Struct
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete game state that gets serialized to disk.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveData {
    pub version: u32,
    pub timestamp: u64,
    pub player_name: String,
    pub player_class: String,
    pub level: u32,
    pub experience: u64,
    pub current_area: String,
    pub position_x: f32,
    pub position_y: f32,
    pub party_members: Vec<PartyMemberData>,
    pub inventory: Vec<ItemData>,
    pub quest_progress: Vec<QuestData>,
    pub completed_quests: Vec<String>,
    pub unlocked_areas: Vec<String>,
    pub play_time_seconds: u64,
    pub settings: SettingsData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartyMemberData {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub current_hp: u32,
    pub max_hp: u32,
    pub current_sp: u32,
    pub max_sp: u32,
    pub bond_level: u32,
    pub bond_xp: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemData {
    pub id: String,
    pub quantity: u32,
    pub equipped: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestData {
    pub quest_id: String,
    pub completed_objectives: Vec<String>,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsData {
    pub master_volume: f32,
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub muted: bool,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub fullscreen: bool,
}
```

### 2. Save to Disk with RON
```rust
use std::fs;
use std::path::{Path, PathBuf};

pub struct SaveManager {
    pub save_dir: PathBuf,
    pub current_slot: Option<usize>,
}

impl SaveManager {
    pub fn new(save_dir: PathBuf) -> Self {
        Self {
            save_dir,
            current_slot: None,
        }
    }

    /// Write save data to a numbered slot.
    pub fn save_to_slot(&self, slot: usize, data: &SaveData) -> Result<(), SaveError> {
        let path = self.save_dir.join(format!("save_{}.ron", slot));
        let ron_string = ron::to_string(data).map_err(|e| SaveError::Serialize(e.to_string()))?;
        fs::write(&path, ron_string).map_err(|e| SaveError::Io(e.to_string()))?;
        Ok(())
    }

    /// List all save slots with metadata.
    pub fn list_saves(&self) -> Result<Vec<SaveSlotInfo>, SaveError> {
        let mut saves = Vec::new();
        if !self.save_dir.exists() {
            return Ok(saves);
        }
        for entry in fs::read_dir(&self.save_dir).map_err(|e| SaveError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| SaveError::Io(e.to_string()))?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "ron") {
                if let Some(slot) = Self::parse_slot_number(&path) {
                    let metadata = fs::metadata(&path)
                        .map_err(|e| SaveError::Io(e.to_string()))?;
                    saves.push(SaveSlotInfo {
                        slot,
                        size_bytes: metadata.len(),
                        modified: metadata.modified().ok(),
                    });
                }
            }
        }
        saves.sort_by_key(|s| s.slot);
        Ok(saves)
    }

    fn parse_slot_number(path: &Path) -> Option<usize> {
        let stem = path.file_stem()?.to_str()?;
        let num = stem.strip_prefix("save_")?;
        num.parse::<usize>().ok()
    }
}

#[derive(Debug)]
pub enum SaveError {
    Serialize(String),
    Io(String),
    Corrupted(String),
}

#[derive(Debug)]
pub struct SaveSlotInfo {
    pub slot: usize,
    pub size_bytes: u64,
    pub modified: Option<std::time::SystemTime>,
}
```

### 3. Load from Disk with RON
```rust
impl SaveManager {
    /// Load save data from a numbered slot.
    pub fn load_from_slot(&self, slot: usize) -> Result<SaveData, SaveError> {
        let path = self.save_dir.join(format!("save_{}.ron", slot));
        if !path.exists() {
            return Err(SaveError::Io(format!("Save slot {} not found", slot)));
        }
        let content =
            fs::read_to_string(&path).map_err(|e| SaveError::Io(e.to_string()))?;
        let data: SaveData =
            ron::from_str(&content).map_err(|e| SaveError::Corrupted(e.to_string()))?;
        Ok(data)
    }
}
```

### 4. Handle Corrupted Saves Gracefully
```rust
impl SaveManager {
    /// Attempt to load, returning default data on corruption.
    pub fn load_or_default(&self, slot: usize, default: SaveData) -> SaveData {
        match self.load_from_slot(slot) {
            Ok(data) => data,
            Err(SaveError::Corrupted(msg)) => {
                eprintln!("Save slot {} corrupted ({}), using defaults", slot, msg);
                // Optionally back up the corrupted file
                let corrupted_path = self.save_dir.join(format!("save_{}.ron.corrupted", slot));
                let original_path = self.save_dir.join(format!("save_{}.ron", slot));
                let _ = fs::rename(&original_path, &corrupted_path);
                default
            }
            Err(e) => {
                eprintln!("Failed to load save slot {}: {:?}", slot, e);
                default
            }
        }
    }

    /// Delete a save slot.
    pub fn delete_slot(&self, slot: usize) -> Result<(), SaveError> {
        let path = self.save_dir.join(format!("save_{}.ron", slot));
        if path.exists() {
            fs::remove_file(&path).map_err(|e| SaveError::Io(e.to_string()))?;
        }
        Ok(())
    }
}
```

### 5. Auto-Save on Area Transitions
```rust
impl SaveManager {
    /// Auto-save triggered on area transitions or checkpoint triggers.
    pub fn auto_save(&self, data: &SaveData) -> Result<(), SaveError> {
        // Auto-save always goes to slot 0
        self.save_to_slot(0, data)
    }
}
```

## Testing
- **Serialization roundtrip:** Create SaveData, serialize to RON, deserialize back, verify all fields match
- **File management:** Save to a temp directory, list saves, verify slot info
- **Auto-save:** Call `auto_save()`, verify slot 0 exists and is readable
- **Corrupted file recovery:** Write garbage to a save file, call `load_or_default()`, verify graceful fallback
- **Delete slot:** Save to slot, delete it, verify file no longer exists
