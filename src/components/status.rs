use crate::core::types::StatusEffect;

#[derive(Debug, Clone)]
pub struct StatusEffectInstance {
    pub effect: StatusEffect,
    pub duration: f32,
    pub remaining: f32,
    pub potency: u32,
}

#[derive(Debug, Clone)]
pub struct StatusEffects {
    pub effects: Vec<StatusEffectInstance>,
}

impl Default for StatusEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusEffects {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn add(&mut self, effect: StatusEffect, duration: f32, potency: u32) {
        self.effects.push(StatusEffectInstance {
            effect,
            duration,
            remaining: duration,
            potency,
        });
    }

    pub fn update(&mut self, dt: f32) {
        self.effects.retain_mut(|e| {
            e.remaining -= dt;
            e.remaining > 0.0
        });
    }

    pub fn has_effect(&self, effect: StatusEffect) -> bool {
        self.effects.iter().any(|e| e.effect == effect)
    }

    pub fn clear(&mut self) {
        self.effects.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_effect() {
        let mut effects = StatusEffects::new();
        effects.add(StatusEffect::Poison, 10.0, 5);
        assert_eq!(effects.effects.len(), 1);
    }

    #[test]
    fn test_effect_expiry() {
        let mut effects = StatusEffects::new();
        effects.add(StatusEffect::Poison, 1.0, 5);
        effects.update(1.5);
        assert!(effects.effects.is_empty());
    }

    #[test]
    fn test_has_effect() {
        let mut effects = StatusEffects::new();
        effects.add(StatusEffect::Paralysis, 5.0, 1);
        assert!(effects.has_effect(StatusEffect::Paralysis));
        assert!(!effects.has_effect(StatusEffect::Poison));
    }
}
