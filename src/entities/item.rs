use crate::components::inventory::{Item, ItemType};
use crate::components::position::Position;
use crate::components::render::{RenderLayer, Renderable};

pub struct ItemEntity;

impl ItemEntity {
    pub fn create(item: Item, x: f32, y: f32) -> (Item, Position, Renderable) {
        let position = Position::new(x, y);
        let renderable = Renderable::new(&format!("item_{}", item.id), RenderLayer::Items);
        (item, position, renderable)
    }

    pub fn create_consumable(id: &str, name: &str, quantity: u32) -> Item {
        Item {
            id: id.to_string(),
            name: name.to_string(),
            quantity,
            max_stack: 99,
            item_type: ItemType::Consumable,
        }
    }

    pub fn create_equipment(id: &str, name: &str, item_type: ItemType) -> Item {
        Item {
            id: id.to_string(),
            name: name.to_string(),
            quantity: 1,
            max_stack: 1,
            item_type,
        }
    }

    pub fn get_healing_value(item: &Item) -> u32 {
        match item.id.as_str() {
            "potion" => 50,
            "hi_potion" => 150,
            "ether" => 30,
            "hi_ether" => 80,
            _ => 0,
        }
    }

    pub fn get_shop_prices() -> Vec<(String, u64)> {
        vec![
            ("potion".to_string(), 50),
            ("hi_potion".to_string(), 150),
            ("ether".to_string(), 100),
            ("hi_ether".to_string(), 300),
            ("antidote".to_string(), 30),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_item_entity() {
        let item = ItemEntity::create_consumable("potion", "Potion", 1);
        let (_, pos, _) = ItemEntity::create(item.clone(), 50.0, 50.0);
        assert_eq!(pos.x, 50.0);
        assert_eq!(item.name, "Potion");
    }

    #[test]
    fn test_healing_values() {
        let potion = ItemEntity::create_consumable("potion", "Potion", 1);
        assert_eq!(ItemEntity::get_healing_value(&potion), 50);
    }

    #[test]
    fn test_shop_prices() {
        let prices = ItemEntity::get_shop_prices();
        assert_eq!(prices.len(), 5);
        assert!(prices.iter().any(|(id, _)| id == "potion"));
    }
}
