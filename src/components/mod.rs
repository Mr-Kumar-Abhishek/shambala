//! ECS component data structures.
//!
//! Components are pure-data structs that carry no logic. They are bundled
//! together by entity factories (see [`entities`](crate::entities)) and
//! processed by systems (see [`systems`](crate::systems)).
//!
//! | Component | Purpose |
//! |---|---|
//! | [`position`] | World-space coordinates & velocity |
//! | [`stats`] | HP, MP, attack, defence, level, EXP |
//! | [`render`] | Sprite ID, layer, visibility, opacity |
//! | [`player`] | Player-specific metadata (name, class) |
//! | [`enemy`] | Enemy type, aggro range, drop table |
//! | [`party`] | Party members & bond levels |
//! | [`inventory`] | Item storage, gold |
//! | [`skill`] | Skill definitions & cooldowns |
//! | [`status`] | Status effect instances (poison, paralysis, etc.) |
//! | [`data_drain`] | Data Drain charge state (special mechanic) |

pub mod position;
pub mod stats;
pub mod render;
pub mod player;
pub mod enemy;
pub mod party;
pub mod inventory;
pub mod skill;
pub mod status;
pub mod data_drain;