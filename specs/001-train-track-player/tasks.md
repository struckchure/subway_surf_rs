# Tasks: Train Track and Player Character

**Input**: Design documents from `/specs/001-train-track-player/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), data-model.md, research.md

**Organization**: Tasks are grouped by user story to enable independent implementation of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create project structure per implementation plan in src/
- [x] T002 Initialize Rust project with Bevy 0.17.3 dependency in Cargo.toml
- [x] T003 [P] Create src/main.rs with basic Bevy app setup
- [x] T004 [P] Create src/game/mod.rs module structure
- [x] T005 [P] Create src/resources/mod.rs module structure

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Create GameState resource enum (Playing, GameOver) in src/game/game_state.rs
- [x] T007 Create GameConfig resource with base_speed, speed_acceleration_rate, track_spacing, barricade_advance_time in src/resources/game_config.rs
- [x] T008 Create TrackPool resource with available_segments and active_segments in src/resources/track_pool.rs
- [x] T009 [P] Create camera setup system in src/game/camera.rs
- [x] T010 [P] Create input handling system for keyboard events in src/game/input.rs
- [x] T011 Create ECS systems module structure in src/game/systems.rs
- [x] T012 Register all systems, resources, and states in src/main.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Visual Game Scene Setup (Priority: P1) 🎯 MVP

**Goal**: Display three parallel train tracks with environmental props that extend infinitely forward

**Verification**: Launch game and observe three parallel train tracks rendered in viewport with props visible around them. Tracks should be clearly identifiable, parallel, and extend continuously forward.

### Implementation for User Story 1

- [x] T013 [P] [US1] Create TrackSegment component with track_index and segment_id in src/game/track.rs
- [x] T014 [P] [US1] Create Prop component with prop_type enum in src/game/props.rs
- [x] T015 [US1] Implement track segment generation system in src/game/track.rs
- [x] T016 [US1] Implement track segment recycling system for infinite extension in src/game/track.rs
- [x] T017 [US1] Implement three parallel track rendering system in src/game/systems.rs
- [x] T018 [US1] Implement props generation and rendering system (minimum 3 distinct types) in src/game/props.rs
- [x] T019 [US1] Integrate track and props systems into main app in src/main.rs

**Checkpoint**: At this point, User Story 1 should be fully functional - three tracks and props visible, extending infinitely

---

## Phase 4: User Story 2 - Player Character Representation (Priority: P1)

**Goal**: Display a character composed of cubes, cylinders, and circles positioned on middle track, running continuously forward

**Verification**: Observe character made of geometric shapes visible on middle track, moving continuously forward along track.

### Implementation for User Story 2

- [x] T020 [P] [US2] Create Player marker component in src/game/player.rs
- [x] T021 [P] [US2] Create CurrentTrack component enum (Left, Middle, Right) in src/game/player.rs
- [x] T022 [P] [US2] Create Velocity component for forward movement in src/game/player.rs
- [x] T023 [P] [US2] Create AnimationState component enum (Running, Sliding, Jumping) in src/game/player.rs
- [x] T024 [US2] Implement player character mesh creation (cubes, cylinders, circles) in src/game/player.rs
- [x] T025 [US2] Implement player spawn system (middle track, initial position) in src/game/systems.rs
- [x] T026 [US2] Implement continuous forward movement system using Velocity component in src/game/systems.rs
- [x] T027 [US2] Implement speed acceleration system updating GameConfig.current_speed over time in src/game/systems.rs
- [x] T028 [US2] Integrate player systems into main app in src/main.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both be functional - tracks, props, and character visible with continuous forward movement

---

## Phase 5: User Story 3 - Track Switching (Priority: P1)

**Goal**: Allow character to switch between three parallel tracks using left/right keyboard input

**Verification**: Press left/right arrow keys (or A/D) and observe character smoothly transitions between adjacent tracks within 0.5 seconds, maintaining forward movement.

### Implementation for User Story 3

- [x] T029 [US3] Implement track switching input handling (left/right arrow keys or A/D keys) in src/game/input.rs
- [x] T030 [US3] Implement track switching movement system with smooth interpolation in src/game/systems.rs
- [x] T031 [US3] Implement track boundary prevention (leftmost/rightmost limits) in src/game/systems.rs
- [x] T032 [US3] Ensure forward movement maintained during track switch transition in src/game/systems.rs
- [x] T033 [US3] Integrate track switching into main app systems in src/main.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all be functional - character can switch tracks smoothly

---

## Phase 6: User Story 4 - Barricade Obstacles and Navigation (Priority: P1)

**Goal**: Display barricades on tracks that character can avoid by sliding under or jumping over, with collision detection and game over

**Verification**: Observe barricades appear on tracks ahead of character (2+ seconds ahead). Character can slide (down/S) or jump (spacebar) to avoid. Collision triggers game over screen with restart option.

### Implementation for User Story 4

- [x] T034 [P] [US4] Create Barricade component with track_index and spawn_time in src/game/barricade.rs
- [x] T035 [P] [US4] Create CollisionShape component for bounding box detection in src/game/barricade.rs
- [x] T036 [US4] Implement procedural barricade generation system with spacing rules in src/game/barricade.rs
- [x] T037 [US4] Implement difficulty scaling system increasing barricade frequency over time in src/game/barricade.rs
- [x] T038 [US4] Implement barricade spawning system (minimum 2 seconds ahead at current speed) in src/game/systems.rs
- [x] T039 [US4] Implement slide input handling (down arrow or S key) in src/game/input.rs
- [x] T040 [US4] Implement jump input handling (spacebar) in src/game/input.rs
- [x] T041 [US4] Implement slide animation system updating AnimationState in src/game/systems.rs
- [x] T042 [US4] Implement jump animation system updating AnimationState in src/game/systems.rs
- [x] T043 [US4] Implement collision detection system between character and barricades in src/game/systems.rs
- [x] T044 [US4] Implement game over state transition on collision in src/game/systems.rs
- [x] T045 [US4] Implement game over screen UI with restart option in src/game/game_state.rs
- [x] T046 [US4] Implement restart functionality transitioning GameOver → Playing in src/game/systems.rs
- [x] T047 [US4] Implement track switch prevention/queuing during slide/jump animations in src/game/systems.rs
- [x] T048 [US4] Ensure slide/jump actions work even without barricades (responsive controls) in src/game/systems.rs
- [x] T049 [US4] Integrate all barricade and navigation systems into main app in src/main.rs

**Checkpoint**: At this point, all user stories should be functional - complete gameplay loop with obstacles, navigation, and game over

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T050 [P] Implement viewport resize handling maintaining visibility and positioning in src/game/systems.rs
- [x] T051 [P] Implement visual clarity system for overlapping elements in src/game/systems.rs
- [x] T052 [P] Implement rendering priority system (tracks/character over props when resources limited) in src/game/systems.rs
- [x] T053 [P] Implement aspect ratio handling ensuring core elements remain visible in src/game/camera.rs
- [x] T054 Code cleanup and refactoring across all modules
- [x] T055 Performance optimization to maintain 30+ FPS target
- [x] T056 Run quickstart.md validation checklist

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can proceed sequentially (US1 → US2 → US3 → US4) for MVP
  - Or in parallel if team capacity allows (after foundational)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Depends on US1 for track positioning
- **User Story 3 (P1)**: Can start after Foundational (Phase 2) - Depends on US2 for character entity
- **User Story 4 (P1)**: Can start after Foundational (Phase 2) - Depends on US2 (character) and US3 (track switching)

### Within Each User Story

- Components before systems
- Systems before integration
- Core implementation before edge cases
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T003, T004, T005)
- All Foundational tasks marked [P] can run in parallel (T009, T010)
- Component creation tasks within a story marked [P] can run in parallel
- Different user stories can be worked on sequentially (recommended for MVP) or in parallel by different team members

---

## Parallel Example: User Story 2

```bash
# Launch all component creation tasks for User Story 2 together:
Task: "Create Player marker component in src/game/player.rs"
Task: "Create CurrentTrack component enum in src/game/player.rs"
Task: "Create Velocity component in src/game/player.rs"
Task: "Create AnimationState component enum in src/game/player.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Verify three tracks and props are visible and extend infinitely
5. Demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Verify tracks/props → Demo (MVP!)
3. Add User Story 2 → Verify character running → Demo
4. Add User Story 3 → Verify track switching → Demo
5. Add User Story 4 → Verify complete gameplay → Demo
6. Each story adds value without breaking previous stories

### Sequential Strategy (Recommended for MVP)

With single developer or focused MVP:

1. Complete Setup + Foundational together
2. Implement User Story 1 → Validate
3. Implement User Story 2 → Validate
4. Implement User Story 3 → Validate
5. Implement User Story 4 → Validate
6. Polish and optimize

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability (US1, US2, US3, US4)
- Each user story should be independently completable
- Commit after each task or logical group
- Stop at any checkpoint to validate story functionality
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- All tasks follow strict format: `- [ ] T### [P?] [US?] Description with file path`

