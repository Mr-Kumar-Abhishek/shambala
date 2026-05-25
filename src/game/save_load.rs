use crate::core::types::Class;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub timestamp: String,
    pub player_name: String,
    pub player_class: Class,
    pub player_level: u32,
    pub player_hp: u32,
    pub player_max_hp: u32,
    pub player_mp: u32,
    pub player_max_mp: u32,
    pub player_exp: u64,
    pub gold: u64,
    pub current_area: String,
    pub completed_quests: Vec<String>,
    pub active_quests: Vec<String>,
    pub play_time_seconds: f64,
}

impl SaveData {
    pub fn new(player_name: &str, player_class: Class) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: chrono_now(),
            player_name: player_name.to_string(),
            player_class,
            player_level: 1,
            player_hp: 100,
            player_max_hp: 100,
            player_mp: 30,
            player_max_mp: 30,
            player_exp: 0,
            gold: 0,
            current_area: "mac_anu".to_string(),
            completed_quests: Vec::new(),
            active_quests: Vec::new(),
            play_time_seconds: 0.0,
        }
    }
}

fn chrono_now() -> String {
    // Simple timestamp without external chrono crate
    "2024-01-01T00:00:00".to_string()
}

pub struct SaveManager {
    pub save_dir: PathBuf,
    pub current_save: Option<SaveData>,
    pub save_slots: Vec<Option<String>>, // Slot index -> save file name
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SaveManager {
    pub fn new() -> Self {
        let save_dir = dirs_path();
        let mut manager = Self {
            save_dir,
            current_save: None,
            save_slots: vec![None, None, None], // 3 save slots
        };
        manager.scan_saves();
        manager
    }

    pub fn save_to_slot(&mut self, slot: usize, data: &SaveData) -> Result<(), String> {
        if slot >= self.save_slots.len() {
            return Err("Invalid save slot".to_string());
        }

        std::fs::create_dir_all(&self.save_dir)
            .map_err(|e| format!("Failed to create save dir: {}", e))?;

        let filename = format!("save_{}.ron", slot);
        let path = self.save_dir.join(&filename);

        let ron_string = ron::to_string(data).map_err(|e| format!("Failed to serialize: {}", e))?;
        std::fs::write(&path, &ron_string).map_err(|e| format!("Failed to write save: {}", e))?;

        self.save_slots[slot] = Some(filename);
        self.current_save = Some(data.clone());
        log::info!("Game saved to slot {}", slot);
        Ok(())
    }

    pub fn load_from_slot(&mut self, slot: usize) -> Result<SaveData, String> {
        if slot >= self.save_slots.len() {
            return Err("Invalid save slot".to_string());
        }

        let filename = format!("save_{}.ron", slot);
        let path = self.save_dir.join(&filename);

        if !path.exists() {
            return Err("Save file does not exist".to_string());
        }

        let ron_string =
            std::fs::read_to_string(&path).map_err(|e| format!("Failed to read save: {}", e))?;
        let data: SaveData =
            ron::from_str(&ron_string).map_err(|e| format!("Failed to deserialize: {}", e))?;

        self.current_save = Some(data.clone());
        log::info!("Game loaded from slot {}", slot);
        Ok(data)
    }

    pub fn delete_slot(&mut self, slot: usize) -> Result<(), String> {
        if slot >= self.save_slots.len() {
            return Err("Invalid save slot".to_string());
        }

        let filename = format!("save_{}.ron", slot);
        let path = self.save_dir.join(&filename);

        if path.exists() {
            std::fs::remove_file(&path).map_err(|e| format!("Failed to delete save: {}", e))?;
        }

        self.save_slots[slot] = None;
        if self.current_save.is_some() {
            // Don't clear current_save if it exists
        }
        Ok(())
    }

    pub fn slot_has_data(&self, slot: usize) -> bool {
        if slot >= self.save_slots.len() {
            return false;
        }
        let filename = format!("save_{}.ron", slot);
        let path = self.save_dir.join(&filename);
        path.exists()
    }

    pub fn get_slot_info(&mut self, slot: usize) -> Option<String> {
        if !self.slot_has_data(slot) {
            return None;
        }
        if let Ok(data) = self.load_from_slot(slot) {
            Some(format!(
                "{} - Lv.{} {:?} ({}h)",
                data.player_name,
                data.player_level,
                data.player_class,
                (data.play_time_seconds / 3600.0) as u64,
            ))
        } else {
            Some("Corrupted save".to_string())
        }
    }

    fn scan_saves(&mut self) {
        for slot in 0..self.save_slots.len() {
            let filename = format!("save_{}.ron", slot);
            let path = self.save_dir.join(&filename);
            if path.exists() {
                self.save_slots[slot] = Some(filename);
            }
        }
    }

    pub fn auto_save(&mut self, data: &SaveData) -> Result<(), String> {
        self.save_to_slot(0, data) // Slot 0 is auto-save
    }

    pub fn has_auto_save(&self) -> bool {
        self.slot_has_data(0)
    }
}

fn dirs_path() -> PathBuf {
    let mut path = PathBuf::new();
    // Use a simple relative path for saves
    path.push("saves");
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// Create a SaveManager with a unique test directory to avoid
    /// file system contention when tests run in parallel.
    fn test_manager() -> SaveManager {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let save_dir = std::path::PathBuf::from(format!("test_saves_{}", id));
        let mut manager = SaveManager {
            save_dir,
            current_save: None,
            save_slots: vec![None, None, None],
        };
        manager.scan_saves();
        manager
    }

    /// Clean up the test save directory after each test.
    fn cleanup(manager: &SaveManager) {
        let _ = std::fs::remove_dir_all(&manager.save_dir);
    }

    #[test]
    fn test_save_data_creation() {
        let data = SaveData::new("Kite", Class::TwinBlade);
        assert_eq!(data.player_name, "Kite");
        assert_eq!(data.player_class, Class::TwinBlade);
        assert_eq!(data.player_level, 1);
    }

    #[test]
    fn test_save_manager_creation() {
        let manager = test_manager();
        assert_eq!(manager.save_slots.len(), 3);
        cleanup(&manager);
    }

    #[test]
    fn test_save_roundtrip() {
        let mut manager = test_manager();
        let data = SaveData::new("Kite", Class::TwinBlade);

        // Save
        assert!(manager.save_to_slot(1, &data).is_ok());
        assert!(manager.slot_has_data(1));

        // Load
        let loaded = manager.load_from_slot(1).unwrap();
        assert_eq!(loaded.player_name, "Kite");
        assert_eq!(loaded.player_class, Class::TwinBlade);

        // Cleanup
        let _ = manager.delete_slot(1);
        cleanup(&manager);
    }

    #[test]
    fn test_delete_slot() {
        let mut manager = test_manager();
        let data = SaveData::new("Test", Class::Wavemaster);
        manager.save_to_slot(2, &data).unwrap();
        assert!(manager.slot_has_data(2));
        manager.delete_slot(2).unwrap();
        assert!(!manager.slot_has_data(2));
        cleanup(&manager);
    }

    #[test]
    fn test_invalid_slot() {
        let mut manager = test_manager();
        assert!(manager
            .save_to_slot(99, &SaveData::new("T", Class::TwinBlade))
            .is_err());
        assert!(manager.load_from_slot(99).is_err());
        cleanup(&manager);
    }

    #[test]
    fn test_empty_slot() {
        let manager = test_manager();
        assert!(!manager.slot_has_data(1));
        cleanup(&manager);
    }

    #[test]
    fn test_auto_save() {
        let mut manager = test_manager();
        let data = SaveData::new("Kite", Class::TwinBlade);
        assert!(manager.auto_save(&data).is_ok());
        assert!(manager.has_auto_save());
        // Cleanup
        let _ = manager.delete_slot(0);
        cleanup(&manager);
    }

    #[test]
    fn test_slot_info() {
        let mut manager = test_manager();
        let data = SaveData::new("Kite", Class::TwinBlade);
        manager.save_to_slot(1, &data).unwrap();
        let info = manager.get_slot_info(1);
        assert!(info.is_some());
        assert!(info.unwrap().contains("Kite"));
        // Cleanup
        let _ = manager.delete_slot(1);
        cleanup(&manager);
    }

    #[test]
    fn test_empty_slot_info() {
        let mut manager = test_manager();
        assert!(manager.get_slot_info(1).is_none());
        cleanup(&manager);
    }
}
