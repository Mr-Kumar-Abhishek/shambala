use crate::core::types::GameState;
use crate::core::game_state::GameStateManager;
use crate::systems::input::InputAction;
use crate::resources::input_state::InputStateResource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuOption {
    NewGame,
    Continue,
    Options,
    Quit,
}

impl MenuOption {
    pub fn all() -> &'static [MenuOption] {
        &[MenuOption::NewGame, MenuOption::Continue, MenuOption::Options, MenuOption::Quit]
    }

    pub fn label(&self) -> &'static str {
        match self {
            MenuOption::NewGame => "New Game",
            MenuOption::Continue => "Continue",
            MenuOption::Options => "Options",
            MenuOption::Quit => "Quit",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MenuOption::NewGame => "Begin a new adventure in the world of Shambala",
            MenuOption::Continue => "Continue your previous journey",
            MenuOption::Options => "Adjust game settings",
            MenuOption::Quit => "Exit the game",
        }
    }
}

pub struct TitleScreen {
    pub selected_index: usize,
    pub options: Vec<MenuOption>,
    pub visible: bool,
    pub transition_timer: f32,
    pub title_animation_progress: f32,
}

impl TitleScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            options: MenuOption::all().to_vec(),
            visible: true,
            transition_timer: 0.0,
            title_animation_progress: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputStateResource) -> Option<MenuOption> {
        if !self.visible {
            return None;
        }

        // Animate title
        self.title_animation_progress = (self.title_animation_progress + dt * 0.5).min(1.0);

        // Navigate menu
        if input.is_action_pressed(InputAction::MoveUp) {
            self.selected_index = if self.selected_index == 0 {
                self.options.len() - 1
            } else {
                self.selected_index - 1
            };
        }

        if input.is_action_pressed(InputAction::MoveDown) {
            self.selected_index = (self.selected_index + 1) % self.options.len();
        }

        // Select option
        if input.is_action_pressed(InputAction::Confirm) {
            let selected = self.options[self.selected_index];
            self.visible = false;
            return Some(selected);
        }

        None
    }

    pub fn handle_selection(option: MenuOption, state_manager: &mut GameStateManager) {
        match option {
            MenuOption::NewGame => {
                state_manager.transition_to(GameState::CharacterSelect);
            }
            MenuOption::Continue => {
                state_manager.transition_to(GameState::Loading);
            }
            MenuOption::Options => {
                state_manager.push_state(GameState::Menu);
            }
            MenuOption::Quit => {
                // Signal quit - handled by the game loop
            }
        }
    }

    pub fn reset(&mut self) {
        self.selected_index = 0;
        self.visible = true;
        self.transition_timer = 0.0;
        self.title_animation_progress = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::input::InputState;

    #[test]
    fn test_title_screen_creation() {
        let title = TitleScreen::new();
        assert_eq!(title.selected_index, 0);
        assert_eq!(title.options.len(), 4);
        assert!(title.visible);
    }

    #[test]
    fn test_menu_option_labels() {
        assert_eq!(MenuOption::NewGame.label(), "New Game");
        assert_eq!(MenuOption::Quit.label(), "Quit");
    }

    #[test]
    fn test_menu_navigation_up() {
        let mut title = TitleScreen::new();
        let mut input = InputStateResource::new();
        input.set_action(InputAction::MoveUp, InputState::Pressed);
        title.update(0.016, &input);
        assert_eq!(title.selected_index, 3); // Wraps to last
    }

    #[test]
    fn test_menu_navigation_down() {
        let mut title = TitleScreen::new();
        let mut input = InputStateResource::new();
        input.set_action(InputAction::MoveDown, InputState::Pressed);
        title.update(0.016, &input);
        assert_eq!(title.selected_index, 1);
    }

    #[test]
    fn test_confirm_selection() {
        let mut title = TitleScreen::new();
        let mut input = InputStateResource::new();
        input.set_action(InputAction::Confirm, InputState::Pressed);
        let result = title.update(0.016, &input);
        assert_eq!(result, Some(MenuOption::NewGame));
        assert!(!title.visible);
    }

    #[test]
    fn test_handle_new_game() {
        let mut state_manager = GameStateManager::new();
        TitleScreen::handle_selection(MenuOption::NewGame, &mut state_manager);
        assert_eq!(state_manager.current(), GameState::CharacterSelect);
    }

    #[test]
    fn test_handle_continue() {
        let mut state_manager = GameStateManager::new();
        TitleScreen::handle_selection(MenuOption::Continue, &mut state_manager);
        assert_eq!(state_manager.current(), GameState::Loading);
    }

    #[test]
    fn test_reset() {
        let mut title = TitleScreen::new();
        title.selected_index = 2;
        title.visible = false;
        title.reset();
        assert_eq!(title.selected_index, 0);
        assert!(title.visible);
    }

    #[test]
    fn test_all_options_have_descriptions() {
        for option in MenuOption::all() {
            assert!(!option.description().is_empty());
        }
    }
}
