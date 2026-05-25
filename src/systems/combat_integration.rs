use crate::components::data_drain::DataDrain;
use crate::components::skill::Skill;
use crate::components::stats::Stats;
use crate::game::event::{EventBus, GameEvent};
use crate::systems::animation::AnimationManager;
use crate::systems::audio_playback::AudioPlaybackSystem;
use crate::systems::combat::CombatSystem;
use crate::systems::data_drain::DataDrainSystem;
use crate::systems::effects::EffectsManager;

pub struct CombatEncounter {
    pub player_stats: Stats,
    pub enemy_stats: Stats,
    pub turn_count: u32,
    pub is_player_turn: bool,
    pub is_active: bool,
    pub combat_log: Vec<String>,
}

impl CombatEncounter {
    pub fn new(player_stats: Stats, enemy_stats: Stats) -> Self {
        Self {
            player_stats,
            enemy_stats,
            turn_count: 0,
            is_player_turn: true,
            is_active: true,
            combat_log: Vec::new(),
        }
    }

    pub fn player_attack(&mut self, skill: Option<&Skill>) -> CombatActionResult {
        if !self.is_active || !self.is_player_turn {
            return CombatActionResult::invalid();
        }

        let damage = if let Some(skill) = skill {
            CombatSystem::calculate_magic_damage(&self.player_stats, &self.enemy_stats, skill)
        } else {
            CombatSystem::calculate_damage(&self.player_stats, &self.enemy_stats, None)
        };

        let critical = CombatSystem::is_critical_hit(self.player_stats.agility);
        let final_damage = if critical { damage * 2 } else { damage };

        self.enemy_stats.take_damage(final_damage);
        self.combat_log.push(format!(
            "Player deals {} damage{}!",
            final_damage,
            if critical { " (CRITICAL!)" } else { "" }
        ));

        let result = CombatActionResult {
            damage: final_damage,
            is_critical: critical,
            is_player_turn: true,
            target_defeated: !self.enemy_stats.is_alive(),
            source: "player".to_string(),
            skill_name: skill.map(|s| s.name.clone()),
        };

        if !self.enemy_stats.is_alive() {
            self.is_active = false;
            self.combat_log.push("Enemy defeated!".to_string());
        }

        self.is_player_turn = !self.enemy_stats.is_alive();
        self.turn_count += 1;
        result
    }

    pub fn enemy_attack(&mut self) -> CombatActionResult {
        if !self.is_active || self.is_player_turn {
            return CombatActionResult::invalid();
        }

        let damage = CombatSystem::calculate_damage(&self.enemy_stats, &self.player_stats, None);
        let critical = CombatSystem::is_critical_hit(self.enemy_stats.agility);
        let final_damage = if critical { damage * 2 } else { damage };

        self.player_stats.take_damage(final_damage);
        self.combat_log.push(format!(
            "Enemy deals {} damage{}!",
            final_damage,
            if critical { " (CRITICAL!)" } else { "" }
        ));

        let result = CombatActionResult {
            damage: final_damage,
            is_critical: critical,
            is_player_turn: false,
            target_defeated: !self.player_stats.is_alive(),
            source: "enemy".to_string(),
            skill_name: None,
        };

        if !self.player_stats.is_alive() {
            self.is_active = false;
            self.combat_log.push("Player defeated!".to_string());
        }

        self.is_player_turn = self.player_stats.is_alive();
        self.turn_count += 1;
        result
    }

    pub fn execute_data_drain(
        &mut self,
        data_drain: &mut DataDrain,
    ) -> (
        CombatActionResult,
        crate::systems::data_drain::DataDrainResult,
    ) {
        let hp_percentage = self.enemy_stats.hp as f32 / self.enemy_stats.max_hp as f32;
        let drain_result = DataDrainSystem::execute_drain(data_drain, hp_percentage);

        let action_result = CombatActionResult {
            damage: 0,
            is_critical: false,
            is_player_turn: true,
            target_defeated: false,
            source: "data_drain".to_string(),
            skill_name: Some("Data Drain".to_string()),
        };

        if drain_result.success {
            self.combat_log
                .push("Data Drain successful! Gained data fragment.".to_string());
            self.enemy_stats.hp = 0;
            self.is_active = false;
        }

        (action_result, drain_result)
    }

    pub fn get_log(&self) -> &[String] {
        &self.combat_log
    }

    pub fn is_over(&self) -> bool {
        !self.is_active
    }

    pub fn player_won(&self) -> bool {
        !self.is_active && !self.enemy_stats.is_alive()
    }
}

#[derive(Debug, Clone)]
pub struct CombatActionResult {
    pub damage: u32,
    pub is_critical: bool,
    pub is_player_turn: bool,
    pub target_defeated: bool,
    pub source: String,
    pub skill_name: Option<String>,
}

impl CombatActionResult {
    pub fn invalid() -> Self {
        Self {
            damage: 0,
            is_critical: false,
            is_player_turn: false,
            target_defeated: false,
            source: "invalid".to_string(),
            skill_name: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.source != "invalid"
    }
}

pub struct CombatIntegrationSystem;

impl CombatIntegrationSystem {
    pub fn process_combat_result(
        result: &CombatActionResult,
        effects: &mut EffectsManager,
        animations: &mut AnimationManager,
        audio: &mut AudioPlaybackSystem,
        event_bus: &mut EventBus,
        target_x: f32,
        target_y: f32,
    ) {
        if !result.is_valid() {
            return;
        }

        // Spawn damage number
        effects.spawn_damage_number(result.damage, target_x, target_y, result.is_critical, false);

        // Play animation
        if result.source == "player" {
            let anim = AnimationManager::create_attack_animation("TwinBlade");
            animations.add_animation(anim);
            audio.play_sfx("sfx_attack");
        } else {
            let anim = AnimationManager::create_hit_animation();
            animations.add_animation(anim);
            audio.play_sfx("sfx_hit");
        }

        // Screen shake on critical
        if result.is_critical {
            effects.shake_screen(8.0, 0.3);
        }

        // Emit event
        if result.target_defeated {
            event_bus.emit(GameEvent::EnemyDefeated {
                enemy_id: result.source.clone(),
                exp_reward: 50,
            });
        }

        if result.damage > 0 {
            event_bus.emit(GameEvent::PlayerDamaged {
                amount: result.damage,
                source: result.source.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Class;

    #[test]
    fn test_combat_encounter_creation() {
        let player = Stats::new(Class::TwinBlade);
        let enemy = Stats::new(Class::HeavyBlade);
        let encounter = CombatEncounter::new(player, enemy);
        assert!(encounter.is_active);
        assert!(encounter.is_player_turn);
        assert_eq!(encounter.turn_count, 0);
    }

    #[test]
    fn test_player_attack() {
        let player = Stats::new(Class::HeavyBlade);
        let enemy = Stats::new(Class::Wavemaster);
        let mut encounter = CombatEncounter::new(player, enemy);
        let initial_hp = encounter.enemy_stats.hp;

        let result = encounter.player_attack(None);
        assert!(result.is_valid());
        assert!(result.damage > 0);
        assert!(encounter.enemy_stats.hp < initial_hp);
        assert_eq!(encounter.turn_count, 1);
    }

    #[test]
    fn test_enemy_attack() {
        let player = Stats::new(Class::HeavyBlade);
        let enemy = Stats::new(Class::TwinBlade);
        let mut encounter = CombatEncounter::new(player, enemy);
        encounter.is_player_turn = false;

        let initial_hp = encounter.player_stats.hp;
        let result = encounter.enemy_attack();
        assert!(result.is_valid());
        assert!(encounter.player_stats.hp < initial_hp || result.damage == 0);
    }

    #[test]
    fn test_full_combat_flow() {
        let player = Stats::new(Class::HeavyBlade);
        let mut enemy = Stats::new(Class::Wavemaster);
        enemy.hp = 10; // Low HP for quick test
        let mut encounter = CombatEncounter::new(player, enemy);

        // Player attacks until enemy is defeated
        let mut player_won = false;
        while encounter.is_active {
            if encounter.is_player_turn {
                let result = encounter.player_attack(None);
                if result.target_defeated {
                    player_won = true;
                }
            }
        }

        assert!(player_won);
        assert!(encounter.is_over());
        assert!(encounter.player_won());
        assert!(!encounter.combat_log.is_empty());
    }

    #[test]
    fn test_combat_log() {
        let player = Stats::new(Class::TwinBlade);
        let mut enemy = Stats::new(Class::Wavemaster);
        enemy.hp = 5;
        let mut encounter = CombatEncounter::new(player, enemy);

        encounter.player_attack(None);
        assert!(!encounter.get_log().is_empty());
        assert!(encounter.get_log()[0].contains("damage"));
    }

    #[test]
    fn test_invalid_action() {
        let result = CombatActionResult::invalid();
        assert!(!result.is_valid());
    }

    #[test]
    fn test_data_drain_in_combat() {
        let player = Stats::new(Class::Wavemaster);
        let enemy = Stats::new(Class::HeavyBlade);
        let mut encounter = CombatEncounter::new(player, enemy);
        let mut data_drain = DataDrain::new();
        data_drain.charge(100.0); // Fully charged

        let (action, drain) = encounter.execute_data_drain(&mut data_drain);
        assert!(action.is_valid());
        assert!(drain.exp_bonus > 0 || drain.success);
    }

    #[test]
    fn test_process_combat_result() {
        let mut effects = EffectsManager::new();
        let mut animations = AnimationManager::new();
        let mut audio = AudioPlaybackSystem::new();
        let mut event_bus = EventBus::new(100);

        let result = CombatActionResult {
            damage: 50,
            is_critical: true,
            is_player_turn: true,
            target_defeated: false,
            source: "player".to_string(),
            skill_name: None,
        };

        CombatIntegrationSystem::process_combat_result(
            &result,
            &mut effects,
            &mut animations,
            &mut audio,
            &mut event_bus,
            100.0,
            100.0,
        );

        assert_eq!(effects.damage_count(), 1);
        assert_eq!(effects.screen_shakes.len(), 1);
    }
}
