/// Charge states for the Data Drain mechanic.
///
/// Inspired by the .hack series, Data Drain lets the player extract
/// data fragments from weakened enemies. It must be charged through
/// combat, then activated within a window of opportunity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDrainState {
    /// No charge accumulated yet.
    Inactive,
    /// Building charge from combat actions.
    Charging,
    /// Fully charged and ready to activate.
    Ready,
    /// Currently being used (brief transition state).
    Active,
    /// Cooling down after use; cannot be used again yet.
    Cooldown,
}

/// Data Drain gauge attached to the player entity.
///
/// Charge is built by dealing damage in combat. Once full, the player
/// can activate the drain on a weakened enemy to extract bonus rewards.
#[derive(Debug, Clone)]
pub struct DataDrain {
    /// Current charge state machine.
    pub state: DataDrainState,
    /// Current charge amount (0.0 – [`max_charge`](Self::max_charge)).
    pub charge_level: f32,
    /// Maximum charge required to reach the Ready state.
    pub max_charge: f32,
    /// Elapsed time in the cooldown state.
    pub cooldown_timer: f32,
    /// Total duration of the cooldown period (seconds).
    pub cooldown_duration: f32,
}

impl DataDrain {
    /// Create a new Data Drain gauge starting empty.
    pub fn new() -> Self {
        Self {
            state: DataDrainState::Inactive,
            charge_level: 0.0,
            max_charge: 100.0,
            cooldown_timer: 0.0,
            cooldown_duration: 30.0,
        }
    }

    /// Add charge from combat damage.
    ///
    /// Transitions from [`Inactive`](DataDrainState::Inactive) →
    /// [`Charging`](DataDrainState::Charging) → [`Ready`](DataDrainState::Ready).
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

    /// Attempt to activate the drain.
    ///
    /// Returns `true` if the drain was Ready and is now Active,
    /// or `false` if it was not ready.
    pub fn activate(&mut self) -> bool {
        if self.state == DataDrainState::Ready {
            self.state = DataDrainState::Active;
            self.charge_level = 0.0;
            true
        } else {
            false
        }
    }

    /// Advance the cooldown timer each frame.
    ///
    /// After an activation the state moves to
    /// [`Active`](DataDrainState::Active) → [`Cooldown`](DataDrainState::Cooldown) →
    /// [`Inactive`](DataDrainState::Inactive).
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

    /// Returns `true` when the gauge is fully charged and ready.
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
        assert_eq!(dd.charge_level, 0.0);
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
        dd.update(0.0); // Active → Cooldown
        assert_eq!(dd.state, DataDrainState::Cooldown);
        dd.update(30.0); // Finish cooldown
        assert_eq!(dd.state, DataDrainState::Inactive);
    }
}