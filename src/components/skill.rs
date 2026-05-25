use serde::{Deserialize, Serialize};
use crate::core::types::Element;

/// A single skill/spell that a character can use in combat.
///
/// Skills have an elemental affinity, base power, MP cost, and
/// a cooldown timer that prevents spam.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Unique identifier (e.g. `"fireball"`, `"heal"`).
    pub id: String,
    /// Human-readable display name.
    pub name: String,
    /// Elemental affinity used for damage typing.
    pub element: Element,
    /// Base power added to the attacker's relevant stat.
    pub power: u32,
    /// MP cost to use this skill.
    pub mp_cost: u32,
    /// Total cooldown duration in seconds.
    pub cooldown: f32,
    /// Remaining cooldown; decremented each frame.
    pub current_cooldown: f32,
    /// Flavour text / tooltip.
    pub description: String,
}

impl Skill {
    /// Create a new skill.
    ///
    /// `cooldown` is the total cooldown in seconds; the skill starts
    /// ready to use.
    pub fn new(id: &str, name: &str, element: Element, power: u32, mp_cost: u32, cooldown: f32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            element,
            power,
            mp_cost,
            cooldown,
            current_cooldown: 0.0,
            description: String::new(),
        }
    }

    /// Returns `true` if the skill is off cooldown and can be used.
    pub fn is_ready(&self) -> bool {
        self.current_cooldown <= 0.0
    }

    /// Trigger the skill, putting it on cooldown.
    pub fn use_skill(&mut self) {
        self.current_cooldown = self.cooldown;
    }

    /// Advance the cooldown timer by `dt` seconds.
    pub fn update_cooldown(&mut self, dt: f32) {
        if self.current_cooldown > 0.0 {
            self.current_cooldown = (self.current_cooldown - dt).max(0.0);
        }
    }
}

/// A hot-bar of skills that a character has equipped.
#[derive(Debug, Clone)]
pub struct SkillSet {
    pub skills: Vec<Skill>,
    pub max_skills: usize,
}

impl SkillSet {
    /// Create an empty skill set with the given capacity.
    pub fn new(max_skills: usize) -> Self {
        Self {
            skills: Vec::new(),
            max_skills,
        }
    }

    /// Add a skill to the hot-bar. Returns an error if the bar is full.
    pub fn add_skill(&mut self, skill: Skill) -> Result<(), &str> {
        if self.skills.len() >= self.max_skills {
            Err("Skill set is full")
        } else {
            self.skills.push(skill);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new("fireball", "Fireball", Element::Fire, 50, 15, 3.0);
        assert_eq!(skill.name, "Fireball");
        assert!(skill.is_ready());
    }

    #[test]
    fn test_skill_cooldown() {
        let mut skill = Skill::new("fireball", "Fireball", Element::Fire, 50, 15, 3.0);
        skill.use_skill();
        assert!(!skill.is_ready());
        skill.update_cooldown(3.0);
        assert!(skill.is_ready());
    }

    #[test]
    fn test_skill_set_full() {
        let mut set = SkillSet::new(2);
        assert!(set.add_skill(Skill::new("a", "A", Element::Fire, 10, 5, 1.0)).is_ok());
        assert!(set.add_skill(Skill::new("b", "B", Element::Water, 10, 5, 1.0)).is_ok());
        assert!(set.add_skill(Skill::new("c", "C", Element::Wind, 10, 5, 1.0)).is_err());
    }
}