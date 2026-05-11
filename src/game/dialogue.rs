use crate::game::quest::{DialogueNode, DialogueChoice, QuestManager};
use crate::systems::input::InputAction;
use crate::resources::input_state::InputStateResource;

#[derive(Debug, Clone)]
pub struct NPCDialogue {
    pub npc_id: String,
    pub npc_name: String,
    pub current_node: String,
    pub nodes: Vec<DialogueNode>,
    pub is_active: bool,
    pub selected_choice: usize,
    pub typing_progress: f32,
    pub typing_speed: f32,
    pub text_fully_displayed: bool,
}

impl NPCDialogue {
    pub fn new(npc_id: &str, npc_name: &str) -> Self {
        Self {
            npc_id: npc_id.to_string(),
            npc_name: npc_name.to_string(),
            current_node: "start".to_string(),
            nodes: Vec::new(),
            is_active: false,
            selected_choice: 0,
            typing_progress: 0.0,
            typing_speed: 30.0, // characters per second
            text_fully_displayed: false,
        }
    }

    pub fn start_dialogue(&mut self, nodes: Vec<DialogueNode>) {
        self.nodes = nodes;
        self.current_node = "start".to_string();
        self.is_active = true;
        self.selected_choice = 0;
        self.typing_progress = 0.0;
        self.text_fully_displayed = false;
    }

    pub fn end_dialogue(&mut self) {
        self.is_active = false;
        self.nodes.clear();
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_active {
            return;
        }

        // Advance typing animation
        if let Some(node) = self.get_current_node() {
            let text_len = node.text.len() as f32;
            self.typing_progress = (self.typing_progress + dt * self.typing_speed).min(text_len);
            self.text_fully_displayed = self.typing_progress >= text_len;
        }
    }

    pub fn handle_input(&mut self, input: &InputStateResource, quest_manager: &mut QuestManager) -> Option<String> {
        if !self.is_active || !self.text_fully_displayed {
            return None;
        }

        // Clone node data to avoid borrow conflicts with self mutations
        let node_data = self.get_current_node().map(|node| {
            (node.is_end, node.event_to_trigger.clone(), node.choices.clone())
        });

        if let Some((is_end, event_to_trigger, choices)) = node_data {
            if is_end {
                if let Some(event) = event_to_trigger {
                    self.end_dialogue();
                    return Some(event);
                }
                self.end_dialogue();
                return None;
            }

            // Filter available choices based on quest status
            let available_choices: Vec<DialogueChoice> = choices.into_iter()
                .filter(|choice| {
                    if let Some((ref quest_id, required_status)) = choice.required_quest_status {
                        if let Some(quest) = quest_manager.get_quest(quest_id) {
                            return quest.status == required_status;
                        }
                        return false;
                    }
                    true
                })
                .collect();

            if available_choices.is_empty() {
                self.end_dialogue();
                return None;
            }

            // Navigate choices
            if input.is_action_pressed(InputAction::MoveUp) {
                self.selected_choice = if self.selected_choice == 0 {
                    available_choices.len() - 1
                } else {
                    self.selected_choice - 1
                };
            }

            if input.is_action_pressed(InputAction::MoveDown) {
                self.selected_choice = (self.selected_choice + 1) % available_choices.len();
            }

            // Confirm choice
            if input.is_action_pressed(InputAction::Confirm) {
                if self.selected_choice < available_choices.len() {
                    let choice = &available_choices[self.selected_choice];
                    self.current_node = choice.next_node_id.clone();
                    self.selected_choice = 0;
                    self.typing_progress = 0.0;
                    self.text_fully_displayed = false;
                }
            }

            // Cancel dialogue
            if input.is_action_pressed(InputAction::Cancel) {
                self.end_dialogue();
            }
        }

        None
    }

    pub fn get_current_node(&self) -> Option<&DialogueNode> {
        self.nodes.iter().find(|n| n.id == self.current_node)
    }

    pub fn get_displayed_text(&self) -> String {
        if let Some(node) = self.get_current_node() {
            let chars_to_show = self.typing_progress as usize;
            node.text.chars().take(chars_to_show).collect()
        } else {
            String::new()
        }
    }

    pub fn get_available_choices(&self, quest_manager: &QuestManager) -> Vec<DialogueChoice> {
        if let Some(node) = self.get_current_node() {
            node.choices.iter()
                .filter(|choice| {
                    if let Some((ref quest_id, required_status)) = choice.required_quest_status {
                        if let Some(quest) = quest_manager.get_quest(quest_id) {
                            return quest.status == required_status;
                        }
                        return false;
                    }
                    true
                })
                .cloned()
                .collect()
        } else {
            vec![]
        }
    }

    pub fn can_interact(&self, player_pos: &crate::components::position::Position, npc_pos: &crate::components::position::Position) -> bool {
        let distance = crate::systems::physics::PhysicsSystem::distance_between(player_pos, npc_pos);
        distance <= 64.0 // Interaction range
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::quest::Quest;
    use crate::systems::input::InputState;

    #[test]
    fn test_dialogue_creation() {
        let dialogue = NPCDialogue::new("npc_helba", "Helba");
        assert_eq!(dialogue.npc_name, "Helba");
        assert!(!dialogue.is_active);
    }

    #[test]
    fn test_start_dialogue() {
        let mut dialogue = NPCDialogue::new("npc_test", "Test");
        dialogue.start_dialogue(vec![
            DialogueNode {
                id: "start".to_string(),
                speaker: "Test".to_string(),
                text: "Hello!".to_string(),
                choices: vec![],
                is_end: true,
                event_to_trigger: None,
            },
        ]);
        assert!(dialogue.is_active);
    }

    #[test]
    fn test_dialogue_typing() {
        let mut dialogue = NPCDialogue::new("npc_test", "Test");
        dialogue.start_dialogue(vec![
            DialogueNode {
                id: "start".to_string(),
                speaker: "Test".to_string(),
                text: "Hello traveler!".to_string(),
                choices: vec![],
                is_end: true,
                event_to_trigger: None,
            },
        ]);
        
        dialogue.update(0.5);
        let displayed = dialogue.get_displayed_text();
        assert!(!displayed.is_empty());
        assert!(displayed.len() <= "Hello traveler!".len());
    }

    #[test]
    fn test_choice_navigation() {
        let mut dialogue = NPCDialogue::new("npc_test", "Test");
        dialogue.start_dialogue(vec![
            DialogueNode {
                id: "start".to_string(),
                speaker: "Test".to_string(),
                text: "Choose:".to_string(),
                choices: vec![
                    DialogueChoice {
                        text: "Option A".to_string(),
                        next_node_id: "a".to_string(),
                        required_quest_status: None,
                    },
                    DialogueChoice {
                        text: "Option B".to_string(),
                        next_node_id: "b".to_string(),
                        required_quest_status: None,
                    },
                ],
                is_end: false,
                event_to_trigger: None,
            },
            DialogueNode {
                id: "a".to_string(), speaker: "Test".to_string(),
                text: "You chose A".to_string(),
                choices: vec![], is_end: true, event_to_trigger: None,
            },
        ]);
        
        // Fast-forward typing
        dialogue.typing_progress = 999.0;
        dialogue.text_fully_displayed = true;
        
        let mut input = InputStateResource::new();
        input.set_action(InputAction::MoveDown, InputState::Pressed);
        let mut quest_manager = QuestManager::new();
        dialogue.handle_input(&input, &mut quest_manager);
        // Should have moved to choice 1
    }

    #[test]
    fn test_interaction_range() {
        let npc_pos = crate::components::position::Position::new(100.0, 100.0);
        let player_pos = crate::components::position::Position::new(110.0, 105.0);
        
        let dialogue = NPCDialogue::new("npc_test", "Test");
        assert!(dialogue.can_interact(&player_pos, &npc_pos));
        
        let far_pos = crate::components::position::Position::new(500.0, 500.0);
        assert!(!dialogue.can_interact(&far_pos, &npc_pos));
    }

    #[test]
    fn test_end_dialogue() {
        let mut dialogue = NPCDialogue::new("npc_test", "Test");
        dialogue.start_dialogue(vec![
            DialogueNode {
                id: "start".to_string(), speaker: "Test".to_string(),
                text: "Bye!".to_string(), choices: vec![], is_end: true,
                event_to_trigger: None,
            },
        ]);
        dialogue.typing_progress = 999.0;
        dialogue.text_fully_displayed = true;
        
        let mut input = InputStateResource::new();
        input.set_action(InputAction::Confirm, InputState::Pressed);
        let mut quest_manager = QuestManager::new();
        dialogue.handle_input(&input, &mut quest_manager);
        assert!(!dialogue.is_active);
    }
}