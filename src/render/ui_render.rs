use crate::systems::ui::{UIElement, UIElementType};

#[derive(Debug, Clone)]
pub struct UIRenderer {
    pub elements: Vec<UIElement>,
}

impl UIRenderer {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: UIElement) {
        self.elements.push(element);
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn get_visible_elements(&self) -> Vec<&UIElement> {
        self.elements.iter()
            .filter(|e| e.visible)
            .collect()
    }

    pub fn get_progress_bars(&self) -> Vec<(&UIElement, f32, f32)> {
        self.elements.iter()
            .filter_map(|e| {
                if let UIElementType::ProgressBar { current, max } = &e.element_type {
                    Some((e, *current, *max))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_buttons(&self) -> Vec<&UIElement> {
        self.elements.iter()
            .filter(|e| matches!(e.element_type, UIElementType::Button(_)))
            .collect()
    }

    pub fn get_text_elements(&self) -> Vec<(&UIElement, &str)> {
        self.elements.iter()
            .filter_map(|e| {
                if let UIElementType::Text(text) = &e.element_type {
                    Some((e, text.as_str()))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_element() {
        let mut renderer = UIRenderer::new();
        renderer.add_element(UIElement {
            id: "hp_bar".to_string(), x: 0.0, y: 0.0,
            width: 200.0, height: 20.0, visible: true,
            element_type: UIElementType::ProgressBar { current: 75.0, max: 100.0 },
        });
        assert_eq!(renderer.elements.len(), 1);
    }

    #[test]
    fn test_visible_elements() {
        let mut renderer = UIRenderer::new();
        renderer.add_element(UIElement {
            id: "visible".to_string(), x: 0.0, y: 0.0,
            width: 100.0, height: 100.0, visible: true,
            element_type: UIElementType::Panel,
        });
        renderer.add_element(UIElement {
            id: "hidden".to_string(), x: 0.0, y: 0.0,
            width: 100.0, height: 100.0, visible: false,
            element_type: UIElementType::Panel,
        });
        assert_eq!(renderer.get_visible_elements().len(), 1);
    }

    #[test]
    fn test_progress_bars() {
        let mut renderer = UIRenderer::new();
        renderer.add_element(UIElement {
            id: "hp".to_string(), x: 0.0, y: 0.0,
            width: 200.0, height: 20.0, visible: true,
            element_type: UIElementType::ProgressBar { current: 50.0, max: 100.0 },
        });
        let bars = renderer.get_progress_bars();
        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].1, 50.0);
    }

    #[test]
    fn test_buttons() {
        let mut renderer = UIRenderer::new();
        renderer.add_element(UIElement {
            id: "btn".to_string(), x: 0.0, y: 0.0,
            width: 100.0, height: 30.0, visible: true,
            element_type: UIElementType::Button("Start".to_string()),
        });
        assert_eq!(renderer.get_buttons().len(), 1);
    }
}