use crate::components::data_drain::DataDrain;

pub struct DataDrainSystem;

#[derive(Debug, Clone)]
pub struct DataDrainResult {
    pub success: bool,
    pub exp_bonus: u64,
    pub item_dropped: Option<String>,
    pub data_fragment: Option<String>,
}

impl DataDrainSystem {
    pub fn execute_drain(data_drain: &mut DataDrain, target_hp_percentage: f32) -> DataDrainResult {
        if !data_drain.activate() {
            return DataDrainResult {
                success: false,
                exp_bonus: 0,
                item_dropped: None,
                data_fragment: None,
            };
        }

        // Success chance based on target HP
        let success_chance = 1.0 - (target_hp_percentage * 0.5);
        let success = rand::random::<f32>() < success_chance;

        if success {
            DataDrainResult {
                success: true,
                exp_bonus: 50,
                item_dropped: Some("data_fragment".to_string()),
                data_fragment: Some(format!("fragment_{}", rand::random::<u16>())),
            }
        } else {
            DataDrainResult {
                success: false,
                exp_bonus: 10,
                item_dropped: None,
                data_fragment: None,
            }
        }
    }

    pub fn charge_from_combat(data_drain: &mut DataDrain, damage_dealt: u32) {
        let charge_amount = (damage_dealt as f32) * 0.5;
        data_drain.charge(charge_amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charge_from_combat() {
        let mut dd = DataDrain::new();
        DataDrainSystem::charge_from_combat(&mut dd, 50);
        assert_eq!(dd.charge_level, 25.0);
    }

    #[test]
    fn test_execute_drain_not_ready() {
        let mut dd = DataDrain::new();
        let result = DataDrainSystem::execute_drain(&mut dd, 0.5);
        assert!(!result.success);
    }

    #[test]
    fn test_execute_drain_ready() {
        let mut dd = DataDrain::new();
        dd.charge(100.0);
        let result = DataDrainSystem::execute_drain(&mut dd, 0.1);
        // Should succeed with high probability at 10% HP
        assert!(result.exp_bonus > 0);
    }
}
