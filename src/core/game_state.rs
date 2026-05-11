use crate::core::types::GameState;

pub struct GameStateManager {
    current_state: GameState,
    previous_state: GameState,
    state_stack: Vec<GameState>,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            current_state: GameState::Boot,
            previous_state: GameState::Boot,
            state_stack: Vec::new(),
        }
    }

    pub fn transition_to(&mut self, new_state: GameState) {
        self.previous_state = self.current_state;
        self.current_state = new_state;
    }

    pub fn push_state(&mut self, state: GameState) {
        self.state_stack.push(self.current_state);
        self.current_state = state;
    }

    pub fn pop_state(&mut self) {
        if let Some(previous) = self.state_stack.pop() {
            self.current_state = previous;
        }
    }

    pub fn current(&self) -> GameState {
        self.current_state
    }

    pub fn previous(&self) -> GameState {
        self.previous_state
    }
}