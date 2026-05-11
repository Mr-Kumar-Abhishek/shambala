use serde::{Deserialize, Serialize};
use crate::core::types::Element;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub element: Element,
    pub power: u32,
    pub mp_cost: u32,
    pub cooldown: f32,
    pub current_cooldown: f32,
    pub description: String,
}

impl Skill {
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

    pub fn is_ready(&self) -> bool {
        self.current_cooldown <= 0.0
    }

    pub fn use_skill(&mut self) {
        self.current_cooldown = self.cooldown;
    }

    pub fn update_cooldown(&mut self, dt: f32) {
        if self.current_cooldown > 0.0 {
            self.current_cooldown = (self.current_cooldown - dt).max(0.0);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkillSet {
    pub skills: Vec<Skill>,
    pub max_skills: usize,
}

impl SkillSet {
    pub fn new(max_skills: usize) -> Self {
        Self {
            skills: Vec::new(),
            max_skills,
        }
    }

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