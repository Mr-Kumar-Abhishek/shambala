use crate::components::skill::Skill;
use crate::components::stats::Stats;

pub struct CombatSystem;

impl CombatSystem {
    pub fn calculate_damage(attacker: &Stats, defender: &Stats, skill: Option<&Skill>) -> u32 {
        let base_attack = if let Some(skill) = skill {
            attacker.attack + skill.power
        } else {
            attacker.attack
        };

        let defense = defender.defense;
        let raw_damage = base_attack.saturating_sub(defense / 2);

        // Minimum damage of 1
        std::cmp::max(raw_damage, 1)
    }

    pub fn calculate_magic_damage(attacker: &Stats, defender: &Stats, skill: &Skill) -> u32 {
        let base_attack = attacker.magic_attack + skill.power;
        let defense = defender.magic_defense;
        let raw_damage = base_attack.saturating_sub(defense / 2);
        std::cmp::max(raw_damage, 1)
    }

    pub fn is_critical_hit(agility: u32) -> bool {
        // Base 10% crit chance, modified by agility
        let crit_chance = 0.10 + (agility as f32 * 0.002);
        rand::random::<f32>() < crit_chance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Class, Element};

    #[test]
    fn test_basic_damage() {
        let attacker = Stats::new(Class::HeavyBlade);
        let defender = Stats::new(Class::Wavemaster);
        let damage = CombatSystem::calculate_damage(&attacker, &defender, None);
        assert!(damage >= 1);
        assert!(damage <= attacker.attack);
    }

    #[test]
    fn test_skill_damage() {
        let attacker = Stats::new(Class::Wavemaster);
        let defender = Stats::new(Class::HeavyBlade);
        let skill = Skill::new("fireball", "Fireball", Element::Fire, 50, 15, 3.0);
        let damage = CombatSystem::calculate_magic_damage(&attacker, &defender, &skill);
        assert!(damage >= 1);
    }

    #[test]
    fn test_minimum_damage() {
        let attacker = Stats::new(Class::Wavemaster);
        let mut defender = Stats::new(Class::HeavyBlade);
        defender.defense = 999;
        let damage = CombatSystem::calculate_damage(&attacker, &defender, None);
        assert_eq!(damage, 1);
    }
}
