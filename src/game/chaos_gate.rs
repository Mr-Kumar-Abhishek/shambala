use crate::entities::area::{Area, AreaEntity};

#[derive(Debug, Clone)]
pub struct ChaosGate {
    pub is_active: bool,
    pub selected_keywords: Vec<String>,
    pub available_keyword_sets: Vec<Vec<String>>,
    pub current_set_index: usize,
    pub current_keyword_index: usize,
    pub generated_area: Option<Area>,
    pub transition_progress: f32,
    pub is_transitioning: bool,
}

impl Default for ChaosGate {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosGate {
    pub fn new() -> Self {
        let keyword_sets = AreaEntity::get_area_keywords()
            .into_iter()
            .map(|set| set.into_iter().map(|s| s.to_string()).collect())
            .collect();

        Self {
            is_active: false,
            selected_keywords: Vec::new(),
            available_keyword_sets: keyword_sets,
            current_set_index: 0,
            current_keyword_index: 0,
            generated_area: None,
            transition_progress: 0.0,
            is_transitioning: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.selected_keywords.clear();
        self.current_set_index = 0;
        self.current_keyword_index = 0;
        self.transition_progress = 0.0;
        self.is_transitioning = false;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.generated_area = None;
    }

    pub fn select_keyword(&mut self, keyword: &str) {
        if self.selected_keywords.len() < 3 {
            self.selected_keywords.push(keyword.to_string());
            if self.selected_keywords.len() < 3 {
                self.current_set_index =
                    (self.current_set_index + 1) % self.available_keyword_sets.len();
                self.current_keyword_index = 0;
            }
        }
    }

    pub fn remove_last_keyword(&mut self) {
        self.selected_keywords.pop();
        if !self.selected_keywords.is_empty() {
            self.current_set_index =
                (self.selected_keywords.len() - 1).min(self.available_keyword_sets.len() - 1);
        }
    }

    pub fn confirm_area(&mut self, seed: u64) {
        if self.selected_keywords.len() == 3 {
            self.is_transitioning = true;
            self.transition_progress = 0.0;

            // Generate area based on keywords
            let area = AreaEntity::create_field(&self.selected_keywords, seed);
            self.generated_area = Some(area);
        }
    }

    pub fn update_transition(&mut self, dt: f32) -> bool {
        if !self.is_transitioning {
            return false;
        }

        self.transition_progress += dt * 0.5; // 2 second transition
        if self.transition_progress >= 1.0 {
            self.transition_progress = 1.0;
            self.is_transitioning = false;
            return true; // Transition complete
        }
        false
    }

    pub fn get_current_keywords(&self) -> &[String] {
        if self.current_set_index < self.available_keyword_sets.len() {
            &self.available_keyword_sets[self.current_set_index]
        } else {
            &[]
        }
    }

    pub fn keyword_count(&self) -> usize {
        self.selected_keywords.len()
    }

    pub fn is_ready(&self) -> bool {
        self.selected_keywords.len() == 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaos_gate_creation() {
        let gate = ChaosGate::new();
        assert!(!gate.is_active);
        assert_eq!(gate.available_keyword_sets.len(), 5);
    }

    #[test]
    fn test_activate() {
        let mut gate = ChaosGate::new();
        gate.activate();
        assert!(gate.is_active);
        assert!(gate.selected_keywords.is_empty());
    }

    #[test]
    fn test_select_keywords() {
        let mut gate = ChaosGate::new();
        gate.activate();
        gate.select_keyword("forest");
        assert_eq!(gate.keyword_count(), 1);
        gate.select_keyword("lake");
        assert_eq!(gate.keyword_count(), 2);
        gate.select_keyword("temple");
        assert_eq!(gate.keyword_count(), 3);
    }

    #[test]
    fn test_remove_keyword() {
        let mut gate = ChaosGate::new();
        gate.activate();
        gate.select_keyword("forest");
        gate.select_keyword("lake");
        gate.remove_last_keyword();
        assert_eq!(gate.keyword_count(), 1);
    }

    #[test]
    fn test_confirm_area() {
        let mut gate = ChaosGate::new();
        gate.activate();
        gate.select_keyword("forest");
        gate.select_keyword("lake");
        gate.select_keyword("temple");
        gate.confirm_area(42);
        assert!(gate.is_transitioning);
        assert!(gate.generated_area.is_some());
    }

    #[test]
    fn test_transition_complete() {
        let mut gate = ChaosGate::new();
        gate.activate();
        gate.select_keyword("forest");
        gate.select_keyword("lake");
        gate.select_keyword("temple");
        gate.confirm_area(42);

        // Simulate transition
        let mut progress = 0.0;
        while progress < 1.0 {
            if gate.update_transition(0.5) {
                break;
            }
            progress += 0.25;
        }
        assert!(!gate.is_transitioning);
    }

    #[test]
    fn test_is_ready() {
        let mut gate = ChaosGate::new();
        gate.activate();
        assert!(!gate.is_ready());
        gate.select_keyword("a");
        assert!(!gate.is_ready());
        gate.select_keyword("b");
        assert!(!gate.is_ready());
        gate.select_keyword("c");
        assert!(gate.is_ready());
    }

    #[test]
    fn test_deactivate() {
        let mut gate = ChaosGate::new();
        gate.activate();
        gate.select_keyword("forest");
        gate.deactivate();
        assert!(!gate.is_active);
        assert!(gate.generated_area.is_none());
    }
}
