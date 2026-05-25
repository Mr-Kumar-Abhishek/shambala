use crate::core::types::Class;

/// Human-readable metadata for the player character.
///
/// Attached to the player entity alongside [`Stats`](crate::components::stats::Stats),
/// [`Position`](crate::components::position::Position), [`SkillSet`](crate::components::skill::SkillSet),
/// and other gameplay components.
#[derive(Debug, Clone)]
pub struct Player {
    /// Character name chosen during character creation.
    pub name: String,
    /// Selected class which determines stat growth and available skills.
    pub class: Class,
    /// Tracks whether the player is connected (for the networking layer).
    pub is_online: bool,
    /// Simulated connection quality (1.0 = perfect, 0.0 = disconnected).
    pub connection_quality: f32,
}

impl Player {
    /// Create a new player with the given name and class.
    pub fn new(name: &str, class: Class) -> Self {
        Self {
            name: name.to_string(),
            class,
            is_online: true,
            connection_quality: 1.0,
        }
    }
}
