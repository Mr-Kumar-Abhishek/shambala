#[derive(Debug, Clone)]
pub struct DamageNumber {
    pub value: u32,
    pub x: f32,
    pub y: f32,
    pub target_y: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub is_critical: bool,
    pub is_heal: bool,
}

impl DamageNumber {
    pub fn new(value: u32, x: f32, y: f32, is_critical: bool, is_heal: bool) -> Self {
        Self {
            value,
            x,
            y: y - 10.0,
            target_y: y - 50.0,
            lifetime: 0.0,
            max_lifetime: 1.0,
            is_critical,
            is_heal,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime += dt;
        // Float upward
        self.y = self.y + (self.target_y - self.y) * dt * 3.0;
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime >= self.max_lifetime
    }

    pub fn alpha(&self) -> f32 {
        1.0 - (self.lifetime / self.max_lifetime)
    }

    pub fn color(&self) -> [f32; 4] {
        if self.is_heal {
            [0.0, 1.0, 0.0, self.alpha()]
        } else if self.is_critical {
            [1.0, 0.5, 0.0, self.alpha()]
        } else {
            [1.0, 0.2, 0.2, self.alpha()]
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScreenShake {
    pub intensity: f32,
    pub duration: f32,
    pub elapsed: f32,
    pub active: bool,
}

impl ScreenShake {
    pub fn new(intensity: f32, duration: f32) -> Self {
        Self {
            intensity,
            duration,
            elapsed: 0.0,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f32) -> (f32, f32) {
        if !self.active {
            return (0.0, 0.0);
        }

        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.active = false;
            return (0.0, 0.0);
        }

        let decay = 1.0 - (self.elapsed / self.duration);
        let current_intensity = self.intensity * decay;
        let offset_x = (rand::random::<f32>() - 0.5) * 2.0 * current_intensity;
        let offset_y = (rand::random::<f32>() - 0.5) * 2.0 * current_intensity;

        (offset_x, offset_y)
    }

    pub fn is_finished(&self) -> bool {
        !self.active || self.elapsed >= self.duration
    }
}

pub struct EffectsManager {
    pub damage_numbers: Vec<DamageNumber>,
    pub screen_shakes: Vec<ScreenShake>,
}

impl EffectsManager {
    pub fn new() -> Self {
        Self {
            damage_numbers: Vec::new(),
            screen_shakes: Vec::new(),
        }
    }

    pub fn spawn_damage_number(&mut self, value: u32, x: f32, y: f32, is_critical: bool, is_heal: bool) {
        self.damage_numbers.push(DamageNumber::new(value, x, y, is_critical, is_heal));
    }

    pub fn shake_screen(&mut self, intensity: f32, duration: f32) {
        self.screen_shakes.push(ScreenShake::new(intensity, duration));
    }

    pub fn update(&mut self, dt: f32) {
        // Update damage numbers
        for number in &mut self.damage_numbers {
            number.update(dt);
        }
        self.damage_numbers.retain(|n| !n.is_expired());

        // Update screen shakes
        self.screen_shakes.retain(|s| !s.is_finished());
        for shake in &mut self.screen_shakes {
            shake.update(dt);
        }
    }

    pub fn get_camera_offset(&mut self) -> (f32, f32) {
        let mut total_x = 0.0;
        let mut total_y = 0.0;
        for shake in &mut self.screen_shakes {
            let (ox, oy) = shake.update(0.0);
            total_x += ox;
            total_y += oy;
        }
        (total_x, total_y)
    }

    pub fn damage_count(&self) -> usize {
        self.damage_numbers.len()
    }

    pub fn clear(&mut self) {
        self.damage_numbers.clear();
        self.screen_shakes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_number_creation() {
        let dn = DamageNumber::new(50, 100.0, 100.0, false, false);
        assert_eq!(dn.value, 50);
        assert!(!dn.is_critical);
        assert!(!dn.is_expired());
    }

    #[test]
    fn test_damage_number_expiry() {
        let mut dn = DamageNumber::new(50, 0.0, 0.0, false, false);
        dn.lifetime = 1.5;
        assert!(dn.is_expired());
    }

    #[test]
    fn test_heal_color() {
        let dn = DamageNumber::new(50, 0.0, 0.0, false, true);
        assert_eq!(dn.color()[0], 0.0); // Green tint
    }

    #[test]
    fn test_critical_color() {
        let dn = DamageNumber::new(50, 0.0, 0.0, true, false);
        assert_eq!(dn.color()[0], 1.0); // Orange tint
    }

    #[test]
    fn test_screen_shake() {
        let mut shake = ScreenShake::new(5.0, 0.5);
        assert!(shake.active);
        let (ox, oy) = shake.update(0.1);
        assert!(ox != 0.0 || oy != 0.0);
    }

    #[test]
    fn test_screen_shake_expiry() {
        let mut shake = ScreenShake::new(5.0, 0.5);
        shake.update(0.6);
        assert!(shake.is_finished());
    }

    #[test]
    fn test_effects_manager() {
        let mut manager = EffectsManager::new();
        manager.spawn_damage_number(100, 50.0, 50.0, true, false);
        manager.shake_screen(10.0, 0.3);
        assert_eq!(manager.damage_count(), 1);
        assert_eq!(manager.screen_shakes.len(), 1);
    }

    #[test]
    fn test_effects_update() {
        let mut manager = EffectsManager::new();
        manager.spawn_damage_number(50, 0.0, 0.0, false, false);
        manager.update(2.0); // Past expiry
        assert!(manager.damage_numbers.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut manager = EffectsManager::new();
        manager.spawn_damage_number(50, 0.0, 0.0, false, false);
        manager.shake_screen(5.0, 0.5);
        manager.clear();
        assert!(manager.damage_numbers.is_empty());
        assert!(manager.screen_shakes.is_empty());
    }
}