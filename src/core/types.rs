use serde::{Deserialize, Serialize};

/// Top-level states the game can be in.
///
/// The [`GameStateManager`](super::game_state::GameStateManager) drives
/// transitions between these states and maintains a push-down stack for
/// sub-states (e.g. opening the menu while exploring).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Engine startup — assets are being registered.
    Boot,
    /// Title / main menu screen.
    Title,
    /// Connecting to the simulated network server.
    Connecting,
    /// Character creation and class selection.
    CharacterSelect,
    /// Loading a save or transitioning between areas.
    Loading,
    /// Free-roam exploration in a field, town, or dungeon.
    Exploring,
    /// Turn-based combat encounter.
    Combat,
    /// In-game menu (options, inventory, party, etc.).
    Menu,
    /// Scripted cutscene / dialogue event.
    Cutscene,
    /// Network connection lost.
    Disconnected,
}

/// Playable character classes, each with a distinct stat profile.
///
/// | Class | Role | Key Stats |
/// |---|---|---|
/// | [`TwinBlade`](Class::TwinBlade) | Melee DPS | High agility |
/// | [`HeavyBlade`](Class::HeavyBlade) | Tank | High HP/defence |
/// | [`LongArm`](Class::LongArm) | Hybrid | Balanced melee & magic |
/// | [`Wavemaster`](Class::Wavemaster) | Mage | High magic attack |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Class {
    TwinBlade,
    HeavyBlade,
    LongArm,
    Wavemaster,
}

/// Elemental affinity for skills and damage typing.
///
/// Used by the [`Skill`](crate::components::skill::Skill) component
/// and the combat system for element-based damage calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Wind,
    Earth,
    Light,
    Dark,
    /// No element (physical / neutral damage).
    Null,
}

/// Temporary status effects that can be applied to entities during combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEffect {
    /// Deals damage over time.
    Poison,
    /// Prevents action for a duration.
    Paralysis,
    /// Target cannot act until awakened.
    Sleep,
    /// Target attacks randomly (ally or enemy).
    Confusion,
    /// Increased attack, lowered defence.
    Berserk,
    /// Special .hack-inspired mechanic that extracts data from enemies.
    DataDrain,
}
