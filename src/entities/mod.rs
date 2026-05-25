//! Entity factory functions.
//!
//! Entity factories bundle one or more components together to create
//! a fully initialised game entity. For example, [`PlayerEntity::create`]
//! returns a tuple of eight components ready for use.

pub mod player;
pub mod enemy;
pub mod npc;
pub mod item;
pub mod area;
