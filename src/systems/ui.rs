#[derive(Debug, Clone)]
pub struct UIElement {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    pub element_type: UIElementType,
}

#[derive(Debug, Clone)]
pub enum UIElementType {
    Text(String),
    Button(String),
    Panel,
    ProgressBar { current: f32, max: f32 },
    Icon(String),
}

#[derive(Debug, Clone)]
pub struct HUD {
    pub hp_bar: UIElement,
    pub mp_bar: UIElement,
    pub data_drain_gauge: UIElement,
    pub party_frames: Vec<UIElement>,
    pub minimap: UIElement,
    pub chat_log: UIElement,
    pub skill_bar: Vec<UIElement>,
}

pub struct UISystem;

impl UISystem {
    pub fn create_hud() -> HUD {
        HUD {
            hp_bar: UIElement {
                id: "hp_bar".to_string(),
                x: 20.0,
                y: 20.0,
                width: 200.0,
                height: 20.0,
                visible: true,
                element_type: UIElementType::ProgressBar {
                    current: 100.0,
                    max: 100.0,
                },
            },
            mp_bar: UIElement {
                id: "mp_bar".to_string(),
                x: 20.0,
                y: 45.0,
                width: 200.0,
                height: 20.0,
                visible: true,
                element_type: UIElementType::ProgressBar {
                    current: 100.0,
                    max: 100.0,
                },
            },
            data_drain_gauge: UIElement {
                id: "data_drain".to_string(),
                x: 20.0,
                y: 70.0,
                width: 200.0,
                height: 15.0,
                visible: true,
                element_type: UIElementType::ProgressBar {
                    current: 0.0,
                    max: 100.0,
                },
            },
            party_frames: Vec::new(),
            minimap: UIElement {
                id: "minimap".to_string(),
                x: 1100.0,
                y: 20.0,
                width: 160.0,
                height: 160.0,
                visible: true,
                element_type: UIElementType::Panel,
            },
            chat_log: UIElement {
                id: "chat_log".to_string(),
                x: 20.0,
                y: 600.0,
                width: 400.0,
                height: 100.0,
                visible: true,
                element_type: UIElementType::Panel,
            },
            skill_bar: vec![UIElement {
                id: "skill_1".to_string(),
                x: 540.0,
                y: 680.0,
                width: 40.0,
                height: 40.0,
                visible: true,
                element_type: UIElementType::Button("Skill 1".to_string()),
            }],
        }
    }

    pub fn update_progress_bar(element: &mut UIElement, current: f32, max: f32) {
        if let UIElementType::ProgressBar {
            current: ref mut cur,
            max: ref mut m,
        } = &mut element.element_type
        {
            *cur = current;
            *m = max;
        }
    }

    pub fn is_clicked(element: &UIElement, mouse_x: f32, mouse_y: f32) -> bool {
        mouse_x >= element.x
            && mouse_x <= element.x + element.width
            && mouse_y >= element.y
            && mouse_y <= element.y + element.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hud() {
        let hud = UISystem::create_hud();
        assert_eq!(hud.hp_bar.id, "hp_bar");
        assert!(hud.hp_bar.visible);
    }

    #[test]
    fn test_update_progress_bar() {
        let mut element = UIElement {
            id: "test".to_string(),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 20.0,
            visible: true,
            element_type: UIElementType::ProgressBar {
                current: 50.0,
                max: 100.0,
            },
        };
        UISystem::update_progress_bar(&mut element, 75.0, 100.0);
        if let UIElementType::ProgressBar { current, max } = &element.element_type {
            assert_eq!(*current, 75.0);
            assert_eq!(*max, 100.0);
        } else {
            panic!("Expected ProgressBar");
        }
    }

    #[test]
    fn test_is_clicked() {
        let element = UIElement {
            id: "btn".to_string(),
            x: 100.0,
            y: 100.0,
            width: 50.0,
            height: 30.0,
            visible: true,
            element_type: UIElementType::Button("Test".to_string()),
        };
        assert!(UISystem::is_clicked(&element, 120.0, 110.0));
        assert!(!UISystem::is_clicked(&element, 50.0, 50.0));
    }
}
