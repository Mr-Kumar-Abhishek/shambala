use crate::game::event::QuestStatus;

#[derive(Debug, Clone)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: QuestStatus,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
    pub dialogue_on_start: Vec<DialogueNode>,
    pub dialogue_on_complete: Vec<DialogueNode>,
}

#[derive(Debug, Clone)]
pub struct QuestObjective {
    pub id: String,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub target_id: String,
    pub required_amount: u32,
    pub current_amount: u32,
    pub is_complete: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveType {
    Kill,
    Collect,
    Talk,
    Explore,
    UseChaosGate,
    DefeatBoss,
}

#[derive(Debug, Clone)]
pub struct QuestRewards {
    pub experience: u64,
    pub gold: u64,
    pub items: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DialogueNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
    pub is_end: bool,
    pub event_to_trigger: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DialogueChoice {
    pub text: String,
    pub next_node_id: String,
    pub required_quest_status: Option<(String, QuestStatus)>,
}

pub struct QuestManager {
    pub quests: Vec<Quest>,
    pub active_quests: Vec<String>,
    pub completed_quests: Vec<String>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            quests: Vec::new(),
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
        }
    }

    pub fn register_quest(&mut self, quest: Quest) {
        self.quests.push(quest);
    }

    pub fn start_quest(&mut self, quest_id: &str) -> Result<(), &str> {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.id == quest_id) {
            if quest.status == QuestStatus::InProgress {
                return Err("Quest already active");
            }
            quest.status = QuestStatus::InProgress;
            self.active_quests.push(quest_id.to_string());
            Ok(())
        } else {
            Err("Quest not found")
        }
    }

    pub fn update_objective(&mut self, quest_id: &str, objective_id: &str, amount: u32) {
        if let Some(quest) = self.quests.iter_mut().find(|q| q.id == quest_id) {
            if let Some(objective) = quest.objectives.iter_mut().find(|o| o.id == objective_id) {
                objective.current_amount = (objective.current_amount + amount).min(objective.required_amount);
                if objective.current_amount >= objective.required_amount {
                    objective.is_complete = true;
                }
            }
            // Check if all objectives are complete
            if quest.objectives.iter().all(|o| o.is_complete) {
                quest.status = QuestStatus::Completed;
                self.active_quests.retain(|id| id != quest_id);
                self.completed_quests.push(quest_id.to_string());
            }
        }
    }

    pub fn get_quest(&self, quest_id: &str) -> Option<&Quest> {
        self.quests.iter().find(|q| q.id == quest_id)
    }

    pub fn get_active_quests(&self) -> Vec<&Quest> {
        self.quests.iter()
            .filter(|q| self.active_quests.contains(&q.id))
            .collect()
    }

    pub fn get_dialogue_for_quest(&self, quest_id: &str, is_completion: bool) -> Option<&[DialogueNode]> {
        if let Some(quest) = self.get_quest(quest_id) {
            if is_completion {
                Some(&quest.dialogue_on_complete)
            } else {
                Some(&quest.dialogue_on_start)
            }
        } else {
            None
        }
    }

    pub fn create_tutorial_quest() -> Quest {
        Quest {
            id: "tutorial_01".to_string(),
            name: "Welcome to Shambala".to_string(),
            description: "Learn the basics of navigating the world of Shambala.".to_string(),
            status: QuestStatus::Started,
            objectives: vec![
                QuestObjective {
                    id: "talk_to_helba".to_string(),
                    description: "Talk to Helba in Mac Anu".to_string(),
                    objective_type: ObjectiveType::Talk,
                    target_id: "npc_helba".to_string(),
                    required_amount: 1,
                    current_amount: 0,
                    is_complete: false,
                },
                QuestObjective {
                    id: "use_chaos_gate".to_string(),
                    description: "Use the Chaos Gate to travel to a field area".to_string(),
                    objective_type: ObjectiveType::UseChaosGate,
                    target_id: "chaos_gate".to_string(),
                    required_amount: 1,
                    current_amount: 0,
                    is_complete: false,
                },
                QuestObjective {
                    id: "defeat_goblins".to_string(),
                    description: "Defeat 3 Goblins in the field".to_string(),
                    objective_type: ObjectiveType::Kill,
                    target_id: "Goblin".to_string(),
                    required_amount: 3,
                    current_amount: 0,
                    is_complete: false,
                },
            ],
            rewards: QuestRewards {
                experience: 100,
                gold: 50,
                items: vec!["potion".to_string(), "ether".to_string()],
            },
            dialogue_on_start: vec![
                DialogueNode {
                    id: "start_1".to_string(),
                    speaker: "Helba".to_string(),
                    text: "Welcome to Shambala, traveler. This world is not what it seems.".to_string(),
                    choices: vec![
                        DialogueChoice {
                            text: "What do you mean?".to_string(),
                            next_node_id: "start_2".to_string(),
                            required_quest_status: None,
                        },
                    ],
                    is_end: false,
                    event_to_trigger: None,
                },
                DialogueNode {
                    id: "start_2".to_string(),
                    speaker: "Helba".to_string(),
                    text: "Many players have found themselves unable to log out. We need your help to uncover the truth.".to_string(),
                    choices: vec![
                        DialogueChoice {
                            text: "I'll help.".to_string(),
                            next_node_id: "start_3".to_string(),
                            required_quest_status: None,
                        },
                    ],
                    is_end: false,
                    event_to_trigger: None,
                },
                DialogueNode {
                    id: "start_3".to_string(),
                    speaker: "Helba".to_string(),
                    text: "Good. First, learn to use the Chaos Gate to travel between areas. Then defeat some Goblins to test your combat skills.".to_string(),
                    choices: vec![],
                    is_end: true,
                    event_to_trigger: Some("quest_started".to_string()),
                },
            ],
            dialogue_on_complete: vec![
                DialogueNode {
                    id: "complete_1".to_string(),
                    speaker: "Helba".to_string(),
                    text: "Excellent work! You've proven yourself capable. There's much more to discover.".to_string(),
                    choices: vec![],
                    is_end: true,
                    event_to_trigger: Some("quest_complete".to_string()),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_manager_creation() {
        let manager = QuestManager::new();
        assert!(manager.quests.is_empty());
        assert!(manager.active_quests.is_empty());
    }

    #[test]
    fn test_register_quest() {
        let mut manager = QuestManager::new();
        let quest = QuestManager::create_tutorial_quest();
        manager.register_quest(quest);
        assert_eq!(manager.quests.len(), 1);
    }

    #[test]
    fn test_start_quest() {
        let mut manager = QuestManager::new();
        let quest = QuestManager::create_tutorial_quest();
        let quest_id = quest.id.clone();
        manager.register_quest(quest);
        assert!(manager.start_quest(&quest_id).is_ok());
        assert_eq!(manager.active_quests.len(), 1);
    }

    #[test]
    fn test_start_quest_not_found() {
        let mut manager = QuestManager::new();
        assert!(manager.start_quest("nonexistent").is_err());
    }

    #[test]
    fn test_update_objective() {
        let mut manager = QuestManager::new();
        let quest = QuestManager::create_tutorial_quest();
        let quest_id = quest.id.clone();
        manager.register_quest(quest);
        manager.start_quest(&quest_id).unwrap();
        
        manager.update_objective(&quest_id, "talk_to_helba", 1);
        let quest = manager.get_quest(&quest_id).unwrap();
        assert!(quest.objectives[0].is_complete);
    }

    #[test]
    fn test_quest_completion() {
        let mut manager = QuestManager::new();
        let quest = QuestManager::create_tutorial_quest();
        let quest_id = quest.id.clone();
        manager.register_quest(quest);
        manager.start_quest(&quest_id).unwrap();
        
        // Complete all objectives
        manager.update_objective(&quest_id, "talk_to_helba", 1);
        manager.update_objective(&quest_id, "use_chaos_gate", 1);
        manager.update_objective(&quest_id, "defeat_goblins", 3);
        
        let quest = manager.get_quest(&quest_id).unwrap();
        assert_eq!(quest.status, QuestStatus::Completed);
        assert!(manager.active_quests.is_empty());
        assert_eq!(manager.completed_quests.len(), 1);
    }

    #[test]
    fn test_get_active_quests() {
        let mut manager = QuestManager::new();
        let quest = QuestManager::create_tutorial_quest();
        let quest_id = quest.id.clone();
        manager.register_quest(quest);
        manager.start_quest(&quest_id).unwrap();
        
        let active = manager.get_active_quests();
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn test_dialogue_nodes() {
        let quest = QuestManager::create_tutorial_quest();
        assert_eq!(quest.dialogue_on_start.len(), 3);
        assert_eq!(quest.dialogue_on_complete.len(), 1);
        assert!(quest.dialogue_on_start[0].choices[0].text.contains("mean"));
    }

    #[test]
    fn test_tutorial_quest_structure() {
        let quest = QuestManager::create_tutorial_quest();
        assert_eq!(quest.id, "tutorial_01");
        assert_eq!(quest.objectives.len(), 3);
        assert_eq!(quest.rewards.experience, 100);
        assert_eq!(quest.rewards.items.len(), 2);
    }
}
