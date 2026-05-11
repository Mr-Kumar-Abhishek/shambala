# Skill: Rendering Pipeline

## Description
Covers the wgpu-based rendering system in Shambala. This skill explains how to set up wgpu, implement sprite batching, render tilemaps, build a UI overlay layer, configure the camera system, and manage sprite animations.

## Prerequisites
- Familiarity with Rust and basic GPU concepts (vertex buffers, shaders, swap chains)
- Understanding of the [`Renderable`](../src/components/render.rs) and [`Animation`](../src/components/render.rs) components
- Knowledge of [`DepthLayer`](../src/components/position.rs) ordering for Z-sorting
- See [`ecs-patterns.md`](ecs-patterns.md) for ECS fundamentals
- Reference [`../docs/TECHNICAL_DESIGN.md`](../docs/TECHNICAL_DESIGN.md) Section 3 for architecture details

## Steps

### 1. wgpu Setup Pattern

Initialise wgpu instance, adapter, device, queue, and swap chain in the correct order.

```rust
// src/renderer/mod.rs
use wgpu;

pub struct RenderContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain: wgpu::SwapChain,
    pub surface: wgpu::Surface,
    pub config: wgpu::SwapChainDescriptor,
}

impl RenderContext {
    pub async fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find a suitable GPU adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: Some("RenderDevice"),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let size = window.inner_size();
        let config = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &config);

        Self { device, queue, swap_chain, surface, config }
    }

    pub fn resize(&mut self, new_size: (u32, u32), window: &winit::window::Window) {
        self.config.width = new_size.0;
        self.config.height = new_size.1;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.config);
    }
}
```

#### Shader Module Loading Pattern

```rust
pub fn load_shader(device: &wgpu::Device, name: &str, source: &str) -> wgpu::ShaderModule {
    device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some(name),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}
```

#### Bind Group Layout Convention

```rust
// Shared bind group layout for per-frame data (camera uniform)
pub fn create_global_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("GlobalBindGroupLayout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}
```

### 2. Sprite Batching

Collect all visible sprites each frame, group by texture, and issue instanced draw calls per batch.

#### ECS Query for Sprite Collection

```rust
// src/systems/render.rs
use bevy_ecs::system::{Query, Res};
use bevy_ecs::world::World;

pub fn collect_sprites_system(
    world: &World,
) -> Vec<SpriteBatch> {
    let mut batches: Vec<SpriteBatch> = Vec::new();

    // Query all entities with Position + Renderable + DepthLayer
    for (position, renderable, depth, anim_opt) in
        world.query::<(&Position, &Renderable, &DepthLayer, Option<&Animation>)>().iter(world)
    {
        if !renderable.visible {
            continue;
        }

        let sprite_index = match anim_opt {
            Some(anim) if anim.playing => anim.frames[anim.current_frame],
            _ => renderable.sprite_index,
        };

        let instance = SpriteInstance {
            position: [position.x, position.y],
            size: renderable.size,
            source_rect: calculate_uv(
                sprite_index,
                &renderable.texture_id,
            ),
            color: renderable.color,
            flip: [
                if renderable.flip_x { -1.0 } else { 1.0 },
                if renderable.flip_y { -1.0 } else { 1.0 },
            ],
            depth: depth.0 as f32,
        };

        // Find or create batch for this texture
        let batch = batches
            .iter_mut()
            .find(|b: &&mut SpriteBatch| b.texture_id == renderable.texture_id);

        match batch {
            Some(b) => b.instances.push(instance),
            None => batches.push(SpriteBatch {
                texture_id: renderable.texture_id.clone(),
                instances: vec![instance],
            }),
        }
    }

    // Sort batches by depth layer
    batches.sort_by_key(|b| b.instances.first().map(|i| i.depth as i32).unwrap_or(0));
    batches
}
```

#### Instance Buffer Update and Draw

```rust
// Inside the render pass
pub fn render_sprite_batch(
    render_ctx: &RenderContext,
    batch: &SpriteBatch,
    texture_manager: &TextureManager,
    camera: &Camera,
    encoder: &mut wgpu::CommandEncoder,
    global_bind_group: &wgpu::BindGroup,
) {
    let texture = texture_manager.get(&batch.texture_id);
    let instance_data: Vec<[f32; 12]> = batch.instances.iter().map(|i| i.to_shader_data()).collect();

    // Create or update instance buffer
    let instance_buffer = render_ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("SpriteInstanceBuffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsage::VERTEX,
        }
    );

    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(&format!("SpriteBatch: {}", batch.texture_id)),
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&sprite_pipeline);
    render_pass.set_bind_group(0, global_bind_group, &[]);
    render_pass.set_bind_group(1, &texture.bind_group, &[]);
    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
    render_pass.draw_indexed(0..6, 0, 0..batch.instances.len() as u32);
}
```

#### Vertex and Instance Layout

```rust
pub fn sprite_vertex_buffer_layout() -> Vec<wgpu::VertexBufferLayout> {
    vec![
        // Per-vertex data (quad)
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![
                0 => Float32x3,  // position (clip space)
                1 => Float32x2,  // uv
            ],
        },
        // Per-instance data
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![
                2 => Float32x2,  // world position
                3 => Float32x2,  // size
                4 => Float32x4,  // source_rect (uv)
                5 => Float32x4,  // color (tint)
                6 => Float32x2,  // flip
                7 => Float32x1,  // depth
            ],
        },
    ]
}
```

### 3. Tilemap Rendering

Tilemaps use a single static mesh rebuilt only when the area changes.

#### Tilemap Mesh Construction

```rust
// src/renderer/tilemap.rs
pub struct TilemapRenderer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    texture_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl TilemapRenderer {
    pub fn new(
        device: &wgpu::Device,
        tilemap: &TilemapData,
        tileset: &Texture,
        layout: &wgpu::PipelineLayout,
    ) -> Self {
        let (vertices, indices) = Self::build_mesh(tilemap);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TilemapVertexBuffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TilemapIndexBuffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsage::INDEX,
        });

        // ... pipeline creation, bind group setup

        Self { vertex_buffer, index_buffer, index_count: indices.len() as u32, texture_bind_group, pipeline }
    }

    fn build_mesh(tilemap: &TilemapData) -> (Vec<TileVertex>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for layer_idx in 0..tilemap.layer_count() {
            for y in 0..tilemap.height {
                for x in 0..tilemap.width {
                    let tile_id = tilemap.tile_at(layer_idx, x, y);
                    if tile_id == 0 { continue; } // Skip empty tiles

                    let base_idx = vertices.len() as u16;
                    let tile_size = tilemap.tile_size;
                    let px = x as f32 * tile_size.0;
                    let py = y as f32 * tile_size.1;

                    // Four corners of the tile quad
                    vertices.push(TileVertex { position: [px, py, layer_idx as f32], uv: [0.0, 0.0] });
                    vertices.push(TileVertex { position: [px + tile_size.0, py, layer_idx as f32], uv: [1.0, 0.0] });
                    vertices.push(TileVertex { position: [px + tile_size.0, py + tile_size.1, layer_idx as f32], uv: [1.0, 1.0] });
                    vertices.push(TileVertex { position: [px, py + tile_size.1, layer_idx as f32], uv: [0.0, 1.0] });

                    indices.extend_from_slice(&[
                        base_idx, base_idx + 1, base_idx + 2,
                        base_idx, base_idx + 2, base_idx + 3,
                    ]);
                }
            }
        }

        (vertices, indices)
    }
}
```

#### Tilemap Render Pass

```rust
pub fn render_tilemap(
    tilemap: &TilemapRenderer,
    render_pass: &mut wgpu::RenderPass,
) {
    render_pass.set_pipeline(&tilemap.pipeline);
    render_pass.set_bind_group(0, &tilemap.texture_bind_group, &[]);
    render_pass.set_vertex_buffer(0, tilemap.vertex_buffer.slice(..));
    render_pass.set_index_buffer(tilemap.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..tilemap.index_count, 0, 0..1);
}
```

### 4. UI Rendering Layer

UI is rendered as a separate orthographic pass on top of the world, using screen-space coordinates.

#### UI Element System

```rust
// src/renderer/ui.rs
pub struct UIRenderer {
    sprite_pipeline: wgpu::RenderPipeline,
    text_pipeline: wgpu::RenderPipeline,
    atlas: TextureAtlas, // Shared glyph + icon atlas
}

#[derive(Clone)]
pub struct UIElement {
    pub element_type: UIType,
    pub position: (f32, f32),        // Screen-space
    pub size: (f32, f32),
    pub texture_id: Option<String>,
    pub color: [f32; 4],
    pub text: Option<String>,
    pub font_size: f32,
    pub interactable: bool,
    pub clip_rect: Option<[f32; 4]>, // Scissor rect
}

pub enum UIType {
    Panel,
    Button,
    Label,
    ProgressBar,
    Icon,
    TextInput,
    ScrollContainer,
}
```

#### UI Render Pass

```rust
pub fn render_ui(
    renderer: &UIRenderer,
    ui_elements: &[UIElement],
    encoder: &mut wgpu::CommandEncoder,
    frame_view: &wgpu::TextureView,
) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("UIOverlay"),
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: frame_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load, // Blend on top of game world
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });

    // Sort by Z-order
    let mut sorted = ui_elements.to_vec();
    sorted.sort_by_key(|e| e.element_type.z_order());

    for element in sorted {
        match element.element_type {
            UIType::Label => render_text(&mut render_pass, &renderer.text_pipeline, &element),
            UIType::ProgressBar => render_progress_bar(&mut render_pass, &renderer.sprite_pipeline, &element),
            _ => render_sprite(&mut render_pass, &renderer.sprite_pipeline, &element),
        }
    }
}

impl UIType {
    fn z_order(&self) -> i32 {
        match self {
            UIType::Panel => 0,
            UIType::ProgressBar => 1,
            UIType::Icon => 2,
            UIType::Label => 3,
            UIType::Button => 4,
            UIType::ScrollContainer => 5,
            UIType::TextInput => 6,
        }
    }
}
```

### 5. Camera System

Manages world-to-screen transformation, smooth follow, and screen shake.

```rust
// src/resources/camera.rs
use bevy_ecs::system::Resource;

#[derive(Resource, Clone, Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub target: Option<(f32, f32)>,
    pub lerp_speed: f32,
    pub shake_intensity: f32,
    pub shake_decay: f32,
    pub shake_offset: (f32, f32),
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            target: None,
            lerp_speed: 8.0,
            shake_intensity: 0.0,
            shake_decay: 4.0,
            shake_offset: (0.0, 0.0),
        }
    }

    /// Transforms world-space coordinates to screen-space.
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_w: f32, screen_h: f32) -> (f32, f32) {
        let sx = (world_x - self.x) * self.zoom + screen_w / 2.0 + self.shake_offset.0;
        let sy = (world_y - self.y) * self.zoom + screen_h / 2.0 + self.shake_offset.1;
        (sx, sy)
    }
}
```

#### Camera Follow System

```rust
// src/systems/camera.rs
pub fn camera_follow_system(
    mut camera: ResMut<Camera>,
    time: Res<Time>,
) {
    // Smooth interpolation to target
    if let Some((tx, ty)) = camera.target {
        camera.x += (tx - camera.x) * camera.lerp_speed * time.delta;
        camera.y += (ty - camera.y) * camera.lerp_speed * time.delta;
    }

    // Camera shake decay
    if camera.shake_intensity > 0.0 {
        let offset_x = (rand::random::<f32>() - 0.5) * 2.0 * camera.shake_intensity;
        let offset_y = (rand::random::<f32>() - 0.5) * 2.0 * camera.shake_intensity;
        camera.shake_offset = (offset_x, offset_y);
        camera.shake_intensity -= camera.shake_decay * time.delta;
        if camera.shake_intensity < 0.0 {
            camera.shake_intensity = 0.0;
            camera.shake_offset = (0.0, 0.0);
        }
    }
}
```

#### Camera Uniform for Shaders

```rust
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4], // Orthographic projection * view matrix
}

impl Camera {
    pub fn build_view_proj(&self, screen_w: f32, screen_h: f32) -> CameraUniform {
        // Orthographic projection centred on camera position
        let left = self.x - screen_w / (2.0 * self.zoom);
        let right = self.x + screen_w / (2.0 * self.zoom);
        let bottom = self.y - screen_h / (2.0 * self.zoom);
        let top = self.y + screen_h / (2.0 * self.zoom);

        let proj = cgmath::ortho(left, right, bottom, top, -1000.0, 1000.0);
        let view = cgmath::Matrix4::identity(); // Camera is the origin in view space

        CameraUniform {
            view_proj: (proj * view).into(),
        }
    }
}
```

### 6. Animation System

Spritesheet animation driven by the [`Animation`](../src/components/render.rs) component.

#### Animation Component

```rust
#[derive(Component, Clone, Debug)]
pub struct Animation {
    pub frames: Vec<usize>,       // Frame indices into spritesheet
    pub frame_duration: f32,      // Seconds per frame
    pub current_frame: usize,
    pub timer: f32,
    pub looping: bool,
    pub playing: bool,
}
```

#### Animation Update System

```rust
// src/systems/animation.rs
pub fn animation_system(
    time: Res<Time>,
    mut query: Query<&mut Animation>,
) {
    for mut anim in query.iter_mut() {
        if !anim.playing { continue; }

        anim.timer += time.delta;
        if anim.timer >= anim.frame_duration {
            anim.timer -= anim.frame_duration;
            anim.current_frame += 1;

            if anim.current_frame >= anim.frames.len() {
                if anim.looping {
                    anim.current_frame = 0;
                } else {
                    anim.current_frame = anim.frames.len() - 1;
                    anim.playing = false;
                }
            }
        }
    }
}
```

#### Animation Factory Methods

```rust
impl Animation {
    pub fn idle() -> Self {
        Self {
            frames: vec![0],
            frame_duration: 1.0,
            current_frame: 0,
            timer: 0.0,
            looping: true,
            playing: true,
        }
    }

    pub fn walk() -> Self {
        Self {
            frames: vec![0, 1, 2, 1],
            frame_duration: 0.15,
            current_frame: 0,
            timer: 0.0,
            looping: true,
            playing: true,
        }
    }

    pub fn attack() -> Self {
        Self {
            frames: vec![3, 4, 5],
            frame_duration: 0.08,
            current_frame: 0,
            timer: 0.0,
            looping: false,
            playing: true,
        }
    }

    pub fn single(frame: usize) -> Self {
        Self {
            frames: vec![frame],
            frame_duration: 1.0,
            current_frame: 0,
            timer: 0.0,
            looping: false,
            playing: true,
        }
    }
}
```

## Examples

### Complete Frame Render Loop

```rust
// Inside the main game loop
pub fn render_frame(
    render_ctx: &mut RenderContext,
    world: &World,
    texture_manager: &TextureManager,
    camera: &Camera,
) {
    let frame = render_ctx.swap_chain
        .get_current_frame()
        .expect("Failed to acquire swap chain frame")
        .output;
    let frame_view = &frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = render_ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("FrameEncoder"),
    });

    // 1. Clear the screen (or load from previous frame)
    // 2. Render tilemap (opaque pass)
    // 3. Collect and render sprite batches (opaque, then transparent)
    // 4. Post-processing pass (tint, weather, damage flash)
    // 5. Render UI overlay pass
    // 6. Submit to queue

    render_ctx.queue.submit(Some(encoder.finish()));
}
```

### Adding a New Animation

```rust
// In entity factory or AI system:
commands.entity(enemy).insert(
    Animation {
        frames: vec![6, 7, 8, 9],
        frame_duration: 0.1,
        current_frame: 0,
        timer: 0.0,
        looping: true,
        playing: true,
    }
);
```

### Triggering Camera Shake

```rust
// When damage is taken or explosion occurs:
fn trigger_shake(mut camera: ResMut<Camera>, intensity: f32) {
    camera.shake_intensity = intensity;
}
```

## Related Skills
- [`ecs-patterns.md`](ecs-patterns.md) — ECS fundamentals, component/system registration
- [`testing-patterns.md`](testing-patterns.md) — How to test render systems with headless CI
- [`../docs/TECHNICAL_DESIGN.md`](../docs/TECHNICAL_DESIGN.md) Section 3 — Full rendering pipeline architecture
- [`../docs/GDD.md`](../docs/GDD.md) — Visual style and UI/UX design references
