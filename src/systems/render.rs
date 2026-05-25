use crate::components::position::Position;
use crate::components::render::Renderable;
use crate::resources::camera::Camera;

pub struct RenderSystem;

impl RenderSystem {
    pub fn is_visible(renderable: &Renderable, position: &Position, camera: &Camera) -> bool {
        if !renderable.visible {
            return false;
        }

        let screen_x = position.x - camera.x;
        let screen_y = position.y - camera.y;

        // Check if within camera view (with margin)
        let margin = 64.0;
        screen_x >= -margin
            && screen_x <= camera.width + margin
            && screen_y >= -margin
            && screen_y <= camera.height + margin
    }

    pub fn sort_by_layer(renderables: &[(Position, Renderable)]) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..renderables.len()).collect();
        indices.sort_by(|&a, &b| renderables[a].1.layer.cmp(&renderables[b].1.layer));
        indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::render::RenderLayer;

    #[test]
    fn test_invisible_not_rendered() {
        let renderable = Renderable {
            visible: false,
            ..Renderable::new("test", RenderLayer::Characters)
        };
        let pos = Position::new(100.0, 100.0);
        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        assert!(!RenderSystem::is_visible(&renderable, &pos, &camera));
    }

    #[test]
    fn test_visible_in_view() {
        let renderable = Renderable::new("test", RenderLayer::Characters);
        let pos = Position::new(640.0, 360.0);
        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        assert!(RenderSystem::is_visible(&renderable, &pos, &camera));
    }

    #[test]
    fn test_not_visible_outside_view() {
        let renderable = Renderable::new("test", RenderLayer::Characters);
        let pos = Position::new(-9999.0, -9999.0);
        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        assert!(!RenderSystem::is_visible(&renderable, &pos, &camera));
    }

    #[test]
    fn test_sort_by_layer() {
        let items = vec![
            (
                Position::new(0.0, 0.0),
                Renderable::new("bg", RenderLayer::Background),
            ),
            (
                Position::new(0.0, 0.0),
                Renderable::new("char", RenderLayer::Characters),
            ),
            (
                Position::new(0.0, 0.0),
                Renderable::new("ui", RenderLayer::UI),
            ),
        ];
        let sorted = RenderSystem::sort_by_layer(&items);
        assert_eq!(sorted, vec![0, 1, 2]);
    }
}
