use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssetManager {
    pub textures: HashMap<String, AssetInfo>,
    pub audio: HashMap<String, AssetInfo>,
    pub fonts: HashMap<String, AssetInfo>,
    pub configs: HashMap<String, String>,
    pub loading_queue: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub path: String,
    pub loaded: bool,
    pub size_bytes: u64,
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            audio: HashMap::new(),
            fonts: HashMap::new(),
            configs: HashMap::new(),
            loading_queue: Vec::new(),
        }
    }

    pub fn register_texture(&mut self, id: &str, path: &str) {
        self.textures.insert(
            id.to_string(),
            AssetInfo {
                path: path.to_string(),
                loaded: false,
                size_bytes: 0,
            },
        );
    }

    pub fn queue_loading(&mut self, asset_id: &str) {
        if !self.loading_queue.contains(&asset_id.to_string()) {
            self.loading_queue.push(asset_id.to_string());
        }
    }

    pub fn mark_loaded(&mut self, asset_id: &str) {
        if let Some(info) = self.textures.get_mut(asset_id) {
            info.loaded = true;
        }
        if let Some(info) = self.audio.get_mut(asset_id) {
            info.loaded = true;
        }
        self.loading_queue.retain(|id| id != asset_id);
    }

    pub fn is_loaded(&self, asset_id: &str) -> bool {
        self.textures.get(asset_id).is_some_and(|a| a.loaded)
            || self.audio.get(asset_id).is_some_and(|a| a.loaded)
    }

    pub fn loading_progress(&self) -> (usize, usize) {
        let total = self.textures.len() + self.audio.len();
        let loaded = self.textures.values().filter(|a| a.loaded).count()
            + self.audio.values().filter(|a| a.loaded).count();
        (loaded, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_texture() {
        let mut manager = AssetManager::new();
        manager.register_texture("player", "assets/sprites/player.png");
        assert!(manager.textures.contains_key("player"));
        assert!(!manager.is_loaded("player"));
    }

    #[test]
    fn test_loading_queue() {
        let mut manager = AssetManager::new();
        manager.register_texture("player", "assets/sprites/player.png");
        manager.queue_loading("player");
        assert_eq!(manager.loading_queue.len(), 1);
    }

    #[test]
    fn test_mark_loaded() {
        let mut manager = AssetManager::new();
        manager.register_texture("player", "assets/sprites/player.png");
        manager.mark_loaded("player");
        assert!(manager.is_loaded("player"));
    }

    #[test]
    fn test_loading_progress() {
        let mut manager = AssetManager::new();
        manager.register_texture("a", "a.png");
        manager.register_texture("b", "b.png");
        manager.mark_loaded("a");
        let (loaded, total) = manager.loading_progress();
        assert_eq!(loaded, 1);
        assert_eq!(total, 2);
    }
}
