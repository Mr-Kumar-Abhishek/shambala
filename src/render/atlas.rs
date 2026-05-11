use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AtlasRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub uv_x: f32,
    pub uv_y: f32,
    pub uv_w: f32,
    pub uv_h: f32,
}

pub struct TextureAtlas {
    pub regions: HashMap<String, AtlasRegion>,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub texture_id: String,
}

impl TextureAtlas {
    pub fn new(texture_id: &str, width: u32, height: u32) -> Self {
        Self {
            regions: HashMap::new(),
            atlas_width: width,
            atlas_height: height,
            texture_id: texture_id.to_string(),
        }
    }

    pub fn add_region(&mut self, id: &str, x: u32, y: u32, width: u32, height: u32) {
        let uv_x = x as f32 / self.atlas_width as f32;
        let uv_y = y as f32 / self.atlas_height as f32;
        let uv_w = width as f32 / self.atlas_width as f32;
        let uv_h = height as f32 / self.atlas_height as f32;

        self.regions.insert(id.to_string(), AtlasRegion {
            x, y, width, height,
            uv_x, uv_y, uv_w, uv_h,
        });
    }

    pub fn get_uv(&self, id: &str) -> Option<(f32, f32, f32, f32)> {
        self.regions.get(id).map(|r| (r.uv_x, r.uv_y, r.uv_w, r.uv_h))
    }

    pub fn get_region(&self, id: &str) -> Option<&AtlasRegion> {
        self.regions.get(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.regions.contains_key(id)
    }

    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    pub fn create_sprite_atlas() -> Self {
        let mut atlas = TextureAtlas::new("sprite_atlas", 512, 512);
        
        // Player sprites (4 classes, 32x32 each)
        let classes = ["TwinBlade", "HeavyBlade", "LongArm", "Wavemaster"];
        for (i, class) in classes.iter().enumerate() {
            atlas.add_region(
                &format!("player_{}", class),
                0, (i as u32) * 32, 32, 32,
            );
        }

        // Enemy sprites (5 types, 32x32 each)
        let enemies = ["Goblin", "Wolf", "Skeleton", "Mage", "Boss"];
        for (i, enemy) in enemies.iter().enumerate() {
            atlas.add_region(
                &format!("enemy_{}", enemy),
                64, (i as u32) * 32, 32, 32,
            );
        }

        // Tile sprites (8 types, 32x32 each)
        let tiles = ["floor", "wall", "water", "grass", "path", "entrance", "exit", "treasure"];
        for (i, tile) in tiles.iter().enumerate() {
            atlas.add_region(
                &format!("tile_{}", tile),
                128, (i as u32) * 32, 32, 32,
            );
        }

        atlas
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atlas_creation() {
        let atlas = TextureAtlas::new("test", 256, 256);
        assert_eq!(atlas.atlas_width, 256);
        assert_eq!(atlas.region_count(), 0);
    }

    #[test]
    fn test_add_region() {
        let mut atlas = TextureAtlas::new("test", 256, 256);
        atlas.add_region("player", 0, 0, 32, 32);
        assert!(atlas.contains("player"));
        assert_eq!(atlas.region_count(), 1);
    }

    #[test]
    fn test_uv_coordinates() {
        let mut atlas = TextureAtlas::new("test", 256, 256);
        atlas.add_region("sprite", 0, 0, 32, 32);
        let uv = atlas.get_uv("sprite").unwrap();
        assert!((uv.0 - 0.0).abs() < f32::EPSILON);  // uv_x
        assert!((uv.1 - 0.0).abs() < f32::EPSILON);  // uv_y
        assert!((uv.2 - 0.125).abs() < f32::EPSILON); // uv_w = 32/256
        assert!((uv.3 - 0.125).abs() < f32::EPSILON); // uv_h = 32/256
    }

    #[test]
    fn test_create_sprite_atlas() {
        let atlas = TextureAtlas::create_sprite_atlas();
        assert!(atlas.contains("player_TwinBlade"));
        assert!(atlas.contains("enemy_Goblin"));
        assert!(atlas.contains("tile_floor"));
        assert_eq!(atlas.region_count(), 17); // 4 players + 5 enemies + 8 tiles
    }

    #[test]
    fn test_get_nonexistent_region() {
        let atlas = TextureAtlas::new("test", 256, 256);
        assert!(!atlas.contains("nonexistent"));
        assert!(atlas.get_uv("nonexistent").is_none());
    }
}
