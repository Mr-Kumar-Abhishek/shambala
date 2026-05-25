#[derive(Debug, Clone)]
pub struct PartyMember {
    pub name: String,
    pub bond_level: u32,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct Party {
    pub members: Vec<PartyMember>,
    pub max_size: usize,
}

impl Party {
    pub fn new(max_size: usize) -> Self {
        Self {
            members: Vec::new(),
            max_size,
        }
    }

    pub fn add_member(&mut self, name: &str) -> Result<(), &str> {
        if self.members.len() >= self.max_size {
            Err("Party is full")
        } else {
            self.members.push(PartyMember {
                name: name.to_string(),
                bond_level: 1,
                is_active: true,
            });
            Ok(())
        }
    }

    pub fn remove_member(&mut self, index: usize) -> Option<PartyMember> {
        if index < self.members.len() {
            Some(self.members.remove(index))
        } else {
            None
        }
    }

    pub fn increase_bond(&mut self, index: usize, amount: u32) {
        if let Some(member) = self.members.get_mut(index) {
            member.bond_level = member.bond_level.saturating_add(amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_member() {
        let mut party = Party::new(3);
        assert!(party.add_member("Kite").is_ok());
        assert_eq!(party.members.len(), 1);
    }

    #[test]
    fn test_party_full() {
        let mut party = Party::new(1);
        assert!(party.add_member("Kite").is_ok());
        assert!(party.add_member("BlackRose").is_err());
    }

    #[test]
    fn test_remove_member() {
        let mut party = Party::new(3);
        party.add_member("Kite").unwrap();
        assert!(party.remove_member(0).is_some());
        assert!(party.members.is_empty());
    }

    #[test]
    fn test_increase_bond() {
        let mut party = Party::new(3);
        party.add_member("Kite").unwrap();
        party.increase_bond(0, 5);
        assert_eq!(party.members[0].bond_level, 6);
    }
}
