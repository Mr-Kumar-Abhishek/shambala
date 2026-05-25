//! Core engine types, constants, and state management.
//!
//! This module provides the foundational building blocks used throughout
//! the Shambala engine:
//!
//! - [`types`] — Game states, character classes, elements, status effects
//! - [`constants`] — Window dimensions, tile size, gameplay limits
//! - [`game_state`] — Stack-based state machine for screen transitions

pub mod constants;
pub mod game_state;
pub mod types;
