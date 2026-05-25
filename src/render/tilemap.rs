use crate::systems::area_gen::{GeneratedArea, TileType};

#[derive(Debug, Clone)]
pub struct Tilemap {
    pub tiles: Vec<Vec<TileInfo>>,
    pub tile_size: f32,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct TileInfo {
    pub tile_type: TileType,
    pub sprite_id: String,
    pub walkable: bool,
}

impl Tilemap {
    pub fn from_generated_area(area: &GeneratedArea, tile_size: f32) -> Self {
        let tiles: Vec<Vec<TileInfo>> = area
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| {
                        let sprite_id = match tile.tile_type {
                            TileType::Floor => "tile_floor".to_string(),
                            TileType::Wall => "tile_wall".to_string(),
                            TileType::Water => "tile_water".to_string(),
                            TileType::Grass => "tile_grass".to_string(),
                            TileType::Path => "tile_path".to_string(),
                            TileType::Entrance => "tile_entrance".to_string(),
                            TileType::Exit => "tile_exit".to_string(),
                            TileType::Treasure => "tile_treasure".to_string(),
                        };
                        TileInfo {
                            tile_type: tile.tile_type,
                            sprite_id,
                            walkable: tile.walkable,
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            tiles,
            tile_size,
            width: area.width,
            height: area.height,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TileInfo> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    pub fn world_to_tile(&self, world_x: f32, world_y: f32) -> (usize, usize) {
        let tx = (world_x / self.tile_size) as usize;
        let ty = (world_y / self.tile_size) as usize;
        (tx.min(self.width - 1), ty.min(self.height - 1))
    }

    pub fn tile_to_world(&self, tx: usize, ty: usize) -> (f32, f32) {
        (tx as f32 * self.tile_size, ty as f32 * self.tile_size)
    }

    pub fn visible_tiles(
        &self,
        camera_x: f32,
        camera_y: f32,
        screen_width: f32,
        screen_height: f32,
    ) -> Vec<(usize, usize, &TileInfo)> {
        let start_tx = (camera_x / self.tile_size).max(0.0) as usize;
        let start_ty = (camera_y / self.tile_size).max(0.0) as usize;
        let end_tx = ((camera_x + screen_width) / self.tile_size).ceil() as usize;
        let end_ty = ((camera_y + screen_height) / self.tile_size).ceil() as usize;

        let mut visible = Vec::new();
        for ty in start_ty..=end_ty.min(self.height - 1) {
            for tx in start_tx..=end_tx.min(self.width - 1) {
                if let Some(tile) = self.get_tile(tx, ty) {
                    visible.push((tx, ty, tile));
                }
            }
        }
        visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::area_gen::AreaGenerationSystem;

    #[test]
    fn test_tilemap_from_area() {
        let area = AreaGenerationSystem::generate_area(10, 10, 42);
        let tilemap = Tilemap::from_generated_area(&area, 32.0);
        assert_eq!(tilemap.width, 10);
        assert_eq!(tilemap.height, 10);
        assert_eq!(tilemap.tile_size, 32.0);
    }

    #[test]
    fn test_world_to_tile() {
        let area = AreaGenerationSystem::generate_area(10, 10, 42);
        let tilemap = Tilemap::from_generated_area(&area, 32.0);
        let (tx, ty) = tilemap.world_to_tile(64.0, 64.0);
        assert_eq!(tx, 2);
        assert_eq!(ty, 2);
    }

    #[test]
    fn test_tile_to_world() {
        let area = AreaGenerationSystem::generate_area(10, 10, 42);
        let tilemap = Tilemap::from_generated_area(&area, 32.0);
        let (wx, wy) = tilemap.tile_to_world(2, 3);
        assert_eq!(wx, 64.0);
        assert_eq!(wy, 96.0);
    }

    #[test]
    fn test_visible_tiles() {
        let area = AreaGenerationSystem::generate_area(20, 20, 42);
        let tilemap = Tilemap::from_generated_area(&area, 32.0);
        let visible = tilemap.visible_tiles(0.0, 0.0, 640.0, 480.0);
        assert!(!visible.is_empty());
        // Should show about 20x15 tiles (inclusive range may add one row)
        assert!(visible.len() <= 20 * 16);
    }

    #[test]
    fn test_get_tile() {
        let area = AreaGenerationSystem::generate_area(10, 10, 42);
        let tilemap = Tilemap::from_generated_area(&area, 32.0);
        assert!(tilemap.get_tile(0, 0).is_some());
        assert!(tilemap.get_tile(99, 99).is_none());
    }
}
