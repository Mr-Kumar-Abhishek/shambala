use crate::core::types::{Class, GameState};
use crate::core::game_state::GameStateManager;
use crate::components::stats::Stats;
use crate::systems::input::InputAction;
use crate::resources::input_state::InputStateResource;

#[derive(Debug, Clone)]
pub struct CharacterPreview {
    pub class: Class,
    pub stats: Stats,
    pub description: &'static str,
}

impl CharacterPreview {
    pub fn new(class: Class) -> Self {
        let stats = Stats::new(class);
        let description = match class {
            Class::TwinBlade => "Fast and agile melee fighter. Excellent speed and combo potential.",
            Class::HeavyBlade => "Powerful tank with high HP and defense. Slow but devastating.",
            Class::LongArm => "Versatile hybrid fighter. Balanced melee and magic capabilities.",
            Class::Wavemaster => "Pure magic user with powerful spells. Fragile but deadly at range.",
        };
        Self { class, stats, description }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::new(Class::TwinBlade),
            Self::new(Class::HeavyBlade),
            Self::new(Class::LongArm),
            Self::new(Class::Wavemaster),
        ]
    }
}

pub struct CharacterSelectScreen {
    pub characters: Vec<CharacterPreview>,
    pub selected_index: usize,
    pub player_name: String,
    pub name_input_active: bool,
    pub confirmed: bool,
    pub visible: bool,
}

impl CharacterSelectScreen {
    pub fn new() -> Self {
        Self {
            characters: CharacterPreview::all(),
            selected_index: 0,
            player_name: String::new(),
            name_input_active: true,
            confirmed: false,
            visible: true,
        }
    }

    pub fn update(&mut self, _dt: f32, input: &InputStateResource) -> Option<(Class, String)> {
        if !self.visible || self.confirmed {
            return None;
        }

        if self.name_input_active {
            // Handle name input (simplified - arrow keys to toggle, confirm to proceed)
            if input.is_action_pressed(InputAction::Confirm) && !self.player_name.is_empty() {
                self.name_input_active = false;
            }
        } else {
            // Character selection
            if input.is_action_pressed(InputAction::MoveLeft) {
                self.selected_index = if self.selected_index == 0 {
                    self.characters.len() - 1
                } else {
                    self.selected_index - 1
                };
            }

            if input.is_action_pressed(InputAction::MoveRight) {
                self.selected_index = (self.selected_index + 1) % self.characters.len();
            }

            if input.is_action_pressed(InputAction::Confirm) {
                self.confirmed = true;
                let selected = self.characters[self.selected_index].class;
                let name = if self.player_name.is_empty() {
                    format!("{:?}", selected)
                } else {
                    self.player_name.clone()
                };
                return Some((selected, name));
            }

            if input.is_action_pressed(InputAction::Cancel) {
                self.name_input_active = true;
            }
        }

        None
    }

    pub fn add_char_to_name(&mut self, c: char) {
        if self.name_input_active && self.player_name.len() < 12 {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                self.player_name.push(c);
            }
        }
    }

    pub fn remove_char_from_name(&mut self) {
        if self.name_input_active && !self.player_name.is_empty() {
            self.player_name.pop();
        }
    }

    pub fn get_selected_class(&self) -> Class {
        self.characters[self.selected_index].class
    }

    pub fn get_selected_preview(&self) -> &CharacterPreview {
        &self.characters[self.selected_index]
    }

    pub fn handle_selection(class: Class, name: &str, state_manager: &mut GameStateManager) {
        log::info!("Character created: {} the {:?}", name, class);
        state_manager.transition_to(GameState::Loading);
    }

    pub fn reset(&mut self) {
        self.selected_index = 0;
        self.player_name.clear();
        self.name_input_active = true;
        self.confirmed = false;
        self.visible = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::input::InputState;

    #[test]
    fn test_character_select_creation() {
        let screen = CharacterSelectScreen::new();
        assert_eq!(screen.characters.len(), 4);
        assert_eq!(screen.selected_index, 0);
        assert!(screen.name_input_active);
    }

    #[test]
    fn test_character_preview_descriptions() {
        let previews = CharacterPreview::all();
        assert_eq!(previews.len(), 4);
        for preview in &previews {
            assert!(!preview.description.is_empty());
        }
    }

    #[test]
    fn test_class_stats_accuracy() {
        let twin = CharacterPreview::new(Class::TwinBlade);
        assert_eq!(twin.stats.agility, 20);
        
        let heavy = CharacterPreview::new(Class::HeavyBlade);
        assert_eq!(heavy.stats.hp, 150);
        
        let wavemaster = CharacterPreview::new(Class::Wavemaster);
        assert_eq!(wavemaster.stats.magic_attack, 20);
    }

    #[test]
    fn test_navigation_left() {
        let mut screen = CharacterSelectScreen::new();
        let mut input = InputStateResource::new();
        screen.name_input_active = false; // Skip name entry
        
        input.set_action(InputAction::MoveLeft, InputState::Pressed);
        screen.update(0.016, &input);
        assert_eq!(screen.selected_index, 3); // Wraps to last
    }

    #[test]
    fn test_navigation_right() {
        let mut screen = CharacterSelectScreen::new();
        let mut input = InputStateResource::new();
        screen.name_input_active = false;
        
        input.set_action(InputAction::MoveRight, InputState::Pressed);
        screen.update(0.016, &input);
        assert_eq!(screen.selected_index, 1);
    }

    #[test]
    fn test_confirm_selection() {
        let mut screen = CharacterSelectScreen::new();
        let mut input = InputStateResource::new();
        screen.name_input_active = false;
        screen.player_name = "Kite".to_string();
        
        input.set_action(InputAction::Confirm, InputState::Pressed);
        let result = screen.update(0.016, &input);
        assert!(result.is_some());
        let (class, name) = result.unwrap();
        assert_eq!(class, Class::TwinBlade);
        assert_eq!(name, "Kite");
    }

    #[test]
    fn test_name_validation() {
        let mut screen = CharacterSelectScreen::new();
        screen.add_char_to_name('K');
        screen.add_char_to_name('i');
        screen.add_char_to_name('t');
        screen.add_char_to_name('e');
        assert_eq!(screen.player_name, "Kite");
    }

    #[test]
    fn test_name_max_length() {
        let mut screen = CharacterSelectScreen::new();
        for c in "ABCDEFGHIJKLMNOP".chars() {
            screen.add_char_to_name(c);
        }
        assert!(screen.player_name.len() <= 12);
    }

    #[test]
    fn test_remove_char() {
        let mut screen = CharacterSelectScreen::new();
        screen.player_name = "Kite".to_string();
        screen.remove_char_from_name();
        assert_eq!(screen.player_name, "Kit");
    }

    #[test]
    fn test_reset() {
        let mut screen = CharacterSelectScreen::new();
        screen.selected_index = 2;
        screen.player_name = "Test".to_string();
        screen.confirmed = true;
        screen.reset();
        assert_eq!(screen.selected_index, 0);
        assert!(screen.player_name.is_empty());
        assert!(!screen.confirmed);
    }

    #[test]
    fn test_get_selected_preview() {
        let screen = CharacterSelectScreen::new();
        let preview = screen.get_selected_preview();
        assert_eq!(preview.class, Class::TwinBlade);
    }
}