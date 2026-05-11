use crate::components::position::Position;
use crate::components::render::{Renderable, RenderLayer};
use crate::components::inventory::Item;

#[derive(Debug, Clone)]
pub struct NPC {
    pub id: String,
    pub name: String,
    pub dialogue_id: String,
    pub npc_type: NPCType,
    pub shop_items: Vec<Item>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPCType {
    Shopkeeper,
    QuestGiver,
    StoryCharacter,
    Informant,
    Healer,
}

pub struct NPCEntity;

impl NPCEntity {
    pub fn create(npc_type: NPCType, name: &str, x: f32, y: f32) -> (NPC, Position, Renderable) {
        let npc = NPC {
            id: format!("npc_{}", name.to_lowercase()),
            name: name.to_string(),
            dialogue_id: format!("dialogue_{}", name.to_lowercase()),
            npc_type,
            shop_items: Vec::new(),
        };
        let position = Position::new(x, y);
        let renderable = Renderable::new(
            &format!("npc_{:?}", npc_type),
            RenderLayer::Characters,
        );

        (npc, position, renderable)
    }

    pub fn create_shopkeeper(name: &str, x: f32, y: f32) -> (NPC, Position, Renderable) {
        let (mut npc, pos, render) = Self::create(NPCType::Shopkeeper, name, x, y);
        npc.shop_items = vec![
            Item { id: "potion".to_string(), name: "Potion".to_string(), quantity: 99, max_stack: 99, item_type: crate::components::inventory::ItemType::Consumable },
            Item { id: "ether".to_string(), name: "Ether".to_string(), quantity: 99, max_stack: 99, item_type: crate::components::inventory::ItemType::Consumable },
            Item { id: "antidote".to_string(), name: "Antidote".to_string(), quantity: 99, max_stack: 99, item_type: crate::components::inventory::ItemType::Consumable },
        ];
        (npc, pos, render)
    }

    pub fn get_story_npcs() -> Vec<(&'static str, &'static str, f32, f32)> {
        vec![
            ("Helba", "Helba", 300.0, 200.0),
            ("BT", "BT", 350.0, 250.0),
            ("Crim", "Crim", 400.0, 300.0),
            ("Mimiru", "Mimiru", 450.0, 350.0),
            ("Bear", "Bear", 500.0, 400.0),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_npc() {
        let (npc, pos, _) = NPCEntity::create(NPCType::Healer, "Healer", 100.0, 100.0);
        assert_eq!(npc.name, "Healer");
        assert_eq!(npc.npc_type, NPCType::Healer);
        assert_eq!(pos.x, 100.0);
    }

    #[test]
    fn test_shopkeeper_items() {
        let (npc, _, _) = NPCEntity::create_shopkeeper("Shop", 0.0, 0.0);
        assert_eq!(npc.shop_items.len(), 3);
        assert!(npc.shop_items.iter().any(|i| i.id == "potion"));
    }

    #[test]
    fn test_story_npcs() {
        let npcs = NPCEntity::get_story_npcs();
        assert_eq!(npcs.len(), 5);
        assert!(npcs.iter().any(|(_, name, _, _)| *name == "Helba"));
    }
}
