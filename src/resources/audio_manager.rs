use crate::systems::audio::{AudioClip, AudioType};

#[derive(Debug, Clone)]
pub struct AudioManager {
    pub master_volume: f32,
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub current_bgm: Option<String>,
    pub clips: Vec<AudioClip>,
    pub muted: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            bgm_volume: 0.7,
            sfx_volume: 1.0,
            current_bgm: None,
            clips: Vec::new(),
            muted: false,
        }
    }

    pub fn play_bgm(&mut self, clip_id: &str) {
        if self.current_bgm.as_deref() != Some(clip_id) {
            self.current_bgm = Some(clip_id.to_string());
        }
    }

    pub fn play_sfx(&mut self, clip_id: &str) {
        if !self.muted {
            self.clips.push(AudioClip {
                id: clip_id.to_string(),
                audio_type: AudioType::SFX,
                volume: self.sfx_volume * self.master_volume,
                loop_: false,
            });
        }
    }

    pub fn get_effective_volume(&self, audio_type: AudioType) -> f32 {
        if self.muted {
            return 0.0;
        }
        let type_volume = match audio_type {
            AudioType::BGM => self.bgm_volume,
            AudioType::SFX => self.sfx_volume,
            AudioType::Ambient => self.bgm_volume * 0.5,
            AudioType::Voice => self.sfx_volume,
        };
        type_volume * self.master_volume
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_audio_manager() {
        let am = AudioManager::new();
        assert_eq!(am.master_volume, 1.0);
        assert!(!am.muted);
    }

    #[test]
    fn test_play_bgm() {
        let mut am = AudioManager::new();
        am.play_bgm("bgm_field");
        assert_eq!(am.current_bgm, Some("bgm_field".to_string()));
    }

    #[test]
    fn test_mute() {
        let mut am = AudioManager::new();
        am.toggle_mute();
        assert!(am.muted);
        assert_eq!(am.get_effective_volume(AudioType::SFX), 0.0);
    }

    #[test]
    fn test_effective_volume() {
        let am = AudioManager::new();
        let vol = am.get_effective_volume(AudioType::BGM);
        assert!((vol - 0.7).abs() < f32::EPSILON);
    }
}
