# Skill: Asset Pipeline

## Description
How to implement texture loading and asset management using the `image` crate and wgpu for the Shambala game.

## Prerequisites
- Understanding of wgpu concepts (Device, Queue, Texture, Sampler)
- Familiarity with the `image` crate for PNG loading
- Understanding of texture atlases for sprite batching

## Steps

### 1. Load PNG with `image`
```rust
use image::GenericImageView;

pub fn load_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
) -> (wgpu::Texture, wgpu::TextureView, u32, u32) {
    let img = image::load_from_memory(bytes).expect("Failed to load image from memory");
    let dimensions = img.dimensions();
    let rgba = img.to_rgba8();

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Loaded Texture"),
        size: wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        },
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view, dimensions.0, dimensions.1)
}
```

### 2. Create a Sampler
```rust
let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    label: Some("Texture Sampler"),
    address_mode_u: wgpu::AddressMode::ClampToEdge,
    address_mode_v: wgpu::AddressMode::ClampToEdge,
    address_mode_w: wgpu::AddressMode::ClampToEdge,
    mag_filter: wgpu::FilterMode::Nearest,
    min_filter: wgpu::FilterMode::Nearest,
    mipmap_filter: wgpu::FilterMode::Nearest,
    ..Default::default()
});
```

### 3. Pack Sprites into Texture Atlas
```rust
pub struct TextureAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
}

impl TextureAtlas {
    pub fn from_grid(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sprite_sheet_bytes: &[u8],
        tile_width: u32,
        tile_height: u32,
    ) -> Self {
        let (texture, view, total_width, total_height) =
            load_texture(device, queue, sprite_sheet_bytes);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        Self {
            texture,
            view,
            sampler,
            tile_width,
            tile_height,
            columns: total_width / tile_width,
            rows: total_height / tile_height,
        }
    }

    /// Get UV coordinates for a sprite at grid position (col, row).
    pub fn sprite_uv(&self, col: u32, row: u32) -> [f32; 4] {
        let u = col as f32 / self.columns as f32;
        let v = row as f32 / self.rows as f32;
        let w = self.tile_width as f32 / (self.columns * self.tile_width) as f32;
        let h = self.tile_height as f32 / (self.rows * self.tile_height) as f32;
        [u, v, u + w, v + h]
    }
}
```

### 4. Implement Async Loading with Progress Tracking
```rust
use std::sync::Arc;

pub struct AssetLoadProgress {
    pub total: usize,
    pub loaded: usize,
    pub errors: Vec<String>,
}

pub struct AssetManager {
    pub textures: HashMap<String, (wgpu::Texture, wgpu::TextureView)>,
    pub atlases: HashMap<String, TextureAtlas>,
    pub progress: AssetLoadProgress,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            atlases: HashMap::new(),
            progress: AssetLoadProgress {
                total: 0,
                loaded: 0,
                errors: Vec::new(),
            },
        }
    }

    pub fn load_texture_async(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: &str,
        bytes: &[u8],
    ) {
        self.progress.total += 1;
        match load_texture(device, queue, bytes) {
            (texture, view, _w, _h) => {
                self.textures.insert(name.to_string(), (texture, view));
                self.progress.loaded += 1;
            }
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.progress.total > 0 && self.progress.loaded == self.progress.total
    }

    pub fn progress_percent(&self) -> f32 {
        if self.progress.total == 0 {
            return 1.0;
        }
        self.progress.loaded as f32 / self.progress.total as f32
    }
}
```

## Testing
- **Texture loading:** Call `load_texture()` with known PNG bytes, verify texture dimensions
- **Atlas packing:** Create atlas from grid, verify `sprite_uv()` returns correct UV rects
- **Cache hit/miss:** Load same texture twice, verify cached vs fresh paths
- **Progress tracking:** Load multiple textures, verify `progress_percent()` advances correctly
