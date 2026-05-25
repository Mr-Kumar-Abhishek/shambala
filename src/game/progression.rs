use crate::components::skill::Skill;
use crate::components::stats::Stats;
use crate::core::types::{Class, Element};
use crate::game::event::{EventBus, GameEvent};

#[derive(Debug, Clone)]
pub struct LevelUpScreen {
    pub is_active: bool,
    pub previous_level: u32,
    pub new_level: u32,
    pub stat_increases: StatIncreases,
    pub new_skills_unlocked: Vec<Skill>,
    pub animation_progress: f32,
    pub confirmed: bool,
}

#[derive(Debug, Clone)]
pub struct StatIncreases {
    pub max_hp: u32,
    pub max_mp: u32,
    pub attack: u32,
    pub defense: u32,
    pub magic_attack: u32,
    pub magic_defense: u32,
    pub agility: u32,
}

impl Default for StatIncreases {
    fn default() -> Self {
        Self::new()
    }
}

impl StatIncreases {
    pub fn new() -> Self {
        Self {
            max_hp: 10,
            max_mp: 5,
            attack: 2,
            defense: 1,
            magic_attack: 2,
            magic_defense: 1,
            agility: 1,
        }
    }

    pub fn total_stats_gained(&self) -> u32 {
        self.max_hp
            + self.max_mp
            + self.attack
            + self.defense
            + self.magic_attack
            + self.magic_defense
            + self.agility
    }
}

impl LevelUpScreen {
    pub fn new(stats: &Stats) -> Self {
        Self {
            is_active: true,
            previous_level: stats.level - 1,
            new_level: stats.level,
            stat_increases: StatIncreases::new(),
            new_skills_unlocked: Vec::new(),
            animation_progress: 0.0,
            confirmed: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_active || self.confirmed {
            return;
        }
        self.animation_progress = (self.animation_progress + dt * 0.5).min(1.0);
    }

    pub fn confirm(&mut self) {
        self.confirmed = true;
        self.is_active = false;
    }

    pub fn get_display_stats(&self) -> Vec<(&'static str, u32, u32)> {
        vec![
            ("Max HP", self.stat_increases.max_hp, 10),
            ("Max MP", self.stat_increases.max_mp, 5),
            ("Attack", self.stat_increases.attack, 2),
            ("Defense", self.stat_increases.defense, 1),
            ("Magic Attack", self.stat_increases.magic_attack, 2),
            ("Magic Defense", self.stat_increases.magic_defense, 1),
            ("Agility", self.stat_increases.agility, 1),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct SkillTreeNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub skill: Skill,
    pub required_level: u32,
    pub required_nodes: Vec<String>,
    pub unlocked: bool,
    pub position_x: f32,
    pub position_y: f32,
}

pub struct SkillTree {
    pub nodes: Vec<SkillTreeNode>,
    pub available_points: u32,
    pub class: Class,
}

impl SkillTree {
    pub fn new(class: Class) -> Self {
        let nodes = Self::create_skill_tree(class);
        Self {
            nodes,
            available_points: 0,
            class,
        }
    }

    fn create_skill_tree(class: Class) -> Vec<SkillTreeNode> {
        match class {
            Class::TwinBlade => vec![
                SkillTreeNode {
                    id: "twin_slash".to_string(),
                    name: "Twin Slash".to_string(),
                    description: "A quick double slash attack".to_string(),
                    skill: Skill::new("twin_slash", "Twin Slash", Element::Null, 20, 5, 1.0),
                    required_level: 1,
                    required_nodes: vec![],
                    unlocked: true,
                    position_x: 100.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "wind_slash".to_string(),
                    name: "Wind Slash".to_string(),
                    description: "A wind-element slash attack".to_string(),
                    skill: Skill::new("wind_slash", "Wind Slash", Element::Wind, 35, 10, 3.0),
                    required_level: 3,
                    required_nodes: vec!["twin_slash".to_string()],
                    unlocked: false,
                    position_x: 250.0,
                    position_y: 150.0,
                },
                SkillTreeNode {
                    id: "swift_blade".to_string(),
                    name: "Swift Blade".to_string(),
                    description: "A lightning-fast blade strike".to_string(),
                    skill: Skill::new("swift_blade", "Swift Blade", Element::Null, 15, 3, 0.5),
                    required_level: 5,
                    required_nodes: vec!["wind_slash".to_string()],
                    unlocked: false,
                    position_x: 400.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "cyclone".to_string(),
                    name: "Cyclone".to_string(),
                    description: "A spinning wind attack that hits all enemies".to_string(),
                    skill: Skill::new("cyclone", "Cyclone", Element::Wind, 50, 20, 6.0),
                    required_level: 10,
                    required_nodes: vec!["swift_blade".to_string()],
                    unlocked: false,
                    position_x: 550.0,
                    position_y: 250.0,
                },
            ],
            Class::HeavyBlade => vec![
                SkillTreeNode {
                    id: "heavy_strike".to_string(),
                    name: "Heavy Strike".to_string(),
                    description: "A powerful overhead strike".to_string(),
                    skill: Skill::new("heavy_strike", "Heavy Strike", Element::Null, 40, 10, 2.0),
                    required_level: 1,
                    required_nodes: vec![],
                    unlocked: true,
                    position_x: 100.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "earth_crush".to_string(),
                    name: "Earth Crush".to_string(),
                    description: "Smashes the ground with earth element".to_string(),
                    skill: Skill::new("earth_crush", "Earth Crush", Element::Earth, 55, 15, 4.0),
                    required_level: 3,
                    required_nodes: vec!["heavy_strike".to_string()],
                    unlocked: false,
                    position_x: 250.0,
                    position_y: 150.0,
                },
                SkillTreeNode {
                    id: "defense_up".to_string(),
                    name: "Defense Up".to_string(),
                    description: "Temporarily increases defense".to_string(),
                    skill: Skill::new("defense_up", "Defense Up", Element::Null, 0, 8, 5.0),
                    required_level: 5,
                    required_nodes: vec!["earth_crush".to_string()],
                    unlocked: false,
                    position_x: 400.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "titan_cleave".to_string(),
                    name: "Titan Cleave".to_string(),
                    description: "A devastating earth-element cleave".to_string(),
                    skill: Skill::new("titan_cleave", "Titan Cleave", Element::Earth, 80, 25, 8.0),
                    required_level: 10,
                    required_nodes: vec!["defense_up".to_string()],
                    unlocked: false,
                    position_x: 550.0,
                    position_y: 250.0,
                },
            ],
            Class::LongArm => vec![
                SkillTreeNode {
                    id: "spiral_stab".to_string(),
                    name: "Spiral Stab".to_string(),
                    description: "A piercing spiral attack".to_string(),
                    skill: Skill::new("spiral_stab", "Spiral Stab", Element::Null, 25, 6, 1.5),
                    required_level: 1,
                    required_nodes: vec![],
                    unlocked: true,
                    position_x: 100.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "flame_lance".to_string(),
                    name: "Flame Lance".to_string(),
                    description: "A fire-element lance thrust".to_string(),
                    skill: Skill::new("flame_lance", "Flame Lance", Element::Fire, 45, 12, 3.5),
                    required_level: 3,
                    required_nodes: vec!["spiral_stab".to_string()],
                    unlocked: false,
                    position_x: 250.0,
                    position_y: 150.0,
                },
                SkillTreeNode {
                    id: "ice_spear".to_string(),
                    name: "Ice Spear".to_string(),
                    description: "A water-element spear attack".to_string(),
                    skill: Skill::new("ice_spear", "Ice Spear", Element::Water, 40, 12, 3.5),
                    required_level: 5,
                    required_nodes: vec!["flame_lance".to_string()],
                    unlocked: false,
                    position_x: 400.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "dragon_thrust".to_string(),
                    name: "Dragon Thrust".to_string(),
                    description: "A legendary dragon-element thrust".to_string(),
                    skill: Skill::new("dragon_thrust", "Dragon Thrust", Element::Fire, 70, 22, 7.0),
                    required_level: 10,
                    required_nodes: vec!["ice_spear".to_string()],
                    unlocked: false,
                    position_x: 550.0,
                    position_y: 250.0,
                },
            ],
            Class::Wavemaster => vec![
                SkillTreeNode {
                    id: "fireball".to_string(),
                    name: "Fireball".to_string(),
                    description: "A basic fire spell".to_string(),
                    skill: Skill::new("fireball", "Fireball", Element::Fire, 50, 15, 3.0),
                    required_level: 1,
                    required_nodes: vec![],
                    unlocked: true,
                    position_x: 100.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "heal".to_string(),
                    name: "Heal".to_string(),
                    description: "Restores HP to an ally".to_string(),
                    skill: Skill::new("heal", "Heal", Element::Light, 0, 20, 4.0),
                    required_level: 3,
                    required_nodes: vec!["fireball".to_string()],
                    unlocked: false,
                    position_x: 250.0,
                    position_y: 150.0,
                },
                SkillTreeNode {
                    id: "thunder_bolt".to_string(),
                    name: "Thunder Bolt".to_string(),
                    description: "A powerful wind-element lightning spell".to_string(),
                    skill: Skill::new("thunder_bolt", "Thunder Bolt", Element::Wind, 60, 18, 4.5),
                    required_level: 5,
                    required_nodes: vec!["heal".to_string()],
                    unlocked: false,
                    position_x: 400.0,
                    position_y: 200.0,
                },
                SkillTreeNode {
                    id: "meteor".to_string(),
                    name: "Meteor".to_string(),
                    description: "An ultimate fire-element spell".to_string(),
                    skill: Skill::new("meteor", "Meteor", Element::Fire, 100, 35, 10.0),
                    required_level: 10,
                    required_nodes: vec!["thunder_bolt".to_string()],
                    unlocked: false,
                    position_x: 550.0,
                    position_y: 250.0,
                },
            ],
        }
    }

    pub fn unlock_skill(&mut self, node_id: &str) -> Result<(), &str> {
        if self.available_points == 0 {
            return Err("No skill points available");
        }

        // Find the node index first using immutable borrow
        let node_idx = self.nodes.iter().position(|n| n.id == node_id);

        if let Some(idx) = node_idx {
            if self.nodes[idx].unlocked {
                return Err("Skill already unlocked");
            }

            // Check if all required nodes are unlocked (immutable borrow)
            let required_nodes = self.nodes[idx].required_nodes.clone();
            for req_id in &required_nodes {
                if !self
                    .nodes
                    .iter()
                    .any(|n| n.id == req_id.as_str() && n.unlocked)
                {
                    return Err("Required skills not unlocked");
                }
            }

            // Now mutate
            self.nodes[idx].unlocked = true;
            self.available_points -= 1;
            Ok(())
        } else {
            Err("Skill not found")
        }
    }

    pub fn get_unlocked_skills(&self) -> Vec<&SkillTreeNode> {
        self.nodes.iter().filter(|n| n.unlocked).collect()
    }

    pub fn get_available_skills(&self) -> Vec<&SkillTreeNode> {
        self.nodes
            .iter()
            .filter(|n| !n.unlocked)
            .filter(|n| {
                n.required_nodes.is_empty()
                    || n.required_nodes
                        .iter()
                        .all(|req_id| self.nodes.iter().any(|n2| n2.id == *req_id && n2.unlocked))
            })
            .collect()
    }

    pub fn add_skill_point(&mut self) {
        self.available_points += 1;
    }

    pub fn can_unlock(&self, node_id: &str) -> bool {
        if self.available_points == 0 {
            return false;
        }
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            if node.unlocked {
                return false;
            }
            node.required_nodes.is_empty()
                || node
                    .required_nodes
                    .iter()
                    .all(|req_id| self.nodes.iter().any(|n| n.id == *req_id && n.unlocked))
        } else {
            false
        }
    }

    pub fn progress_percentage(&self) -> f32 {
        let unlocked = self.nodes.iter().filter(|n| n.unlocked).count();
        if self.nodes.is_empty() {
            0.0
        } else {
            unlocked as f32 / self.nodes.len() as f32
        }
    }
}

pub struct ProgressionSystem;

impl ProgressionSystem {
    pub fn apply_level_up(
        stats: &mut Stats,
        skill_tree: &mut SkillTree,
        event_bus: &mut EventBus,
    ) -> LevelUpScreen {
        // Add skill point
        skill_tree.add_skill_point();

        // Create level-up screen
        let mut screen = LevelUpScreen::new(stats);

        // Check for new unlockable skills
        let available = skill_tree.get_available_skills();
        for node in &available {
            if node.required_level <= stats.level {
                screen.new_skills_unlocked.push(node.skill.clone());
            }
        }

        event_bus.emit(GameEvent::LevelUp {
            new_level: stats.level,
        });

        screen
    }

    pub fn check_level_up(
        stats: &Stats,
        exp_gained: u64,
        skill_tree: &mut SkillTree,
        event_bus: &mut EventBus,
    ) -> Option<LevelUpScreen> {
        let mut temp_stats = stats.clone();
        temp_stats.add_experience(exp_gained);

        if temp_stats.level > stats.level {
            Some(Self::apply_level_up(&mut temp_stats, skill_tree, event_bus))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_up_screen_creation() {
        let stats = Stats::new(Class::TwinBlade);
        let screen = LevelUpScreen::new(&stats);
        assert!(screen.is_active);
        assert!(!screen.confirmed);
    }

    #[test]
    fn test_level_up_confirm() {
        let stats = Stats::new(Class::TwinBlade);
        let mut screen = LevelUpScreen::new(&stats);
        screen.confirm();
        assert!(!screen.is_active);
        assert!(screen.confirmed);
    }

    #[test]
    fn test_stat_increases() {
        let increases = StatIncreases::new();
        assert_eq!(increases.max_hp, 10);
        assert!(increases.total_stats_gained() > 0);
    }

    #[test]
    fn test_skill_tree_creation() {
        for class in &[
            Class::TwinBlade,
            Class::HeavyBlade,
            Class::LongArm,
            Class::Wavemaster,
        ] {
            let tree = SkillTree::new(*class);
            assert_eq!(tree.nodes.len(), 4, "{:?} should have 4 skills", class);
            assert!(tree.nodes[0].unlocked, "First skill should be unlocked");
        }
    }

    #[test]
    fn test_unlock_skill() {
        let mut tree = SkillTree::new(Class::TwinBlade);
        tree.available_points = 1;

        // Try to unlock second skill (requires first)
        assert!(tree.unlock_skill("wind_slash").is_ok());
        assert!(tree.nodes[1].unlocked);
    }

    #[test]
    fn test_unlock_without_points() {
        let mut tree = SkillTree::new(Class::TwinBlade);
        assert!(tree.unlock_skill("wind_slash").is_err());
    }

    #[test]
    fn test_can_unlock() {
        let mut tree = SkillTree::new(Class::TwinBlade);
        tree.available_points = 1;
        assert!(tree.can_unlock("wind_slash"));
        assert!(!tree.can_unlock("cyclone")); // Requires chain
    }

    #[test]
    fn test_get_available_skills() {
        let tree = SkillTree::new(Class::Wavemaster);
        let available = tree.get_available_skills();
        assert_eq!(available.len(), 1); // Only heal (requires fireball which is unlocked)
        assert_eq!(available[0].id, "heal");
    }

    #[test]
    fn test_progress_percentage() {
        let tree = SkillTree::new(Class::TwinBlade);
        assert!((tree.progress_percentage() - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn test_apply_level_up() {
        let mut stats = Stats::new(Class::TwinBlade);
        let mut tree = SkillTree::new(Class::TwinBlade);
        let mut event_bus = EventBus::new(100);

        stats.add_experience(100); // Level up to 2
        let screen = ProgressionSystem::apply_level_up(&mut stats, &mut tree, &mut event_bus);
        assert_eq!(screen.new_level, 2);
        assert_eq!(tree.available_points, 1);
    }

    #[test]
    fn test_check_level_up() {
        let stats = Stats::new(Class::TwinBlade);
        let mut tree = SkillTree::new(Class::TwinBlade);
        let mut event_bus = EventBus::new(100);

        let result = ProgressionSystem::check_level_up(&stats, 100, &mut tree, &mut event_bus);
        assert!(result.is_some());
    }

    #[test]
    fn test_no_level_up() {
        let stats = Stats::new(Class::TwinBlade);
        let mut tree = SkillTree::new(Class::TwinBlade);
        let mut event_bus = EventBus::new(100);

        let result = ProgressionSystem::check_level_up(&stats, 5, &mut tree, &mut event_bus);
        assert!(result.is_none());
    }
}
