use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioTrackType {
    Bgm,
    Sfx,
    Ambient,
}

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: String,
    pub track_type: AudioTrackType,
    pub file_path: String,
    pub volume: f32,
    pub loop_: bool,
}

pub struct AudioPlaybackSystem {
    pub tracks: HashMap<String, AudioTrack>,
    pub master_volume: f32,
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub current_bgm: Option<String>,
    pub muted: bool,
    pub _stream: Option<rodio::OutputStream>,
    pub _stream_handle: Option<rodio::OutputStreamHandle>,
}

impl Default for AudioPlaybackSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlaybackSystem {
    pub fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default()
            .ok()
            .map(|(s, h)| (Some(s), Some(h)))
            .unwrap_or((None, None));

        Self {
            tracks: HashMap::new(),
            master_volume: 1.0,
            bgm_volume: 0.7,
            sfx_volume: 1.0,
            current_bgm: None,
            muted: false,
            _stream: stream,
            _stream_handle: stream_handle,
        }
    }

    pub fn register_track(
        &mut self,
        id: &str,
        file_path: &str,
        track_type: AudioTrackType,
        loop_: bool,
    ) {
        self.tracks.insert(
            id.to_string(),
            AudioTrack {
                id: id.to_string(),
                track_type,
                file_path: file_path.to_string(),
                volume: 1.0,
                loop_,
            },
        );
    }

    pub fn play_bgm(&mut self, track_id: &str) {
        if self.muted {
            return;
        }
        if self.current_bgm.as_deref() != Some(track_id) {
            self.current_bgm = Some(track_id.to_string());
            log::info!("Playing BGM: {}", track_id);
        }
    }

    pub fn play_sfx(&mut self, track_id: &str) {
        if self.muted {
            return;
        }
        log::info!("Playing SFX: {}", track_id);
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_bgm_volume(&mut self, volume: f32) {
        self.bgm_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn get_effective_volume(&self, track_type: AudioTrackType) -> f32 {
        if self.muted {
            return 0.0;
        }
        let type_volume = match track_type {
            AudioTrackType::Bgm => self.bgm_volume,
            AudioTrackType::Sfx => self.sfx_volume,
            AudioTrackType::Ambient => self.bgm_volume * 0.5,
        };
        type_volume * self.master_volume
    }

    pub fn stop_bgm(&mut self) {
        self.current_bgm = None;
    }

    pub fn is_playing(&self) -> bool {
        self.current_bgm.is_some()
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_system_creation() {
        let audio = AudioPlaybackSystem::new();
        assert!((audio.master_volume - 1.0).abs() < f32::EPSILON);
        assert!(!audio.muted);
    }

    #[test]
    fn test_register_track() {
        let mut audio = AudioPlaybackSystem::new();
        audio.register_track(
            "bgm_field",
            "assets/audio/bgm/field.ogg",
            AudioTrackType::Bgm,
            true,
        );
        assert_eq!(audio.track_count(), 1);
    }

    #[test]
    fn test_play_bgm() {
        let mut audio = AudioPlaybackSystem::new();
        audio.register_track(
            "bgm_field",
            "assets/audio/bgm/field.ogg",
            AudioTrackType::Bgm,
            true,
        );
        audio.play_bgm("bgm_field");
        assert_eq!(audio.current_bgm, Some("bgm_field".to_string()));
    }

    #[test]
    fn test_volume_control() {
        let mut audio = AudioPlaybackSystem::new();
        audio.set_master_volume(0.5);
        audio.set_bgm_volume(0.8);
        let vol = audio.get_effective_volume(AudioTrackType::Bgm);
        assert!((vol - 0.4).abs() < f32::EPSILON); // 0.5 * 0.8
    }

    #[test]
    fn test_mute() {
        let mut audio = AudioPlaybackSystem::new();
        audio.toggle_mute();
        assert!(audio.muted);
        assert_eq!(audio.get_effective_volume(AudioTrackType::Sfx), 0.0);
    }

    #[test]
    fn test_stop_bgm() {
        let mut audio = AudioPlaybackSystem::new();
        audio.register_track("bgm_field", "path", AudioTrackType::Bgm, true);
        audio.play_bgm("bgm_field");
        audio.stop_bgm();
        assert!(!audio.is_playing());
    }

    #[test]
    fn test_volume_clamping() {
        let mut audio = AudioPlaybackSystem::new();
        audio.set_master_volume(2.0);
        assert!((audio.master_volume - 1.0).abs() < f32::EPSILON);
        audio.set_master_volume(-1.0);
        assert!((audio.master_volume - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sfx_volume() {
        let mut audio = AudioPlaybackSystem::new();
        audio.set_sfx_volume(0.5);
        let vol = audio.get_effective_volume(AudioTrackType::Sfx);
        assert!((vol - 0.5).abs() < f32::EPSILON);
    }
}
