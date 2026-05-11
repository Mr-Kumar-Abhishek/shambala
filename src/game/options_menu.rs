use crate::systems::input::InputAction;
use crate::resources::input_state::InputStateResource;
use crate::systems::audio_playback::AudioPlaybackSystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsTab {
    Audio,
    Controls,
    Display,
}

impl OptionsTab {
    pub fn all() -> &'static [OptionsTab] {
        &[OptionsTab::Audio, OptionsTab::Controls, OptionsTab::Display]
    }

    pub fn label(&self) -> &'static str {
        match self {
            OptionsTab::Audio => "Audio",
            OptionsTab::Controls => "Controls",
            OptionsTab::Display => "Display",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionsMenuItem {
    pub id: String,
    pub label: String,
    pub value: String,
    pub item_type: OptionsItemType,
}

#[derive(Debug, Clone)]
pub enum OptionsItemType {
    Slider { min: f32, max: f32, current: f32, step: f32 },
    Toggle { enabled: bool },
    KeyBinding { action: String, key: String },
    Action,
}

pub struct OptionsMenu {
    pub visible: bool,
    pub selected_tab: usize,
    pub tabs: Vec<OptionsTab>,
    pub selected_item: usize,
    pub items: Vec<Vec<OptionsMenuItem>>,
    pub is_adjusting: bool,
}

impl OptionsMenu {
    pub fn new() -> Self {
        let tabs = OptionsTab::all().to_vec();
        let items = vec![
            Self::create_audio_items(),
            Self::create_controls_items(),
            Self::create_display_items(),
        ];

        Self {
            visible: false,
            selected_tab: 0,
            tabs,
            selected_item: 0,
            items,
            is_adjusting: false,
        }
    }

    fn create_audio_items() -> Vec<OptionsMenuItem> {
        vec![
            OptionsMenuItem {
                id: "master_volume".to_string(),
                label: "Master Volume".to_string(),
                value: "100%".to_string(),
                item_type: OptionsItemType::Slider { min: 0.0, max: 100.0, current: 100.0, step: 5.0 },
            },
            OptionsMenuItem {
                id: "bgm_volume".to_string(),
                label: "BGM Volume".to_string(),
                value: "70%".to_string(),
                item_type: OptionsItemType::Slider { min: 0.0, max: 100.0, current: 70.0, step: 5.0 },
            },
            OptionsMenuItem {
                id: "sfx_volume".to_string(),
                label: "SFX Volume".to_string(),
                value: "100%".to_string(),
                item_type: OptionsItemType::Slider { min: 0.0, max: 100.0, current: 100.0, step: 5.0 },
            },
            OptionsMenuItem {
                id: "mute".to_string(),
                label: "Mute All".to_string(),
                value: "Off".to_string(),
                item_type: OptionsItemType::Toggle { enabled: false },
            },
        ]
    }

    fn create_controls_items() -> Vec<OptionsMenuItem> {
        vec![
            OptionsMenuItem {
                id: "move_up".to_string(), label: "Move Up".to_string(),
                value: "W / Up Arrow".to_string(),
                item_type: OptionsItemType::KeyBinding { action: "move_up".to_string(), key: "W".to_string() },
            },
            OptionsMenuItem {
                id: "move_down".to_string(), label: "Move Down".to_string(),
                value: "S / Down Arrow".to_string(),
                item_type: OptionsItemType::KeyBinding { action: "move_down".to_string(), key: "S".to_string() },
            },
            OptionsMenuItem {
                id: "move_left".to_string(), label: "Move Left".to_string(),
                value: "A / Left Arrow".to_string(),
                item_type: OptionsItemType::KeyBinding { action: "move_left".to_string(), key: "A".to_string() },
            },
            OptionsMenuItem {
                id: "move_right".to_string(), label: "Move Right".to_string(),
                value: "D / Right Arrow".to_string(),
                item_type: OptionsItemType::KeyBinding { action: "move_right".to_string(), key: "D".to_string() },
            },
            OptionsMenuItem {
                id: "confirm".to_string(), label: "Confirm".to_string(),
                value: "Enter / Space".to_string(),
                item_type: OptionsItemType::KeyBinding { action: "confirm".to_string(), key: "Enter".to_string() },
            },
            OptionsMenuItem {
                id: "cancel".to_string(), label: "Cancel".to_string(),
                value: "Escape".to_string(),
                item_type: OptionsItemType::KeyBinding { action: "cancel".to_string(), key: "Escape".to_string() },
            },
        ]
    }

    fn create_display_items() -> Vec<OptionsMenuItem> {
        vec![
            OptionsMenuItem {
                id: "resolution".to_string(), label: "Resolution".to_string(),
                value: "1280x720".to_string(),
                item_type: OptionsItemType::Action,
            },
            OptionsMenuItem {
                id: "fullscreen".to_string(), label: "Fullscreen".to_string(),
                value: "Windowed".to_string(),
                item_type: OptionsItemType::Toggle { enabled: false },
            },
            OptionsMenuItem {
                id: "vsync".to_string(), label: "V-Sync".to_string(),
                value: "On".to_string(),
                item_type: OptionsItemType::Toggle { enabled: true },
            },
            OptionsMenuItem {
                id: "fps_display".to_string(), label: "Show FPS".to_string(),
                value: "Off".to_string(),
                item_type: OptionsItemType::Toggle { enabled: false },
            },
        ]
    }

    pub fn open(&mut self) {
        self.visible = true;
        self.selected_tab = 0;
        self.selected_item = 0;
        self.is_adjusting = false;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn update(&mut self, input: &InputStateResource, audio: &mut AudioPlaybackSystem) {
        if !self.visible {
            return;
        }

        if input.is_action_pressed(InputAction::Cancel) {
            self.close();
            return;
        }

        if self.is_adjusting {
            self.handle_adjustment(input, audio);
            return;
        }

        // Tab navigation
        if input.is_action_pressed(InputAction::MoveLeft) && self.selected_item == 0 {
            self.selected_tab = if self.selected_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.selected_tab - 1
            };
            self.selected_item = 0;
        }

        if input.is_action_pressed(InputAction::MoveRight) && self.selected_item == 0 {
            self.selected_tab = (self.selected_tab + 1) % self.tabs.len();
            self.selected_item = 0;
        }

        // Item navigation
        let current_items = &self.items[self.selected_tab];
        if input.is_action_pressed(InputAction::MoveUp) {
            self.selected_item = if self.selected_item == 0 {
                current_items.len() - 1
            } else {
                self.selected_item - 1
            };
        }

        if input.is_action_pressed(InputAction::MoveDown) {
            self.selected_item = (self.selected_item + 1) % current_items.len();
        }

        // Confirm to adjust/toggle
        if input.is_action_pressed(InputAction::Confirm) {
            if self.selected_item < current_items.len() {
                match &current_items[self.selected_item].item_type {
                    OptionsItemType::Slider { .. } => {
                        self.is_adjusting = true;
                    }
                    OptionsItemType::Toggle { .. } => {
                        // Toggle will be handled in a real implementation
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_adjustment(&mut self, input: &InputStateResource, audio: &mut AudioPlaybackSystem) {
        if input.is_action_pressed(InputAction::Cancel) || input.is_action_pressed(InputAction::Confirm) {
            self.is_adjusting = false;
            // Apply audio settings
            if let Some(item) = self.items[self.selected_tab].get_mut(self.selected_item) {
                if let OptionsItemType::Slider { current, .. } = &mut item.item_type {
                    audio.set_master_volume(*current / 100.0);
                    item.value = format!("{}%", *current as u32);
                }
            }
            return;
        }

        if let Some(item) = self.items[self.selected_tab].get_mut(self.selected_item) {
            if let OptionsItemType::Slider { min, max, current, step } = &mut item.item_type {
                if input.is_action_pressed(InputAction::MoveLeft) {
                    *current = (*current - *step).max(*min);
                }
                if input.is_action_pressed(InputAction::MoveRight) {
                    *current = (*current + *step).min(*max);
                }
            }
        }
    }

    pub fn get_current_items(&self) -> &[OptionsMenuItem] {
        &self.items[self.selected_tab]
    }

    pub fn get_current_tab_label(&self) -> &str {
        self.tabs[self.selected_tab].label()
    }

    pub fn is_open(&self) -> bool {
        self.visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::input::InputState;

    #[test]
    fn test_options_menu_creation() {
        let menu = OptionsMenu::new();
        assert!(!menu.visible);
        assert_eq!(menu.tabs.len(), 3);
        assert_eq!(menu.items.len(), 3);
    }

    #[test]
    fn test_open_close() {
        let mut menu = OptionsMenu::new();
        menu.open();
        assert!(menu.visible);
        menu.close();
        assert!(!menu.visible);
    }

    #[test]
    fn test_audio_items_count() {
        let items = OptionsMenu::create_audio_items();
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn test_controls_items_count() {
        let items = OptionsMenu::create_controls_items();
        assert_eq!(items.len(), 6);
    }

    #[test]
    fn test_display_items_count() {
        let items = OptionsMenu::create_display_items();
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn test_tab_labels() {
        assert_eq!(OptionsTab::Audio.label(), "Audio");
        assert_eq!(OptionsTab::Controls.label(), "Controls");
        assert_eq!(OptionsTab::Display.label(), "Display");
    }

    #[test]
    fn test_tab_navigation() {
        let mut menu = OptionsMenu::new();
        menu.open();
        
        let mut input = InputStateResource::new();
        input.set_action(InputAction::MoveRight, InputState::Pressed);
        let mut audio = AudioPlaybackSystem::new();
        menu.update(&input, &mut audio);
        assert_eq!(menu.selected_tab, 1);
    }

    #[test]
    fn test_slider_adjustment() {
        let mut menu = OptionsMenu::new();
        menu.open();
        
        let mut audio = AudioPlaybackSystem::new();
        let mut input = InputStateResource::new();
        
        // Enter adjustment mode
        input.set_action(InputAction::Confirm, InputState::Pressed);
        menu.update(&input, &mut audio);
        
        // Adjust left
        let mut adjust_input = InputStateResource::new();
        adjust_input.set_action(InputAction::MoveLeft, InputState::Pressed);
        menu.update(&adjust_input, &mut audio);
        
        // Confirm to exit adjustment
        let mut confirm_input = InputStateResource::new();
        confirm_input.set_action(InputAction::Confirm, InputState::Pressed);
        menu.update(&confirm_input, &mut audio);
    }

    #[test]
    fn test_close_with_cancel() {
        let mut menu = OptionsMenu::new();
        menu.open();
        
        let mut input = InputStateResource::new();
        input.set_action(InputAction::Cancel, InputState::Pressed);
        let mut audio = AudioPlaybackSystem::new();
        menu.update(&input, &mut audio);
        assert!(!menu.visible);
    }
}