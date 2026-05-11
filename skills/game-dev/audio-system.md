# Skill: Audio System

## Description
How to implement audio playback using `rodio` for BGM and SFX in the Shambala game.

## Prerequisites
- Understanding of the `rodio` crate (OutputStream, Sink, Decoder)
- Familiarity with OGG/WAV audio formats
- Basic understanding of spatial audio concepts

## Steps

### 1. Initialize rodio OutputStream
```rust
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    bgm_sink: Option<Sink>,
    sfx_sinks: Vec<Sink>,
    master_volume: f32,
    bgm_volume: f32,
    sfx_volume: f32,
    muted: bool,
}

impl AudioManager {
    pub fn new() -> Result<Self, rodio::StreamError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            _stream,
            stream_handle,
            bgm_sink: None,
            sfx_sinks: Vec::new(),
            master_volume: 1.0,
            bgm_volume: 1.0,
            sfx_volume: 1.0,
            muted: false,
        })
    }
}
```

### 2. Load and Play BGM
```rust
use std::fs::File;
use std::io::BufReader;
use rodio::Source;

impl AudioManager {
    pub fn play_bgm(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Stop any existing BGM
        if let Some(sink) = self.bgm_sink.take() {
            sink.stop();
        }

        let file = File::open(path)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        let sink = Sink::try_new(&self.stream_handle)?;

        // Apply volume
        let effective_volume = if self.muted {
            0.0
        } else {
            self.master_volume * self.bgm_volume
        };
        sink.set_volume(effective_volume);
        sink.append(source);
        self.bgm_sink = Some(sink);
        Ok(())
    }

    pub fn stop_bgm(&mut self) {
        if let Some(sink) = self.bgm_sink.take() {
            sink.stop();
        }
    }

    pub fn is_bgm_playing(&self) -> bool {
        self.bgm_sink.as_ref().map_or(false, |s| !s.empty())
    }
}
```

### 3. Play SFX
```rust
impl AudioManager {
    pub fn play_sfx(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        let sink = Sink::try_new(&self.stream_handle)?;

        let effective_volume = if self.muted {
            0.0
        } else {
            self.master_volume * self.sfx_volume
        };
        sink.set_volume(effective_volume);
        sink.append(source);

        // Clean up finished sinks
        self.sfx_sinks.retain(|s| !s.empty());
        self.sfx_sinks.push(sink);
        Ok(())
    }
}
```

### 4. Volume Control
```rust
impl AudioManager {
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
        self.apply_volumes();
    }

    pub fn set_bgm_volume(&mut self, volume: f32) {
        self.bgm_volume = volume.clamp(0.0, 1.0);
        self.apply_volumes();
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
        self.apply_volumes();
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        self.apply_volumes();
    }

    fn apply_volumes(&self) {
        let bgm_vol = if self.muted { 0.0 } else { self.master_volume * self.bgm_volume };
        if let Some(sink) = &self.bgm_sink {
            sink.set_volume(bgm_vol);
        }
        let sfx_vol = if self.muted { 0.0 } else { self.master_volume * self.sfx_volume };
        for sink in &self.sfx_sinks {
            sink.set_volume(sfx_vol);
        }
    }
}
```

### 5. Spatial Audio (Distance-Based Falloff)
```rust
impl AudioManager {
    /// Play SFX with distance-based volume attenuation.
    /// `listener_pos` and `source_pos` are 2D coordinates.
    pub fn play_sfx_spatial(
        &mut self,
        path: &str,
        listener_pos: (f32, f32),
        source_pos: (f32, f32),
        max_distance: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dx = listener_pos.0 - source_pos.0;
        let dy = listener_pos.1 - source_pos.1;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > max_distance {
            return Ok(()); // Too far to hear
        }

        let attenuation = 1.0 - (distance / max_distance);
        let file = File::open(path)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        let sink = Sink::try_new(&self.stream_handle)?;

        let effective_volume = if self.muted {
            0.0
        } else {
            self.master_volume * self.sfx_volume * attenuation
        };
        sink.set_volume(effective_volume);
        sink.append(source);
        self.sfx_sinks.retain(|s| !s.empty());
        self.sfx_sinks.push(sink);
        Ok(())
    }
}
```

## Testing
- **Playback:** Call `play_bgm()` / `play_sfx()`, verify `is_bgm_playing()` returns true
- **Volume control:** Set volumes to 0.0, 0.5, 1.0, verify sink volumes are updated
- **Mute toggle:** Toggle mute on/off, verify effective volume is 0.0 or restored
- **Spatial audio:** Place source at varying distances, verify attenuation is correct
