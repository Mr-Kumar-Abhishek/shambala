use rand::Rng;
use rand::SeedableRng;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AreaTile {
    pub tile_type: TileType,
    pub walkable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
    Water,
    Grass,
    Path,
    Entrance,
    Exit,
    Treasure,
}

#[derive(Debug, Clone)]
pub struct GeneratedArea {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<AreaTile>>,
    pub entrance: (usize, usize),
    pub exit: (usize, usize),
}

pub struct AreaGenerationSystem;

impl AreaGenerationSystem {
    pub fn generate_area(width: usize, height: usize, seed: u64) -> GeneratedArea {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut tiles = Vec::with_capacity(height);

        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                // Border walls
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    row.push(AreaTile {
                        tile_type: TileType::Wall,
                        walkable: false,
                    });
                } else {
                    // Random floor/wall with more floor
                    let tile_type = if rng.gen_range(0..100) < 80 {
                        TileType::Floor
                    } else {
                        TileType::Wall
                    };
                    row.push(AreaTile {
                        tile_type,
                        walkable: tile_type == TileType::Floor,
                    });
                }
            }
            tiles.push(row);
        }

        // Place entrance and exit
        let entrance = (1, 1);
        let exit = (width - 2, height - 2);
        tiles[entrance.1][entrance.0] = AreaTile {
            tile_type: TileType::Entrance,
            walkable: true,
        };
        tiles[exit.1][exit.0] = AreaTile {
            tile_type: TileType::Exit,
            walkable: true,
        };

        GeneratedArea {
            width,
            height,
            tiles,
            entrance,
            exit,
        }
    }

    pub fn is_walkable(area: &GeneratedArea, x: usize, y: usize) -> bool {
        if x >= area.width || y >= area.height {
            return false;
        }
        area.tiles[y][x].walkable
    }

    pub fn find_path(
        area: &GeneratedArea,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        // Simple BFS pathfinding
        let mut visited = vec![vec![false; area.width]; area.height];
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited[start.1][start.0] = true;

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = Vec::new();
                let mut pos = current;
                while pos != start {
                    path.push(pos);
                    pos = *parent.get(&pos).unwrap_or(&start);
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            let neighbors = [
                (current.0.wrapping_sub(1), current.1),
                (current.0 + 1, current.1),
                (current.0, current.1.wrapping_sub(1)),
                (current.0, current.1 + 1),
            ];

            for (nx, ny) in neighbors {
                if nx < area.width
                    && ny < area.height
                    && !visited[ny][nx]
                    && area.tiles[ny][nx].walkable
                {
                    visited[ny][nx] = true;
                    parent.insert((nx, ny), current);
                    queue.push_back((nx, ny));
                }
            }
        }

        None // No path found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_area() {
        let area = AreaGenerationSystem::generate_area(20, 20, 42);
        assert_eq!(area.width, 20);
        assert_eq!(area.height, 20);
        assert_eq!(
            area.tiles[area.entrance.1][area.entrance.0].tile_type,
            TileType::Entrance
        );
        assert_eq!(
            area.tiles[area.exit.1][area.exit.0].tile_type,
            TileType::Exit
        );
    }

    #[test]
    fn test_is_walkable() {
        let area = AreaGenerationSystem::generate_area(10, 10, 42);
        assert!(AreaGenerationSystem::is_walkable(&area, 1, 1)); // Entrance
        assert!(!AreaGenerationSystem::is_walkable(&area, 0, 0)); // Wall
    }

    #[test]
    fn test_out_of_bounds() {
        let area = AreaGenerationSystem::generate_area(10, 10, 42);
        assert!(!AreaGenerationSystem::is_walkable(&area, 99, 99));
    }

    #[test]
    fn test_pathfinding() {
        let area = AreaGenerationSystem::generate_area(10, 10, 42);
        let path = AreaGenerationSystem::find_path(&area, area.entrance, area.exit);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path[0], area.entrance);
        assert_eq!(path[path.len() - 1], area.exit);
    }

    #[test]
    fn test_deterministic_generation() {
        let area1 = AreaGenerationSystem::generate_area(15, 15, 123);
        let area2 = AreaGenerationSystem::generate_area(15, 15, 123);
        for y in 0..15 {
            for x in 0..15 {
                assert_eq!(area1.tiles[y][x].tile_type, area2.tiles[y][x].tile_type);
            }
        }
    }
}
