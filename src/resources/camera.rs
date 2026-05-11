#[derive(Debug, Clone)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x, y, width, height, zoom: 1.0,
        }
    }

    pub fn follow(&mut self, target_x: f32, target_y: f32, lerp_speed: f32) {
        let target_center_x = target_x - self.width / 2.0;
        let target_center_y = target_y - self.height / 2.0;
        self.x += (target_center_x - self.x) * lerp_speed;
        self.y += (target_center_y - self.y) * lerp_speed;
    }

    pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> (f32, f32) {
        (
            (world_x - self.x) * self.zoom,
            (world_y - self.y) * self.zoom,
        )
    }

    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        (
            screen_x / self.zoom + self.x,
            screen_y / self.zoom + self.y,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_new() {
        let cam = Camera::new(0.0, 0.0, 1280.0, 720.0);
        assert_eq!(cam.width, 1280.0);
        assert_eq!(cam.zoom, 1.0);
    }

    #[test]
    fn test_world_to_screen() {
        let cam = Camera::new(100.0, 100.0, 1280.0, 720.0);
        let (sx, sy) = cam.world_to_screen(200.0, 200.0);
        assert_eq!(sx, 100.0);
        assert_eq!(sy, 100.0);
    }

    #[test]
    fn test_screen_to_world() {
        let cam = Camera::new(100.0, 100.0, 1280.0, 720.0);
        let (wx, wy) = cam.screen_to_world(100.0, 100.0);
        assert_eq!(wx, 200.0);
        assert_eq!(wy, 200.0);
    }

    #[test]
    fn test_follow() {
        let mut cam = Camera::new(0.0, 0.0, 100.0, 100.0);
        cam.follow(200.0, 200.0, 1.0);
        assert_eq!(cam.x, 150.0);
        assert_eq!(cam.y, 150.0);
    }
}
