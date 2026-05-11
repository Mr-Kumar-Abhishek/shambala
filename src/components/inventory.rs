#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub max_stack: u32,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Consumable,
    Weapon,
    Armor,
    Accessory,
    KeyItem,
    Material,
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub max_items: usize,
    pub gold: u64,
}

impl Inventory {
    pub fn new(max_items: usize) -> Self {
        Self {
            items: Vec::new(),
            max_items,
            gold: 0,
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), &str> {
        if self.items.len() >= self.max_items {
            Err("Inventory is full")
        } else {
            self.items.push(item);
            Ok(())
        }
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn add_gold(&mut self, amount: u64) {
        self.gold = self.gold.saturating_add(amount);
    }

    pub fn spend_gold(&mut self, amount: u64) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut inv = Inventory::new(10);
        let item = Item {
            id: "potion".to_string(),
            name: "Potion".to_string(),
            quantity: 1,
            max_stack: 99,
            item_type: ItemType::Consumable,
        };
        assert!(inv.add_item(item).is_ok());
        assert_eq!(inv.items.len(), 1);
    }

    #[test]
    fn test_gold() {
        let mut inv = Inventory::new(10);
        inv.add_gold(100);
        assert_eq!(inv.gold, 100);
        assert!(inv.spend_gold(50));
        assert_eq!(inv.gold, 50);
        assert!(!inv.spend_gold(100));
    }
}