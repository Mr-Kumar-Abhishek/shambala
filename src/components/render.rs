#[derive(Debug, Clone)]
pub struct Renderable {
    pub sprite_id: String,
    pub layer: RenderLayer,
    pub visible: bool,
    pub opacity: f32,
    pub animation_state: String,
    pub frame_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderLayer {
    Background,
    Floor,
    Items,
    Characters,
    Effects,
    UI,
    Overlay,
}

impl Renderable {
    pub fn new(sprite_id: &str, layer: RenderLayer) -> Self {
        Self {
            sprite_id: sprite_id.to_string(),
            layer,
            visible: true,
            opacity: 1.0,
            animation_state: "idle".to_string(),
            frame_index: 0,
        }
    }
}
