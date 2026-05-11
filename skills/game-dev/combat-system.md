# Skill: Combat System

## Description
Covers the implementation of combat-related features in Shambala: damage calculation formulas, adding new skills/abilities, status effects, the Data Drain mechanic, and party AI behavior patterns.

## Prerequisites
- Understanding of the ECS patterns ([`ecs-patterns.md`](ecs-patterns.md))
- Familiarity with the [`Stats`](../src/resources/stats.rs), [`Health`](../src/components/health.rs), [`Combat`](../src/components/combat.rs) components
- Knowledge of the class system (Twin Blade, Heavy Blade, Long Arm, Wavemaster)

## Steps

### 1. Damage Calculation Formula

The damage formula follows this sequence:

```rust
// src/systems/combat.rs

/// Calculates raw damage before defense mitigation.
/// Formula: base_damage = attacker.attack * skill_multiplier
///          defense_mitigation = 1.0 - (defender.defense / (defender.defense + 100.0))
///          final_damage = max(1, base_damage * defense_mitigation)
pub fn calculate_damage(
    attacker: &Stats,
    defender: &Stats,
    skill_multiplier: f32,
) -> u32 {
    let raw = (attacker.attack as f32 * skill_multiplier).max(1.0);

    // Diminishing returns on defense: def/(def+100)
    let mitigation = 1.0 - (defender.defense as f32 / (defender.defense as f32 + 100.0));
    let damage = (raw * mitigation).round().max(1.0);

    damage as u32
}

/// Calculates damage with critical hit check.
/// Returns (damage, was_critical).
pub fn calculate_damage_with_crit(
    attacker: &Stats,
    defender: &Stats,
    skill_multiplier: f32,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let base = calculate_damage(attacker, defender, skill_multiplier);

    if rng.gen_bool(attacker.crit_rate as f64) {
        let crit_damage = (base as f32 * attacker.crit_damage).round() as u32;
        (crit_damage, true)
    } else {
        (base, false)
    }
}
```

#### Damage Type Modifiers
```rust
pub enum DamageType {
    Physical,   // Mitigated by defense
    Magical,    // Mitigated by magic_defense
    True,       // Ignores all mitigation — minimum damage formula
    DataDrain,  // Affects CorruptionMeter instead of HP
}

pub fn calculate_damage_by_type(
    attacker: &Stats,
    defender: &Stats,
    skill_multiplier: f32,
    damage_type: DamageType,
) -> u32 {
    match damage_type {
        DamageType::Physical => calculate_damage(attacker, defender, skill_multiplier),
        DamageType::Magical => {
            let raw = (attacker.magic_attack as f32 * skill_multiplier).max(1.0);
            let mitigation = 1.0 - (defender.magic_defense as f32 / (defender.magic_defense as f32 + 100.0));
            (raw * mitigation).round().max(1.0) as u32
        }
        DamageType::True => (attacker.attack as f32 * skill_multiplier * 0.5).max(1.0) as u32,
        DamageType::DataDrain => 0, // Handled by DataDrainSystem
    }
}
```

### 2. Add New Skills/Abilities

#### Define Skill Data
```rust
// src/data/skills.rs

/// A skill definition loaded from assets/config/skills.ron
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillDef {
    pub id: String,                    // Unique identifier, e.g. "tb_sword_flurry"
    pub name: String,                  // Display name, e.g. "Sword Flurry"
    pub skill_type: SkillType,         // Melee, Ranged, Magic, Heal, Buff, Debuff, Utility
    pub damage_multiplier: f32,        // Multiplier applied to attacker's attack stat
    pub sp_cost: u32,                  // Skill Points consumed on use
    pub cooldown: f32,                 // Seconds before skill can be used again
    pub cast_time: f32,                // Seconds to channel before effect
    pub range: f32,                    // Maximum distance to target in pixels
    pub area_of_effect: f32,           // 0.0 = single target, >0 = radius in pixels
    pub status_effect: Option<StatusEffectData>, // Status effect applied on hit
    pub animation_key: String,         // Key into the animation spritesheet
    pub unlock_level: u32,             // Minimum character level to use
    pub class: ClassType,              // Which class can use this skill
    pub description: String,           // Tooltip text
}
```

#### Register a New Skill in the Config
```ron
// assets/config/skills.ron
SkillsConfig(
    skills: [
        SkillDef(
            id: "tb_sword_flurry",
            name: "Sword Flurry",
            skill_type: Melee,
            damage_multiplier: 1.5,
            sp_cost: 15,
            cooldown: 3.0,
            cast_time: 0.2,
            range: 48.0,
            area_of_effect: 0.0,
            status_effect: None,
            animation_key: "tb_attack_3",
            unlock_level: 1,
            class: TwinBlade,
            description: "A rapid three-hit combo that builds chain gauge quickly.",
        ),
        // Add new skills here
    ],
)
```

#### Skill Execution System
```rust
// src/systems/combat.rs

pub fn skill_execution_system(
    mut commands: Commands,
    time: Res<Time>,
    mut skill_query: Query<(&mut SkillSet, &mut Stats, &mut Health)>,
    mut target_query: Query<(&mut Health, &Stats)>,
    mut skill_events: EventReader<SkillEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for event in skill_events.read() {
        let Ok((mut skill_set, attacker_stats, _)) = skill_query.get_mut(event.caster) else {
            continue;
        };

        // Find the skill slot
        let skill = skill_set.skills.iter_mut()
            .flatten()
            .find(|s| s.id == event.skill_id);

        let Some(skill) = skill else { continue };

        // Check cooldown
        if skill.current_cooldown > 0.0 { continue; }

        // Check SP cost
        if attacker_stats.current_sp < skill.sp_cost { continue; }

        // Deduct SP and set cooldown
        attacker_stats.current_sp -= skill.sp_cost;
        skill.current_cooldown = skill.cooldown;

        // Apply damage to target
        if let Ok((mut target_health, target_stats)) = target_query.get_mut(event.target) {
            let damage = calculate_damage(attacker_stats, target_stats, skill.damage_multiplier);
            target_health.take_damage(damage);
            damage_events.send(DamageEvent {
                target: event.target,
                source: event.caster,
                damage,
                damage_type: DamageType::Physical,
                is_critical: false,
            });
        }
    }
}
```

### 3. Implement Status Effects

#### Define the Effect Data
```rust
// src/components/status.rs

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusEffectData {
    pub effect_type: StatusEffectType,
    pub duration: f32,         // Total duration in seconds
    pub magnitude: f32,        // e.g., damage per tick, slow percentage
    pub tick_interval: f32,    // Seconds between ticks
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusEffectType {
    Poison,       // Damage over time (DoT)
    Burn,         // Damage over time, stronger but shorter
    Freeze,       // Prevents movement
    Stun,         // Prevents all actions
    Slow,         // Reduces movement speed by magnitude%
    Haste,        // Increases movement speed by magnitude%
    Regen,        // Heals over time
    Barrier,      // Absorbs damage up to magnitude
    AtkUp,        // Increases attack by magnitude%
    DefUp,        // Increases defense by magnitude%
    AtkDown,      // Decreases attack by magnitude%
    DefDown,      // Decreases defense by magnitude%
    DataOverflow, // Data Drain failure debuff — reduces stats
    Corruption,   // Zone corruption — stacking debuff
    Invincible,   // Temporarily immune to all damage
}
```

#### Status Effect Tick System
```rust
// src/systems/status.rs

pub fn status_effect_system(
    time: Res<Time>,
    mut query: Query<(&mut StatusEffects, &mut Health, &mut Stats)>,
) {
    for (mut effects, mut health, mut stats) in query.iter_mut() {
        let mut to_remove = Vec::new();

        for (i, effect) in effects.effects.iter_mut().enumerate() {
            effect.remaining_duration -= time.delta;
            effect.tick_timer += time.delta;

            if effect.tick_timer >= effect.tick_interval {
                effect.tick_timer = 0.0;
                match effect.effect_type {
                    StatusEffectType::Poison | StatusEffectType::Burn => {
                        health.take_damage(effect.magnitude as u32);
                    }
                    StatusEffectType::Regen => {
                        health.heal(effect.magnitude as u32);
                    }
                    StatusEffectType::Barrier => {
                        // Barrier absorbs damage — handled in take_damage
                    }
                    _ => {} // Stat modifiers handled separately
                }
            }

            if effect.remaining_duration <= 0.0 {
                to_remove.push(i);
            }
        }

        // Remove expired effects (reverse order to preserve indices)
        for &i in to_remove.iter().rev() {
            effects.effects.remove(i);
        }
    }
}
```

#### Applying a Status Effect
```rust
pub fn apply_status_effect(
    effects: &mut StatusEffects,
    data: &StatusEffectData,
    source: &str,
) {
    // Check for stacking: same type + same source replaces duration
    if let Some(existing) = effects.effects.iter_mut()
        .find(|e| e.effect_type == data.effect_type && e.source == source)
    {
        existing.remaining_duration = data.duration;
        existing.magnitude = data.magnitude;
    } else {
        effects.effects.push(StatusEffectInstance {
            effect_id: format!("{}_{}", source, effects.effects.len()),
            effect_type: data.effect_type.clone(),
            remaining_duration: data.duration,
            tick_interval: data.tick_interval,
            tick_timer: 0.0,
            magnitude: data.magnitude,
            source: source.to_string(),
        });
    }
}
```

### 4. Data Drain Mechanic

The Data Drain is the signature mechanic. It has a charge gauge, activation, minigame, and result resolution.

#### Gauge Charging
```rust
// src/systems/data_drain.rs

pub fn data_drain_charge_system(
    mut player_query: Query<&mut DataDrain, With<Player>>,
    mut death_events: EventReader<DeathEvent>,
) {
    let Ok(mut drain) = player_query.get_single_mut() else { return; };

    for _event in death_events.read() {
        drain.gauge = (drain.gauge + drain.charge_rate).min(drain.max_gauge);
    }
}
```

#### Activation and Minigame
```rust
pub fn data_drain_activation_system(
    mut commands: Commands,
    mut drain_query: Query<&mut DataDrain, With<Player>>,
    input: Res<InputState>,
    mut drain_events: EventWriter<DataDrainEvent>,
) {
    let Ok(mut drain) = drain_query.get_single_mut() else { return; };

    if input.action_just_pressed(GameAction::DataDrain) && drain.gauge >= drain.max_gauge {
        drain.active = true;
        drain.minigame_active = true;
        drain.minigame_phase = DrainPhase::Minigame;
        drain_events.send(DataDrainEvent::Activated);
    }
}

/// Resolves minigame result and applies rewards or penalties.
pub fn data_drain_result_system(
    mut drain_query: Query<(&mut DataDrain, &mut Inventory), With<Player>>,
    mut corruption_query: Query<&mut CorruptionMeter>,
    mut status_query: Query<&mut StatusEffects, With<Player>>,
    mut drain_results: EventReader<DrainResultEvent>,
) {
    let Ok((mut drain, mut inventory)) = drain_query.get_single_mut() else { return; };

    for result in drain_results.read() {
        drain.gauge = 0.0;
        drain.active = false;
        drain.minigame_active = false;

        if result.success {
            drain.minigame_phase = DrainPhase::Complete;
            // Grant rewards based on score
            let reward = determine_drain_reward(result.score);
            match reward {
                DrainReward::VirusCore => {
                    inventory.add_item(ItemData::virus_core());
                }
                DrainReward::MemoryFragment(id) => {
                    inventory.add_item(ItemData::memory_fragment(id));
                }
                DrainReward::CorruptionPurge => {
                    // Heal party and remove debuffs
                }
                DrainReward::RareDrop(item_id) => {
                    inventory.add_item(ItemData::rare(item_id));
                }
            }
        } else {
            drain.minigame_phase = DrainPhase::Failed;
            // Apply Data Overflow debuff
            if let Ok(mut status) = status_query.get_single_mut() {
                apply_status_effect(&mut status, &StatusEffectData {
                    effect_type: StatusEffectType::DataOverflow,
                    duration: 30.0,
                    magnitude: 0.5, // 50% stat reduction
                    tick_interval: 1.0,
                }, "data_drain_failure");
            }
        }
    }
}
```

#### Boss Corruption Meter
```rust
// Corruption fills separately from HP. When full, boss is vulnerable.
pub fn corruption_system(
    mut boss_query: Query<(&mut CorruptionMeter, &mut StatusEffects)>,
    mut drain_events: EventReader<DataDrainEvent>,
) {
    for event in drain_events.read() {
        if let DataDrainEvent::Completed { target, score } = event {
            if let Ok((mut corruption, mut effects)) = boss_query.get_mut(*target) {
                corruption.current += *score as f32 * corruption.drain_resistance;
                if corruption.current >= corruption.max {
                    corruption.corrupted = true;
                    apply_status_effect(&mut effects, &StatusEffectData {
                        effect_type: StatusEffectType::Stun,
                        duration: 5.0,
                        magnitude: 0.0,
                        tick_interval: 1.0,
                    }, "corruption_break");
                }
            }
        }
    }
}
```

### 5. Party AI Behavior

Party AI uses behavior patterns based on the assigned tactic.

```rust
// src/systems/ai.rs

/// Evaluates companion AI based on their assigned tactic.
pub fn companion_ai_system(
    time: Res<Time>,
    mut companion_query: Query<(
        &mut AIState,
        &PartyMember,
        &CompanionAI,
        &Stats,
        &Health,
        &Position,
        &mut Velocity,
    ), With<Companion>>,
    player_query: Query<&Position, (With<Player>, Without<Companion>)>,
    enemy_query: Query<(&Position, &Health, &EnemyType), Without<Player>>,
) {
    let Ok(player_pos) = player_query.get_single() else { return; };

    for (mut ai, party_member, companion, stats, health, pos, mut velocity) in companion_query.iter_mut() {
        match party_member.tactics {
            PartyTactic::Aggressive => {
                // Find closest enemy and attack
                if let Some((enemy_pos, _, _)) = enemy_query.iter()
                    .min_by(|a, b| {
                        let da = distance(pos, a.0);
                        let db = distance(pos, b.0);
                        da.partial_cmp(&db).unwrap()
                    })
                {
                    // Move toward enemy
                    let dx = enemy_pos.x - pos.x;
                    let dy = enemy_pos.y - pos.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist > ai.attack_radius {
                        velocity.x = (dx / dist) * stats.speed as f32;
                        velocity.y = (dy / dist) * stats.speed as f32;
                    } else {
                        velocity.x = 0.0;
                        velocity.y = 0.0;
                        ai.behavior = AIBehavior::Attack;
                    }
                }
            }
            PartyTactic::Defensive => {
                // Stay near player, heal if low HP
                let dx = player_pos.x - pos.x;
                let dy = player_pos.y - pos.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist > 100.0 {
                    // Move toward player
                    velocity.x = (dx / dist) * stats.speed as f32 * 0.8;
                    velocity.y = (dy / dist) * stats.speed as f32 * 0.8;
                } else {
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }

                // Use heal skill if HP < 50%
                if health.current_hp < health.max_hp / 2 {
                    ai.behavior = AIBehavior::Retreat;
                }
            }
            PartyTactic::Balanced => {
                // Adapt: attack if healthy, defend if hurt
                if health.current_hp > health.max_hp / 2 {
                    // Same as Aggressive
                } else {
                    // Same as Defensive
                }
            }
            PartyTactic::FocusTarget => {
                // Attack the player's current target
            }
            PartyTactic::Scatter => {
                // Spread out from other party members
            }
        }
    }
}

fn distance(a: &Position, b: &Position) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
```

## Examples

### Creating a New Skill: "Flame Wall" for Wavemaster

1. **Add to config** (`assets/config/skills.ron`):
```ron
SkillDef(
    id: "wm_flame_wall",
    name: "Flame Wall",
    skill_type: Magic,
    damage_multiplier: 2.0,
    sp_cost: 35,
    cooldown: 12.0,
    cast_time: 1.0,
    range: 150.0,
    area_of_effect: 60.0, // AoE radius
    status_effect: Some(StatusEffectData(
        effect_type: Burn,
        duration: 6.0,
        magnitude: 15.0, // 15 damage per tick
        tick_interval: 1.5,
    )),
    animation_key: "wm_cast_fire",
    unlock_level: 5,
    class: Wavemaster,
    description: "Creates a wall of flame that burns enemies in the area.",
)
```

2. **Unlock at level up** — the [`SkillSet`](../src/components/skill.rs) component recalculates available skills when level changes.

## Related Skills
- [`ecs-patterns.md`](ecs-patterns.md) — ECS patterns for registering combat systems
- [`area-generation.md`](area-generation.md) — Enemy placement rules for combat encounters
- [`testing-patterns.md`](testing-patterns.md) — How to write tests for combat formulas
- [`../docs/GDD.md`](../docs/GDD.md) — Game design specification for combat
- [`../docs/TECHNICAL_DESIGN.md`](../docs/TECHNICAL_DESIGN.md) — Technical specification for combat systems
