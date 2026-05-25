use crate::components::party::Party;
use crate::components::stats::Stats;

pub struct PartySystem;

impl PartySystem {
    pub fn distribute_experience(party: &Party, total_exp: u64) -> Vec<u64> {
        let member_count = party.members.len() as u64;
        if member_count == 0 {
            return Vec::new();
        }

        let exp_per_member = total_exp / member_count;
        party
            .members
            .iter()
            .map(|m| {
                let bond_bonus = 1.0 + (m.bond_level as f64 * 0.1);
                (exp_per_member as f64 * bond_bonus) as u64
            })
            .collect()
    }

    pub fn get_party_average_level(stats: &[&Stats]) -> u32 {
        if stats.is_empty() {
            return 0;
        }
        let total: u32 = stats.iter().map(|s| s.level).sum();
        total / stats.len() as u32
    }

    pub fn can_form_party(member_count: usize, max_size: usize) -> bool {
        member_count >= 1 && member_count <= max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Class;

    #[test]
    fn test_distribute_experience() {
        let mut party = Party::new(3);
        party.add_member("Kite").unwrap();
        party.add_member("BlackRose").unwrap();

        let exp = PartySystem::distribute_experience(&party, 100);
        assert_eq!(exp.len(), 2);
        assert!(exp[0] >= 50); // With bond bonus
    }

    #[test]
    fn test_empty_party_exp() {
        let party = Party::new(3);
        let exp = PartySystem::distribute_experience(&party, 100);
        assert!(exp.is_empty());
    }

    #[test]
    fn test_average_level() {
        let stats1 = Stats::new(Class::TwinBlade);
        let stats2 = Stats::new(Class::HeavyBlade);
        let avg = PartySystem::get_party_average_level(&[&stats1, &stats2]);
        assert_eq!(avg, 1);
    }

    #[test]
    fn test_can_form_party() {
        assert!(PartySystem::can_form_party(1, 3));
        assert!(PartySystem::can_form_party(3, 3));
        assert!(!PartySystem::can_form_party(0, 3));
        assert!(!PartySystem::can_form_party(4, 3));
    }
}
