//! Rendering pipeline and helpers.
//!
//! Wraps the raw `wgpu` API into high-level render primitives.
//!
//! | Module | Description |
//! |---|---|
//! | [`pipeline`] | WGPU pipeline creation, resize, and management |
//! | [`sprite`] | Sprite batching with layer sorting & culling |
//! | [`tilemap`] | Tilemap rendering from procedural [`GeneratedArea`](crate::systems::area_gen::GeneratedArea) |
//! | [`ui_render`] | UI element rendering (panels, buttons, progress bars) |
//! | [`text`] | Bitmap font text rendering with wrapping |
//! | [`texture`] | Texture manager with ID tracking |
//! | [`atlas`] | Sprite atlas / texture region management |

pub mod atlas;
pub mod pipeline;
pub mod sprite;
pub mod text;
pub mod texture;
pub mod tilemap;
pub mod ui_render;
