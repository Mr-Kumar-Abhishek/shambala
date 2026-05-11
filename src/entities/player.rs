use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::stats::Stats;
use crate::components::render::{Renderable, RenderLayer};
use crate::components::skill::{Skill, SkillSet};
use crate::components::inventory::Inventory;
use crate::components::data_drain::DataDrain;
use crate::components::party::Party;
use crate::core::types::{Class, Element};

pub struct PlayerEntity;

impl PlayerEntity {
    pub fn create(name: &str, class: Class) -> (Player, Stats, Position, Renderable, SkillSet, Inventory, DataDrain, Party) {
        let player = Player::new(name, class);
        let stats = Stats::new(class);
        let position = Position::new(400.0, 300.0);
        let renderable = Renderable::new(
            &format!("player_{:?}", class),
            RenderLayer::Characters,
        );
        let skill_set = Self::create_skill_set(class);
        let inventory = Inventory::new(20);
        let data_drain = DataDrain::new();
        let party = Party::new(3);

        (player, stats, position, renderable, skill_set, inventory, data_drain, party)
    }

    fn create_skill_set(class: Class) -> SkillSet {
        let mut set = SkillSet::new(6);
        match class {
            Class::TwinBlade => {
                let _ = set.add_skill(Skill::new("twin_slash", "Twin Slash", Element::Null, 20, 5, 1.0));
                let _ = set.add_skill(Skill::new("wind_slash", "Wind Slash", Element::Wind, 35, 10, 3.0));
                let _ = set.add_skill(Skill::new("swift_blade", "Swift Blade", Element::Null, 15, 3, 0.5));
            }
            Class::HeavyBlade => {
                let _ = set.add_skill(Skill::new("heavy_strike", "Heavy Strike", Element::Null, 40, 10, 2.0));
                let _ = set.add_skill(Skill::new("earth_crush", "Earth Crush", Element::Earth, 55, 15, 4.0));
                let _ = set.add_skill(Skill::new("defense_up", "Defense Up", Element::Null, 0, 8, 5.0));
            }
            Class::LongArm => {
                let _ = set.add_skill(Skill::new("spiral_stab", "Spiral Stab", Element::Null, 25, 6, 1.5));
                let _ = set.add_skill(Skill::new("flame_lance", "Flame Lance", Element::Fire, 45, 12, 3.5));
                let _ = set.add_skill(Skill::new("ice_spear", "Ice Spear", Element::Water, 40, 12, 3.5));
            }
            Class::Wavemaster => {
                let _ = set.add_skill(Skill::new("fireball", "Fireball", Element::Fire, 50, 15, 3.0));
                let _ = set.add_skill(Skill::new("heal", "Heal", Element::Light, 0, 20, 4.0));
                let _ = set.add_skill(Skill::new("thunder_bolt", "Thunder Bolt", Element::Wind, 60, 18, 4.5));
            }
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_twin_blade() {
        let (player, stats, pos, _, skills, _, _, _) = PlayerEntity::create("Kite", Class::TwinBlade);
        assert_eq!(player.name, "Kite");
        assert_eq!(player.class, Class::TwinBlade);
        assert_eq!(stats.class, Class::TwinBlade);
        assert_eq!(pos.x, 400.0);
        assert_eq!(skills.skills.len(), 3);
    }

    #[test]
    fn test_create_wavemaster() {
        let (player, stats, _, _, skills, _, _, _) = PlayerEntity::create("Misty", Class::Wavemaster);
        assert_eq!(player.name, "Misty");
        assert_eq!(stats.class, Class::Wavemaster);
        assert_eq!(skills.skills.len(), 3);
    }

    #[test]
    fn test_all_classes_have_skills() {
        for class in &[Class::TwinBlade, Class::HeavyBlade, Class::LongArm, Class::Wavemaster] {
            let (_, _, _, _, skills, _, _, _) = PlayerEntity::create("Test", *class);
            assert!(!skills.skills.is_empty(), "{:?} has no skills", class);
        }
    }
}
