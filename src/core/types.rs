use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Boot,
    Title,
    Connecting,
    CharacterSelect,
    Loading,
    Exploring,
    Combat,
    Menu,
    Cutscene,
    Disconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Class {
    TwinBlade,
    HeavyBlade,
    LongArm,
    Wavemaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Wind,
    Earth,
    Light,
    Dark,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEffect {
    Poison,
    Paralysis,
    Sleep,
    Confusion,
    Berserk,
    DataDrain,
}