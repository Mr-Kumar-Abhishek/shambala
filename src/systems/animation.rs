#[derive(Debug, Clone)]
pub struct Animation {
    pub id: String,
    pub frames: Vec<AnimationFrame>,
    pub current_frame: usize,
    pub frame_timer: f32,
    pub frame_duration: f32,
    pub loop_: bool,
    pub playing: bool,
    pub finished: bool,
}

#[derive(Debug, Clone)]
pub struct AnimationFrame {
    pub sprite_id: String,
    pub duration: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Animation {
    pub fn new(id: &str, frame_duration: f32, loop_: bool) -> Self {
        Self {
            id: id.to_string(),
            frames: Vec::new(),
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration,
            loop_,
            playing: true,
            finished: false,
        }
    }

    pub fn add_frame(&mut self, sprite_id: &str) {
        self.frames.push(AnimationFrame {
            sprite_id: sprite_id.to_string(),
            duration: self.frame_duration,
            offset_x: 0.0,
            offset_y: 0.0,
        });
    }

    pub fn add_frame_with_offset(&mut self, sprite_id: &str, offset_x: f32, offset_y: f32) {
        self.frames.push(AnimationFrame {
            sprite_id: sprite_id.to_string(),
            duration: self.frame_duration,
            offset_x,
            offset_y,
        });
    }

    pub fn update(&mut self, dt: f32) {
        if !self.playing || self.finished {
            return;
        }

        self.frame_timer += dt;
        if self.frame_timer >= self.frames[self.current_frame].duration {
            self.frame_timer = 0.0;
            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                if self.loop_ {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                    self.finished = true;
                    self.playing = false;
                }
            }
        }
    }

    pub fn current_sprite(&self) -> Option<&str> {
        if self.frames.is_empty() {
            None
        } else {
            Some(&self.frames[self.current_frame].sprite_id)
        }
    }

    pub fn current_offset(&self) -> (f32, f32) {
        if self.frames.is_empty() {
            (0.0, 0.0)
        } else {
            (self.frames[self.current_frame].offset_x, self.frames[self.current_frame].offset_y)
        }
    }

    pub fn play(&mut self) {
        self.playing = true;
        self.finished = false;
        self.current_frame = 0;
        self.frame_timer = 0.0;
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.current_frame = 0;
        self.frame_timer = 0.0;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn resume(&mut self) {
        self.playing = true;
    }

    pub fn progress(&self) -> f32 {
        if self.frames.is_empty() {
            return 1.0;
        }
        let total_frames = self.frames.len() as f32;
        (self.current_frame as f32 + self.frame_timer / self.frames[self.current_frame].duration) / total_frames
    }
}

pub struct AnimationManager {
    pub animations: Vec<Animation>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
        }
    }

    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    pub fn update(&mut self, dt: f32) {
        for anim in &mut self.animations {
            anim.update(dt);
        }
    }

    pub fn get(&self, id: &str) -> Option<&Animation> {
        self.animations.iter().find(|a| a.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Animation> {
        self.animations.iter_mut().find(|a| a.id == id)
    }

    pub fn remove_finished(&mut self) {
        self.animations.retain(|a| !a.finished || a.loop_);
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }

    pub fn create_attack_animation(class: &str) -> Animation {
        let mut anim = Animation::new(&format!("attack_{}", class), 0.1, false);
        anim.add_frame(&format!("{}_attack_1", class));
        anim.add_frame(&format!("{}_attack_2", class));
        anim.add_frame(&format!("{}_attack_3", class));
        anim.add_frame(&format!("{}_idle", class));
        anim
    }

    pub fn create_hit_animation() -> Animation {
        let mut anim = Animation::new("hit", 0.05, false);
        anim.add_frame_with_offset("hit_spark", 0.0, 0.0);
        anim.add_frame_with_offset("hit_spark", 4.0, -4.0);
        anim.add_frame_with_offset("hit_spark", -4.0, 4.0);
        anim.add_frame_with_offset("hit_spark", 2.0, -2.0);
        anim
    }

    pub fn create_spell_animation(element: &str) -> Animation {
        let mut anim = Animation::new(&format!("spell_{}", element), 0.15, false);
        anim.add_frame(&format!("spell_{}_1", element));
        anim.add_frame(&format!("spell_{}_2", element));
        anim.add_frame(&format!("spell_{}_3", element));
        anim.add_frame(&format!("spell_{}_explosion", element));
        anim
    }

    pub fn create_idle_animation(class: &str) -> Animation {
        let mut anim = Animation::new(&format!("idle_{}", class), 0.5, true);
        anim.add_frame(&format!("{}_idle_1", class));
        anim.add_frame(&format!("{}_idle_2", class));
        anim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_creation() {
        let anim = Animation::new("test", 0.1, false);
        assert_eq!(anim.id, "test");
        assert!(anim.playing);
        assert!(!anim.finished);
    }

    #[test]
    fn test_add_frames() {
        let mut anim = Animation::new("test", 0.1, false);
        anim.add_frame("frame_1");
        anim.add_frame("frame_2");
        assert_eq!(anim.frames.len(), 2);
    }

    #[test]
    fn test_frame_progression() {
        let mut anim = Animation::new("test", 0.1, false);
        anim.add_frame("frame_1");
        anim.add_frame("frame_2");
        anim.update(0.15);
        assert_eq!(anim.current_frame, 1);
    }

    #[test]
    fn test_animation_completion() {
        let mut anim = Animation::new("test", 0.1, false);
        anim.add_frame("frame_1");
        anim.add_frame("frame_2");
        anim.update(0.1);
        anim.update(0.1);
        assert!(anim.finished);
    }

    #[test]
    fn test_looping_animation() {
        let mut anim = Animation::new("test", 0.1, true);
        anim.add_frame("frame_1");
        anim.add_frame("frame_2");
        anim.update(0.1);
        anim.update(0.1);
        assert_eq!(anim.current_frame, 0); // Should loop back
        assert!(!anim.finished);
    }

    #[test]
    fn test_current_sprite() {
        let mut anim = Animation::new("test", 0.1, false);
        anim.add_frame("sprite_1");
        assert_eq!(anim.current_sprite(), Some("sprite_1"));
    }

    #[test]
    fn test_progress() {
        let mut anim = Animation::new("test", 0.1, false);
        anim.add_frame("frame_1");
        anim.add_frame("frame_2");
        assert!((anim.progress() - 0.0).abs() < f32::EPSILON);
        anim.update(0.15);
        assert!(anim.progress() >= 0.5);
    }

    #[test]
    fn test_play_stop() {
        let mut anim = Animation::new("test", 0.1, false);
        anim.add_frame("frame_1");
        anim.stop();
        assert!(!anim.playing);
        anim.play();
        assert!(anim.playing);
    }

    #[test]
    fn test_create_attack_animation() {
        let anim = AnimationManager::create_attack_animation("TwinBlade");
        assert_eq!(anim.frames.len(), 4);
    }

    #[test]
    fn test_create_hit_animation() {
        let anim = AnimationManager::create_hit_animation();
        assert_eq!(anim.frames.len(), 4);
        assert!(anim.frames[1].offset_x != 0.0);
    }

    #[test]
    fn test_animation_manager() {
        let mut manager = AnimationManager::new();
        manager.add_animation(AnimationManager::create_idle_animation("TwinBlade"));
        assert_eq!(manager.animations.len(), 1);
        
        manager.update(1.0);
        assert!(manager.get("idle_TwinBlade").is_some());
    }
}