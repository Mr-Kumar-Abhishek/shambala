#[derive(Debug, Clone)]
pub struct GameTime {
    pub delta_time: f32,
    pub total_time: f32,
    pub fps: f64,
    pub frame_count: u64,
    pub time_scale: f32,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            total_time: 0.0,
            fps: 0.0,
            frame_count: 0,
            time_scale: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.delta_time = dt * self.time_scale;
        self.total_time += self.delta_time;
        self.frame_count += 1;
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta_time
    }

    pub fn total_seconds(&self) -> f32 {
        self.total_time
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0).min(10.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_time() {
        let time = GameTime::new();
        assert_eq!(time.delta_time, 0.0);
        assert_eq!(time.frame_count, 0);
    }

    #[test]
    fn test_update() {
        let mut time = GameTime::new();
        time.update(0.016);
        assert!((time.delta_seconds() - 0.016).abs() < f32::EPSILON);
        assert_eq!(time.frame_count, 1);
    }

    #[test]
    fn test_time_scale() {
        let mut time = GameTime::new();
        time.set_time_scale(2.0);
        time.update(0.016);
        assert!((time.delta_seconds() - 0.032).abs() < f32::EPSILON);
    }

    #[test]
    fn test_time_scale_clamping() {
        let mut time = GameTime::new();
        time.set_time_scale(-1.0);
        assert_eq!(time.time_scale, 0.0);
        time.set_time_scale(20.0);
        assert_eq!(time.time_scale, 10.0);
    }
}
