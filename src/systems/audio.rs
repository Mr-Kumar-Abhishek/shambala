#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioType {
    BGM,
    SFX,
    Ambient,
    Voice,
}

#[derive(Debug, Clone)]
pub struct AudioClip {
    pub id: String,
    pub audio_type: AudioType,
    pub volume: f32,
    pub loop_: bool,
}

pub struct AudioSystem;

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_volume(base_volume: f32, distance: f32, max_distance: f32) -> f32 {
        if distance >= max_distance {
            0.0
        } else {
            let normalized = 1.0 - (distance / max_distance);
            base_volume * normalized
        }
    }

    pub fn get_music_for_area(area_type: &str) -> &'static str {
        match area_type {
            "root_town" => "bgm_root_town",
            "field" => "bgm_field",
            "dungeon" => "bgm_dungeon",
            "boss" => "bgm_boss",
            "menu" => "bgm_menu",
            _ => "bgm_default",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_at_max_distance() {
        let vol = AudioSystem::calculate_volume(1.0, 100.0, 100.0);
        assert_eq!(vol, 0.0);
    }

    #[test]
    fn test_volume_at_zero_distance() {
        let vol = AudioSystem::calculate_volume(1.0, 0.0, 100.0);
        assert_eq!(vol, 1.0);
    }

    #[test]
    fn test_volume_half_distance() {
        let vol = AudioSystem::calculate_volume(1.0, 50.0, 100.0);
        assert!((vol - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_music_for_area() {
        assert_eq!(
            AudioSystem::get_music_for_area("root_town"),
            "bgm_root_town"
        );
        assert_eq!(AudioSystem::get_music_for_area("boss"), "bgm_boss");
        assert_eq!(AudioSystem::get_music_for_area("unknown"), "bgm_default");
    }
}
