# Skill: Rendering Pipeline (wgpu + winit)

## Description
How to implement the 2D rendering pipeline using wgpu and winit for the Shambala game.

## Prerequisites
- Understanding of wgpu concepts (Instance, Surface, Device, Queue, Pipeline)
- Understanding of winit (EventLoop, Window, Events)
- Rust async/await basics

## Steps

### 1. Window Creation with winit
```rust
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

let event_loop = EventLoop::new()?;
let window = WindowBuilder::new()
    .with_title("Shambala")
    .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
    .build(&event_loop)?;
```

### 2. wgpu Instance & Surface
```rust
let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
    backends: wgpu::Backends::PRIMARY,
    ..Default::default()
});
let surface = unsafe { instance.create_surface(&window) }?;
let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
    power_preference: wgpu::PowerPreference::HighPerformance,
    compatible_surface: Some(&surface),
    ..Default::default()
}).await.unwrap();
```

### 3. Device & Queue
```rust
let (device, queue) = adapter.request_device(
    &wgpu::DeviceDescriptor {
        label: Some("Render Device"),
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
    },
    None,
).await.unwrap();
```

### 4. Swap Chain & Surface Configuration
```rust
let surface_caps = surface.get_capabilities(&adapter);
let surface_format = surface_caps.formats.iter()
    .find(|f| f.is_srgb())
    .unwrap_or(&surface_caps.formats[0]);
let config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: *surface_format,
    width: 1280,
    height: 720,
    present_mode: wgpu::PresentMode::Fifo,
    alpha_mode: wgpu::CompositeAlphaMode::Auto,
    view_formats: vec![],
};
surface.configure(&device, &config);
```

### 5. Sprite Rendering Pipeline
```rust
// Vertex shader
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Sprite Shader"),
    source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite.wgsl").into()),
});

// Pipeline layout
let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("Sprite Pipeline Layout"),
    bind_group_layouts: &[&texture_bind_group_layout],
    push_constant_ranges: &[],
});

// Render pipeline
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Sprite Pipeline"),
    layout: Some(&pipeline_layout),
    vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[vertex_buffer_layout],
    },
    fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(wgpu::ColorTargetState {
            format: surface_format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })],
    }),
    primitive: wgpu::PrimitiveState::default(),
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
    multiview: None,
});
```

### 6. Render Loop
```rust
event_loop.run(move |event, _, control_flow| {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            winit::event::WindowEvent::Resized(size) => {
                // Reconfigure surface
            }
            _ => {}
        },
        winit::event::Event::RedrawRequested(_) => {
            // Render frame
            let frame = surface.get_current_texture().unwrap();
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            // ... render passes
            queue.submit(std::iter::once(encoder.finish()));
            frame.present();
        }
        _ => {}
    }
});
```

## Examples
- See `src/render/pipeline.rs` for full implementation
- See `src/render/sprite.rs` for sprite batching

## Related Skills
- `ecs-patterns.md` — ECS integration with rendering
- `testing-patterns.md` — Testing rendering code
