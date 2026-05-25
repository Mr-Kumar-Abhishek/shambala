#[derive(Debug, Clone)]
pub enum GameEvent {
    PlayerDamaged {
        amount: u32,
        source: String,
    },
    PlayerHealed {
        amount: u32,
        source: String,
    },
    EnemyDefeated {
        enemy_id: String,
        exp_reward: u64,
    },
    LevelUp {
        new_level: u32,
    },
    ItemObtained {
        item_id: String,
        quantity: u32,
    },
    DataDrainExecuted {
        success: bool,
        target: String,
    },
    AreaTransition {
        from: String,
        to: String,
    },
    PartyMemberJoined {
        name: String,
    },
    PartyMemberLeft {
        name: String,
    },
    DialogueStarted {
        npc_id: String,
    },
    DialogueEnded {
        npc_id: String,
    },
    QuestUpdated {
        quest_id: String,
        status: QuestStatus,
    },
    GameSaved,
    ConnectionLost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestStatus {
    Started,
    InProgress,
    Completed,
    Failed,
}

pub struct EventBus {
    pub events: Vec<GameEvent>,
    pub max_events: usize,
}

impl EventBus {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
        }
    }

    pub fn emit(&mut self, event: GameEvent) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<GameEvent> {
        self.events.drain(..).collect()
    }

    pub fn has_event(&self, event_type: &str) -> bool {
        self.events.iter().any(|e| match e {
            GameEvent::PlayerDamaged { .. } => event_type == "PlayerDamaged",
            GameEvent::EnemyDefeated { .. } => event_type == "EnemyDefeated",
            GameEvent::LevelUp { .. } => event_type == "LevelUp",
            GameEvent::DataDrainExecuted { .. } => event_type == "DataDrainExecuted",
            GameEvent::AreaTransition { .. } => event_type == "AreaTransition",
            _ => false,
        })
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_event() {
        let mut bus = EventBus::new(100);
        bus.emit(GameEvent::PlayerDamaged {
            amount: 10,
            source: "Goblin".to_string(),
        });
        assert_eq!(bus.events.len(), 1);
    }

    #[test]
    fn test_drain_events() {
        let mut bus = EventBus::new(100);
        bus.emit(GameEvent::LevelUp { new_level: 2 });
        let drained = bus.drain();
        assert_eq!(drained.len(), 1);
        assert!(bus.events.is_empty());
    }

    #[test]
    fn test_max_events() {
        let mut bus = EventBus::new(2);
        bus.emit(GameEvent::LevelUp { new_level: 2 });
        bus.emit(GameEvent::LevelUp { new_level: 3 });
        bus.emit(GameEvent::LevelUp { new_level: 4 });
        assert_eq!(bus.events.len(), 2);
    }

    #[test]
    fn test_has_event() {
        let mut bus = EventBus::new(100);
        bus.emit(GameEvent::EnemyDefeated {
            enemy_id: "goblin_1".to_string(),
            exp_reward: 15,
        });
        assert!(bus.has_event("EnemyDefeated"));
        assert!(!bus.has_event("LevelUp"));
    }
}
