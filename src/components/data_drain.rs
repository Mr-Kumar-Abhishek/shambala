#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDrainState {
    Inactive,
    Charging,
    Ready,
    Active,
    Cooldown,
}

#[derive(Debug, Clone)]
pub struct DataDrain {
    pub state: DataDrainState,
    pub charge_level: f32,
    pub max_charge: f32,
    pub cooldown_timer: f32,
    pub cooldown_duration: f32,
}

impl DataDrain {
    pub fn new() -> Self {
        Self {
            state: DataDrainState::Inactive,
            charge_level: 0.0,
            max_charge: 100.0,
            cooldown_timer: 0.0,
            cooldown_duration: 30.0,
        }
    }

    pub fn charge(&mut self, amount: f32) {
        if self.state == DataDrainState::Inactive || self.state == DataDrainState::Charging {
            self.charge_level = (self.charge_level + amount).min(self.max_charge);
            if self.charge_level >= self.max_charge {
                self.state = DataDrainState::Ready;
            } else {
                self.state = DataDrainState::Charging;
            }
        }
    }

    pub fn activate(&mut self) -> bool {
        if self.state == DataDrainState::Ready {
            self.state = DataDrainState::Active;
            self.charge_level = 0.0;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.state {
            DataDrainState::Cooldown => {
                self.cooldown_timer += dt;
                if self.cooldown_timer >= self.cooldown_duration {
                    self.state = DataDrainState::Inactive;
                    self.cooldown_timer = 0.0;
                }
            }
            DataDrainState::Active => {
                self.state = DataDrainState::Cooldown;
            }
            _ => {}
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == DataDrainState::Ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let dd = DataDrain::new();
        assert_eq!(dd.state, DataDrainState::Inactive);
        assert_eq!(dd.charge_level, 0.0);
    }

    #[test]
    fn test_charging() {
        let mut dd = DataDrain::new();
        dd.charge(50.0);
        assert_eq!(dd.state, DataDrainState::Charging);
        assert_eq!(dd.charge_level, 50.0);
    }

    #[test]
    fn test_full_charge() {
        let mut dd = DataDrain::new();
        dd.charge(100.0);
        assert_eq!(dd.state, DataDrainState::Ready);
        assert!(dd.is_ready());
    }

    #[test]
    fn test_activate() {
        let mut dd = DataDrain::new();
        dd.charge(100.0);
        assert!(dd.activate());
        assert_eq!(dd.state, DataDrainState::Active);
    }

    #[test]
    fn test_activate_when_not_ready() {
        let mut dd = DataDrain::new();
        assert!(!dd.activate());
    }

    #[test]
    fn test_cooldown() {
        let mut dd = DataDrain::new();
        dd.charge(100.0);
        dd.activate();
        dd.update(0.0);
        assert_eq!(dd.state, DataDrainState::Cooldown);
        dd.update(30.0);
        assert_eq!(dd.state, DataDrainState::Inactive);
    }
}