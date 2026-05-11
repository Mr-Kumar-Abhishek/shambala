# Skill: Agile Sprint

## Description
Defines the Agile sprint workflow for the Shambala project. This skill covers sprint planning, daily standups, sprint reviews, retrospectives, story point estimation, and task board management.

## Prerequisites
- Access to the project task board (GitHub Projects / Jira / Linear)
- Understanding of the project's skill files and documentation
- Familiarity with Agile/Scrum terminology

## Steps

### 1. Sprint Planning Process

Sprint planning occurs at the start of each sprint. The team selects work items from the backlog and commits to a sprint goal.

#### Pre-Planning Preparation

1. **Product Owner** grooms the backlog before planning:
   - Ensure top-priority items have clear acceptance criteria
   - Attach relevant skill files to each item (e.g., [`ecs-patterns.md`](../game-dev/ecs-patterns.md) for ECS work)
   - Link related documentation ([`../docs/GDD.md`](../docs/GDD.md), [`../docs/TECHNICAL_DESIGN.md`](../docs/TECHNICAL_DESIGN.md))
   - Remove items that are no longer relevant

2. **Team members** review the candidate backlog items before planning:
   - Read linked skill files to understand implementation patterns
   - Note questions or dependencies
   - Identify any blocking issues

#### Planning Meeting Agenda

| Step | Duration | Activity |
|---|---|---|
| 1 | 10 min | Review sprint goal and priorities |
| 2 | 20 min | Walk through candidate backlog items |
| 3 | 15 min | Estimate story points (see Section 4) |
| 4 | 10 min | Commit to sprint scope |
| 5 | 5 min | Assign initial tasks |

#### Sprint Goal Template

```
Sprint [N]: [Goal Statement]

Focus Area: [e.g., Combat System, Area Generation, UI]
Key Deliverables:
- [Deliverable 1]
- [Deliverable 2]

Non-Goals:
- [Explicitly excluded items]
```

**Example:**
```
Sprint 3: Core Combat Loop

Focus Area: Combat System
Key Deliverables:
- Basic attack and damage calculation
- Skill execution system with cooldowns
- Party AI behaviour patterns

Non-Goals:
- Data Drain minigame (deferred to Sprint 4)
- Equipment system (deferred to Sprint 5)
```

### 2. Daily Standup Format

Daily standups are time-boxed to **15 minutes**. Each team member answers three questions:

#### The Three Questions

1. **What did I accomplish yesterday?**
   - Reference specific tasks from the board
   - Mention any completed TDD cycles (RED/GREEN/REFACTOR)
   - Link to merged PRs if applicable

2. **What will I work on today?**
   - State the next task from the board
   - Mention which skill file guides the work
   - Call out the TDD phase (writing tests first)

3. **What blockers do I have?**
   - Technical blockers (e.g., "ECS query filter not working as expected")
   - Dependency blockers (e.g., "Waiting on area generation data format")
   - Knowledge blockers (e.g., "Need clarification on damage formula")

#### Standup Etiquette

- Standups are for **status sharing**, not problem-solving
- Blockers are noted and addressed in a separate **breakout session** after standup
- Keep updates concise — 2-3 minutes per person maximum
- Use the task board as a visual reference during standup

#### Async Standup Format (Remote Days)

When team members are in different time zones, use an async check-in:

```
# Daily Check-in — [Date]

## @username
- **Yesterday:** [accomplishment]
- **Today:** [planned work]
- **Blockers:** [none / description]
- **TDD Phase:** RED / GREEN / REFACTOR
```

### 3. Sprint Review and Retrospective

#### Sprint Review

Held at the end of each sprint. Focus is on **demonstrating working software**.

**Agenda (60 minutes):**

| Step | Duration | Activity |
|---|---|---|
| 1 | 5 min | Recap sprint goal |
| 2 | 30 min | Demo completed features |
| 3 | 15 min | Review metrics (velocity, burndown) |
| 4 | 10 min | Update backlog based on feedback |

**Demo Checklist:**
- [ ] Feature works in the actual game (not just unit tests)
- [ ] Edge cases are handled gracefully
- [ ] Performance is acceptable (no frame drops)
- [ ] Tests are passing (`cargo t-all`)
- [ ] Documentation is updated

#### Sprint Retrospective

Held after the sprint review. Focus is on **process improvement**.

**Agenda (45 minutes):**

| Step | Duration | Activity |
|---|---|---|
| 1 | 5 min | Set the stage (safety check) |
| 2 | 10 min | What went well? |
| 3 | 10 min | What could be improved? |
| 4 | 10 min | Action items for next sprint |
| 5 | 10 min | Close and assign action items |

**Retrospective Format (Start-Stop-Continue):**

```
## What went well (Continue)
- [ ] Item 1
- [ ] Item 2

## What could be improved (Stop)
- [ ] Item 1
- [ ] Item 2

## New ideas to try (Start)
- [ ] Item 1 (owner: @username)
- [ ] Item 2 (owner: @username)
```

**Example Retrospective Output:**

```
## Continue
- TDD cycle is working well — catching bugs early
- Daily standups are focused and efficient

## Stop
- PR reviews taking longer than 24 hours
- Skipping REFACTOR phase in the TDD cycle

## Start
- Set up PR review rotation schedule
- Add REFACTOR checklist item to Definition of Done
```

### 4. Story Point Estimation

Use **Fibonacci sequence** (1, 2, 3, 5, 8, 13) for story points. Points represent **relative effort**, not hours.

#### Estimation Scale

| Points | Meaning | Example |
|---|---|---|
| 1 | Trivial — known pattern, minimal code | Add a new component with existing fields |
| 2 | Small — straightforward implementation | Add a new system with known query pattern |
| 3 | Medium — some unknowns | Implement a new status effect type |
| 5 | Large — multiple systems or new patterns | Implement the Data Drain minigame |
| 8 | Very large — significant new subsystem | Implement the area generation pipeline |
| 13 | Epic — needs to be broken down | Full combat system with all skills |

#### Estimation Technique: Planning Poker

1. Each team member privately selects a point value
2. Values are revealed simultaneously
3. If values differ significantly (e.g., 3 vs 8), discuss:
   - What makes this item complex?
   - Which skill files apply?
   - Are there dependencies?
4. Re-vote until consensus is reached

#### When to Break Down Items

An item is **too large** (needs splitting) if:
- Estimated at 13+ points
- Cannot be completed within one sprint
- Spans multiple skill domains (e.g., both [`rendering-pipeline.md`](../game-dev/rendering-pipeline.md) and [`combat-system.md`](../game-dev/combat-system.md))
- Has unclear acceptance criteria

### 5. Task Board Management

The task board tracks all work items through their lifecycle.

#### Board Columns

| Column | Definition | Exit Criteria |
|---|---|---|
| **Backlog** | All known work, prioritised | Has acceptance criteria and story points |
| **Ready** | Groomed and ready for sprint | Estimated, dependencies resolved |
| **In Progress** | Currently being worked on | Assigned to a team member |
| **In Review** | PR submitted, awaiting review | PR approved by reviewer |
| **Done** | Merged and deployed | Meets Definition of Done |

#### Task Card Template

```
Title: [Short description]

Description:
[Detailed description of the work]

Acceptance Criteria:
- [ ] Criterion 1
- [ ] Criterion 2

Technical Notes:
- Skill file: [link to relevant skill]
- Design doc: [link to relevant section]

Estimate: [Story points]
Assignee: @username
Labels: [area/combat, type/feat, phase/green]
```

#### Work In Progress (WIP) Limits

| Column | WIP Limit | Rationale |
|---|---|---|
| In Progress | 2 per person | Focus on completing before starting new work |
| In Review | 3 total | Ensure timely feedback on PRs |

#### Moving Tasks Across the Board

```
Backlog → Ready: Product Owner grooms and prioritises
Ready → In Progress: Developer assigns self and starts work
In Progress → In Review: PR submitted, link PR in task
In Review → Done: PR approved and merged
```

## Examples

### Sprint Planning Output

```
Sprint 4: Data Drain and Status Effects

Sprint Goal: Implement the Data Drain mechanic and status effect system

Committed Items:
- [5] Data Drain component and resource setup
- [8] Data Drain minigame system
- [3] Status effect component and tick system
- [5] Poison, Burn, and Freeze effects
- [2] Status effect UI indicators

Total: 23 points
```

### Daily Standup Example

```
@alice
- Yesterday: Completed RED phase for Data Drain minigame test
- Today: GREEN phase — implement the minigame system
- Blockers: None

@bob
- Yesterday: Merged PR for poison status effect
- Today: Start work on Burn effect (REFACTOR phase)
- Blockers: Need clarification on Burn vs Fire damage stacking
```

### Retrospective Action Items

```
Action Items for Sprint 5:
1. [Start] Add a "TDD phase" label to all tasks (@alice)
2. [Stop] Merging without benchmark comparison (@bob)
3. [Continue] Daily standup async option for remote days (@charlie)
```

## Related Skills
- [`tdd-cycle.md`](tdd-cycle.md) — TDD workflow for implementing sprint items
- [`git-workflow.md`](git-workflow.md) — PR process and code review guidelines
- [`../docs/GDD.md`](../docs/GDD.md) — Game design reference for feature planning
- [`../docs/TECHNICAL_DESIGN.md`](../docs/TECHNICAL_DESIGN.md) — Technical architecture reference