//! Stateless game systems that operate on ECS components.
//!
//! Each system encapsulates a single concern and works with borrowed
//! component data through public associated functions.
//!
//! | System | Responsibility |
//! |---|---|
//! | [`input`] | Input action detection and mapping |
//! | [`render`] | Visibility culling and layer sorting |
//! | [`physics`] | Movement integration, collision, distance |
//! | [`combat`] | Damage calculation, critical hits |
//! | [`ai`] | Enemy state machine (idle, patrol, chase, attack) |
//! | [`party`] | EXP distribution, average level |
//! | [`data_drain`] | Data Drain execution and charging |
//! | [`area_gen`] | Procedural tile generation + BFS pathfinding |
//! | [`ui`] | HUD creation, progress bars, click detection |
//! | [`audio`] | Volume falloff, area-to-music mapping |
//! | [`audio_playback`] | BGM/SFX track management |
//! | [`animation`] | Frame-based sprite animation |
//! | [`effects`] | Damage numbers, screen shake |
//! | [`combat_integration`] | Turn-based combat encounters and VFX |
//! | [`exploration`] | Player movement, camera, collision, enemy proximity |

pub mod input;
pub mod render;
pub mod physics;
pub mod combat;
pub mod ai;
pub mod party;
pub mod data_drain;
pub mod area_gen;
pub mod ui;
pub mod audio;
pub mod audio_playback;
pub mod animation;
pub mod effects;
pub mod combat_integration;
pub mod exploration;
