use serde::{Deserialize, Serialize};
use crate::core::types::Class;

/// Combat statistics for a character (player or enemy).
///
/// Each [`Class`] has a unique starting stat profile. Stats grow on
/// level-up and are used by the [`CombatSystem`](crate::systems::combat::CombatSystem)
/// for damage calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub level: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub mp: u32,
    pub max_mp: u32,
    pub attack: u32,
    pub defense: u32,
    pub magic_attack: u32,
    pub magic_defense: u32,
    pub agility: u32,
    pub class: Class,
    pub experience: u64,
    pub next_level_exp: u64,
}

impl Stats {
    /// Create stats for the given class at level 1 with starting values.
    pub fn new(class: Class) -> Self {
        match class {
            Class::TwinBlade => Self {
                level: 1, hp: 100, max_hp: 100, mp: 30, max_mp: 30,
                attack: 15, defense: 8, magic_attack: 5, magic_defense: 6,
                agility: 20, class, experience: 0, next_level_exp: 100,
            },
            Class::HeavyBlade => Self {
                level: 1, hp: 150, max_hp: 150, mp: 20, max_mp: 20,
                attack: 20, defense: 15, magic_attack: 3, magic_defense: 10,
                agility: 8, class, experience: 0, next_level_exp: 100,
            },
            Class::LongArm => Self {
                level: 1, hp: 90, max_hp: 90, mp: 40, max_mp: 40,
                attack: 12, defense: 6, magic_attack: 12, magic_defense: 8,
                agility: 15, class, experience: 0, next_level_exp: 100,
            },
            Class::Wavemaster => Self {
                level: 1, hp: 70, max_hp: 70, mp: 80, max_mp: 80,
                attack: 5, defense: 4, magic_attack: 20, magic_defense: 15,
                agility: 12, class, experience: 0, next_level_exp: 100,
            },
        }
    }

    /// Reduce HP by `damage` (clamped to 0).
    pub fn take_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }

    /// Restore HP by `amount` (capped at max_hp).
    pub fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    /// Returns `true` if the character is still alive (HP > 0).
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Add experience and trigger automatic level-ups if thresholds are met.
    pub fn add_experience(&mut self, exp: u64) {
        self.experience += exp;
        while self.experience >= self.next_level_exp {
            self.level_up();
        }
    }

    /// Apply a single level-up: increase stats and set next EXP threshold.
    ///
    /// Base stat gains per level:
    /// - HP +10, MP +5, Attack +2, Defence +1
    pub fn level_up(&mut self) {
        self.experience -= self.next_level_exp;
        self.level += 1;
        self.max_hp += 10;
        self.hp = self.max_hp;
        self.max_mp += 5;
        self.mp = self.max_mp;
        self.attack += 2;
        self.defense += 1;
        self.next_level_exp = self.level as u64 * 100;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twin_blade_stats() {
        let stats = Stats::new(Class::TwinBlade);
        assert_eq!(stats.hp, 100);
        assert_eq!(stats.agility, 20);
    }

    #[test]
    fn test_take_damage() {
        let mut stats = Stats::new(Class::HeavyBlade);
        stats.take_damage(50);
        assert_eq!(stats.hp, 100);
    }

    #[test]
    fn test_take_damage_overflow() {
        let mut stats = Stats::new(Class::Wavemaster);
        stats.take_damage(999);
        assert_eq!(stats.hp, 0);
    }

    #[test]
    fn test_heal() {
        let mut stats = Stats::new(Class::TwinBlade);
        stats.take_damage(50);
        stats.heal(30);
        assert_eq!(stats.hp, 80);
    }

    #[test]
    fn test_heal_overflow() {
        let mut stats = Stats::new(Class::TwinBlade);
        stats.heal(999);
        assert_eq!(stats.hp, stats.max_hp);
    }

    #[test]
    fn test_is_alive() {
        let mut stats = Stats::new(Class::TwinBlade);
        assert!(stats.is_alive());
        stats.take_damage(999);
        assert!(!stats.is_alive());
    }

    #[test]
    fn test_level_up() {
        let mut stats = Stats::new(Class::TwinBlade);
        stats.add_experience(100);
        assert_eq!(stats.level, 2);
        assert_eq!(stats.max_hp, 110);
    }
}