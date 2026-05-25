#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Attack,
    Skill1,
    Skill2,
    Skill3,
    Skill4,
    DataDrain,
    Interact,
    Menu,
    Cancel,
    Confirm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Pressed,
    Held,
    Released,
    None,
}

pub struct InputSystem;

impl InputSystem {
    pub fn is_movement_action(action: InputAction) -> bool {
        matches!(
            action,
            InputAction::MoveUp
                | InputAction::MoveDown
                | InputAction::MoveLeft
                | InputAction::MoveRight
        )
    }

    pub fn is_combat_action(action: InputAction) -> bool {
        matches!(
            action,
            InputAction::Attack
                | InputAction::Skill1
                | InputAction::Skill2
                | InputAction::Skill3
                | InputAction::Skill4
                | InputAction::DataDrain
        )
    }

    pub fn get_movement_vector(actions: &[(InputAction, InputState)]) -> (f32, f32) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        for (action, state) in actions {
            if *state == InputState::Held || *state == InputState::Pressed {
                match action {
                    InputAction::MoveUp => dy -= 1.0,
                    InputAction::MoveDown => dy += 1.0,
                    InputAction::MoveLeft => dx -= 1.0,
                    InputAction::MoveRight => dx += 1.0,
                    _ => {}
                }
            }
        }

        // Normalize diagonal movement
        let length = f32::sqrt(dx * dx + dy * dy);
        if length > 1.0 {
            dx /= length;
            dy /= length;
        }

        (dx, dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_action() {
        assert!(InputSystem::is_movement_action(InputAction::MoveUp));
        assert!(!InputSystem::is_movement_action(InputAction::Attack));
    }

    #[test]
    fn test_combat_action() {
        assert!(InputSystem::is_combat_action(InputAction::Attack));
        assert!(!InputSystem::is_combat_action(InputAction::MoveUp));
    }

    #[test]
    fn test_single_direction() {
        let actions = vec![(InputAction::MoveRight, InputState::Held)];
        let (dx, dy) = InputSystem::get_movement_vector(&actions);
        assert_eq!(dx, 1.0);
        assert_eq!(dy, 0.0);
    }

    #[test]
    fn test_diagonal_normalization() {
        let actions = vec![
            (InputAction::MoveRight, InputState::Held),
            (InputAction::MoveDown, InputState::Held),
        ];
        let (dx, dy) = InputSystem::get_movement_vector(&actions);
        assert!((dx - 0.707).abs() < 0.01);
        assert!((dy - 0.707).abs() < 0.01);
    }

    #[test]
    fn test_no_input() {
        let actions = vec![];
        let (dx, dy) = InputSystem::get_movement_vector(&actions);
        assert_eq!(dx, 0.0);
        assert_eq!(dy, 0.0);
    }
}
