use crate::components::position::Position;
use crate::components::render::RenderLayer;
use crate::resources::camera::Camera;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl SpriteVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpriteInstance {
    pub texture_id: String,
    pub position: Position,
    pub size: (f32, f32),
    pub layer: RenderLayer,
    pub color: [f32; 4],
    pub visible: bool,
}

pub struct SpriteBatch {
    pub sprites: Vec<SpriteInstance>,
}

impl SpriteBatch {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, sprite: SpriteInstance) {
        self.sprites.push(sprite);
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
    }

    pub fn collect_visible(&self, camera: &Camera) -> Vec<&SpriteInstance> {
        self.sprites.iter()
            .filter(|s| s.visible)
            .filter(|s| {
                let screen_x = s.position.x - camera.x;
                let screen_y = s.position.y - camera.y;
                let margin = 64.0;
                screen_x >= -margin 
                    && screen_x <= camera.width + margin
                    && screen_y >= -margin 
                    && screen_y <= camera.height + margin
            })
            .collect()
    }

    pub fn sort_by_layer(sprites: &mut Vec<&SpriteInstance>) {
        sprites.sort_by(|a, b| a.layer.cmp(&b.layer));
    }

    pub fn group_by_texture<'a>(sprites: &'a [&'a SpriteInstance]) -> Vec<(&'a str, Vec<&'a SpriteInstance>)> {
        let mut groups: Vec<(&str, Vec<&SpriteInstance>)> = Vec::new();
        for sprite in sprites {
            let texture_id = sprite.texture_id.as_str();
            if let Some((_, group)) = groups.iter_mut().find(|(id, _)| *id == texture_id) {
                group.push(sprite);
            } else {
                groups.push((texture_id, vec![sprite]));
            }
        }
        groups
    }

    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }

    pub fn visible_count(&self, camera: &Camera) -> usize {
        self.collect_visible(camera).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_batch_add() {
        let mut batch = SpriteBatch::new();
        batch.add_sprite(SpriteInstance {
            texture_id: "player".to_string(),
            position: Position::new(100.0, 100.0),
            size: (32.0, 32.0),
            layer: RenderLayer::Characters,
            color: [1.0, 1.0, 1.0, 1.0],
            visible: true,
        });
        assert_eq!(batch.sprite_count(), 1);
    }

    #[test]
    fn test_visible_culling() {
        let mut batch = SpriteBatch::new();
        batch.add_sprite(SpriteInstance {
            texture_id: "visible".to_string(),
            position: Position::new(640.0, 360.0),
            size: (32.0, 32.0),
            layer: RenderLayer::Characters,
            color: [1.0, 1.0, 1.0, 1.0],
            visible: true,
        });
        batch.add_sprite(SpriteInstance {
            texture_id: "hidden".to_string(),
            position: Position::new(-9999.0, -9999.0),
            size: (32.0, 32.0),
            layer: RenderLayer::Characters,
            color: [1.0, 1.0, 1.0, 1.0],
            visible: true,
        });

        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        assert_eq!(batch.visible_count(&camera), 1);
    }

    #[test]
    fn test_invisible_sprite() {
        let mut batch = SpriteBatch::new();
        batch.add_sprite(SpriteInstance {
            texture_id: "invisible".to_string(),
            position: Position::new(640.0, 360.0),
            size: (32.0, 32.0),
            layer: RenderLayer::Characters,
            color: [1.0, 1.0, 1.0, 1.0],
            visible: false,
        });

        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        assert_eq!(batch.visible_count(&camera), 0);
    }

    #[test]
    fn test_layer_sorting() {
        let mut batch = SpriteBatch::new();
        batch.add_sprite(SpriteInstance {
            texture_id: "ui".to_string(), position: Position::new(0.0, 0.0),
            size: (32.0, 32.0), layer: RenderLayer::UI,
            color: [1.0, 1.0, 1.0, 1.0], visible: true,
        });
        batch.add_sprite(SpriteInstance {
            texture_id: "bg".to_string(), position: Position::new(0.0, 0.0),
            size: (32.0, 32.0), layer: RenderLayer::Background,
            color: [1.0, 1.0, 1.0, 1.0], visible: true,
        });

        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        let mut visible = batch.collect_visible(&camera);
        SpriteBatch::sort_by_layer(&mut visible);
        assert_eq!(visible[0].layer, RenderLayer::Background);
        assert_eq!(visible[1].layer, RenderLayer::UI);
    }

    #[test]
    fn test_texture_grouping() {
        let mut batch = SpriteBatch::new();
        batch.add_sprite(SpriteInstance {
            texture_id: "a".to_string(), position: Position::new(0.0, 0.0),
            size: (32.0, 32.0), layer: RenderLayer::Characters,
            color: [1.0, 1.0, 1.0, 1.0], visible: true,
        });
        batch.add_sprite(SpriteInstance {
            texture_id: "b".to_string(), position: Position::new(0.0, 0.0),
            size: (32.0, 32.0), layer: RenderLayer::Characters,
            color: [1.0, 1.0, 1.0, 1.0], visible: true,
        });
        batch.add_sprite(SpriteInstance {
            texture_id: "a".to_string(), position: Position::new(0.0, 0.0),
            size: (32.0, 32.0), layer: RenderLayer::Characters,
            color: [1.0, 1.0, 1.0, 1.0], visible: true,
        });

        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        let visible = batch.collect_visible(&camera);
        let groups = SpriteBatch::group_by_texture(&visible);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.iter().find(|(id, _)| *id == "a").unwrap().1.len(), 2);
        assert_eq!(groups.iter().find(|(id, _)| *id == "b").unwrap().1.len(), 1);
    }
}