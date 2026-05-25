//! High-level game screens, state managers, and systems.
//!
//! | Module | Description |
//! |---|---|
//! | [`engine`] | Main [`GameEngine`] that owns all resources & runs the loop |
//! | [`scene`] | Scene registry and loading/unloading |
//! | [`event`] | Event bus for intra-engine communication |
//! | [`title_screen`] | Animated title menu with New Game / Continue / Options |
//! | [`character_select`] | Class picker + name entry |
//! | [`chaos_gate`] | Keyword-based procedural area generator |
//! | [`quest`] | Quest definitions, objectives, rewards, dialogue nodes |
//! | [`save_load`] | RON-based save/load with 3 slots |
//! | [`network`] | Simulated network layer with latency & packet loss |
//! | [`dialogue`] | NPC dialogue with typing animation and branching choices |
//! | [`options_menu`] | Settings: audio sliders, controls, display |
//! | [`progression`] | Level-up screen, skill trees per class |

pub mod chaos_gate;
pub mod character_select;
pub mod dialogue;
pub mod engine;
pub mod event;
pub mod network;
pub mod options_menu;
pub mod progression;
pub mod quest;
pub mod save_load;
pub mod scene;
pub mod title_screen;
