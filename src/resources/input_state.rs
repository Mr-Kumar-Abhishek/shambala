use crate::systems::input::{InputAction, InputState};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InputStateResource {
    pub actions: HashMap<InputAction, InputState>,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_pressed: bool,
}

impl InputStateResource {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_pressed: false,
        }
    }

    pub fn is_action_pressed(&self, action: InputAction) -> bool {
        self.actions.get(&action) == Some(&InputState::Pressed)
    }

    pub fn is_action_held(&self, action: InputAction) -> bool {
        self.actions.get(&action) == Some(&InputState::Held)
    }

    pub fn set_action(&mut self, action: InputAction, state: InputState) {
        self.actions.insert(action, state);
    }

    pub fn clear_frame(&mut self) {
        self.actions.retain(|_, state| *state == InputState::Held);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_input_state() {
        let state = InputStateResource::new();
        assert!(!state.is_action_pressed(InputAction::Attack));
    }

    #[test]
    fn test_set_and_check_action() {
        let mut state = InputStateResource::new();
        state.set_action(InputAction::Attack, InputState::Pressed);
        assert!(state.is_action_pressed(InputAction::Attack));
    }

    #[test]
    fn test_clear_frame() {
        let mut state = InputStateResource::new();
        state.set_action(InputAction::Attack, InputState::Pressed);
        state.set_action(InputAction::MoveUp, InputState::Held);
        state.clear_frame();
        assert!(!state.is_action_pressed(InputAction::Attack));
        assert!(state.is_action_held(InputAction::MoveUp));
    }
}
