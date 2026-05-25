use crate::core::types::GameState;

/// Stack-based finite-state machine for game-screen transitions.
///
/// The manager keeps track of the **current** and **previous** states,
/// and supports a push-down stack so the game can open sub-screens
/// (e.g. an options menu) and later pop back to the previous screen.
///
/// # Example
///
/// ```ignore
/// let mut sm = GameStateManager::new();
/// assert_eq!(sm.current(), GameState::Boot);
///
/// sm.transition_to(GameState::Title);
/// assert_eq!(sm.current(), GameState::Title);
///
/// sm.push_state(GameState::Menu);   // Exploring → Menu
/// sm.pop_state();                    // Menu → Exploring
/// ```
pub struct GameStateManager {
    current_state: GameState,
    previous_state: GameState,
    state_stack: Vec<GameState>,
}

impl GameStateManager {
    /// Create a new manager starting in the [`Boot`](GameState::Boot) state.
    pub fn new() -> Self {
        Self {
            current_state: GameState::Boot,
            previous_state: GameState::Boot,
            state_stack: Vec::new(),
        }
    }

    /// Replace the current state with a new state (non-stack).
    ///
    /// The previous state is saved and can be retrieved via
    /// [`previous()`](Self::previous).
    pub fn transition_to(&mut self, new_state: GameState) {
        self.previous_state = self.current_state;
        self.current_state = new_state;
    }

    /// Push the current state onto the stack and enter a new sub-state.
    ///
    /// Call [`pop_state()`](Self::pop_state) to return to the previous state.
    pub fn push_state(&mut self, state: GameState) {
        self.state_stack.push(self.current_state);
        self.current_state = state;
    }

    /// Pop the state stack, returning to the most recently pushed state.
    pub fn pop_state(&mut self) {
        if let Some(previous) = self.state_stack.pop() {
            self.current_state = previous;
        }
    }

    /// Current active state.
    pub fn current(&self) -> GameState {
        self.current_state
    }

    /// The state that was active before the last transition.
    pub fn previous(&self) -> GameState {
        self.previous_state
    }
}

impl Default for GameStateManager {
    fn default() -> Self {
        Self::new()
    }
}
