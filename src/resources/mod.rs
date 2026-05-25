//! Shared singleton resources available to all systems.
//!
//! | Resource | Purpose |
//! |---|---|
//! | [`asset_manager`] | Texture / audio asset registration and loading |
//! | [`input_state`] | Per-frame input action state |
//! | [`camera`] | Viewport transform (world ↔ screen) |
//! | [`time`] | Delta time, frame count, time scale |
//! | [`audio_manager`] | BGM switching by game state |

pub mod asset_manager;
pub mod input_state;
pub mod camera;
pub mod time;
pub mod audio_manager;
