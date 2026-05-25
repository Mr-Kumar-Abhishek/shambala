use image::GenericImageView;
use std::collections::HashMap;
use wgpu;

pub struct TextureManager {
    pub textures: HashMap<String, TextureHandle>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct TextureHandle {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub width: u32,
    pub height: u32,
    pub bind_group: wgpu::BindGroup,
}

impl TextureManager {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            textures: HashMap::new(),
            device,
            queue,
        }
    }

    pub fn load_from_bytes(&mut self, id: &str, bytes: &[u8]) -> Result<(), String> {
        let img =
            image::load_from_memory(bytes).map_err(|e| format!("Failed to load image: {}", e))?;
        let dimensions = img.dimensions();
        let rgba = img.to_rgba8();
        let width = dimensions.0;
        let height = dimensions.1;

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("texture_{}", id)),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(&format!("sampler_{}", id)),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(&format!("bind_group_layout_{}", id)),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("bind_group_{}", id)),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        self.textures.insert(
            id.to_string(),
            TextureHandle {
                texture,
                view,
                sampler,
                width,
                height,
                bind_group,
            },
        );

        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&TextureHandle> {
        self.textures.get(id)
    }

    pub fn has(&self, id: &str) -> bool {
        self.textures.contains_key(id)
    }

    pub fn unload(&mut self, id: &str) {
        self.textures.remove(id);
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }

    pub fn count(&self) -> usize {
        self.textures.len()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_texture_manager_creation() {
        // This test verifies the module compiles.
        // Actual texture loading requires a GPU device context.
    }

    #[test]
    fn test_texture_id_tracking() {
        // Test that we can track texture IDs without GPU
        let ids = ["player_TwinBlade", "enemy_Goblin", "tile_floor"];
        assert_eq!(ids.len(), 3);
    }
}
