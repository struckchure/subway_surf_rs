# Contracts: Train Track and Player Character

**Feature**: Train Track and Player Character  
**Date**: 2025-01-27

## Note

This is a single-player game application, not a service with API endpoints. Traditional API contracts (REST, GraphQL) do not apply.

Instead, this document defines the **internal contracts** between game systems:

## System Contracts

### Input System → Player Movement System

**Contract**: Input system processes keyboard events and updates player intent.

**Input Events**:
- `KeyCode::Left` or `KeyCode::A` → Move left track
- `KeyCode::Right` or `KeyCode::D` → Move right track  
- `KeyCode::Space` → Jump action
- `KeyCode::Down` or `KeyCode::S` → Slide action

**Output**: Player intent flags (move_left, move_right, jump, slide) set on player entity or resource.

### Player Movement System → Animation System

**Contract**: Movement system triggers animation state changes.

**State Transitions**:
- Jump input → AnimationState::Jumping (if not already in animation)
- Slide input → AnimationState::Sliding (if not already in animation)
- Animation complete → AnimationState::Running

**Constraints**: Track switching prevented during Sliding/Jumping (FR-021).

### Collision System → Game State System

**Contract**: Collision system detects character-barricade collisions and triggers game over.

**Input**: Character position, barricade positions, collision shapes.

**Output**: GameState transition: Playing → GameOver (FR-025).

### Track Generation System → Track Pool

**Contract**: Track generation system requests and recycles track segments.

**Operations**:
- `request_segment(track_index: u8) -> Entity` - Get segment from pool or create new
- `recycle_segment(entity: Entity)` - Return segment to pool when behind camera

**Constraint**: Pool size must remain bounded (FR-016, SC-010).

### Barricade Spawning System → Game Config

**Contract**: Barricade spawning system reads difficulty parameters and spawns barricades.

**Input**: Current game time, current speed, difficulty_scale from GameConfig.

**Output**: New barricade entities spawned on tracks with spacing rules.

**Rules**:
- Minimum 2 seconds ahead at current speed (SC-008)
- Frequency increases with difficulty_scale (FR-029)
- Each barricade associated with specific track (FR-024)

## Data Flow Contracts

### Speed Acceleration Contract

**System**: Game update loop → GameConfig

**Behavior**: Each frame, update `current_speed` based on `speed_acceleration_rate` and elapsed time.

**Formula**: `current_speed = base_speed + (speed_acceleration_rate * elapsed_time)`

**Constraint**: Speed must increase gradually (FR-027).

### Track Recycling Contract

**System**: Camera/Viewport → Track Generation System

**Behavior**: When track segment moves behind camera viewport, recycle it to pool.

**Constraint**: Must prevent unbounded memory growth (FR-016).

## Interface Contracts

### Keyboard Input Interface

**Provider**: Bevy's `KeyboardInput` events

**Consumer**: Input system

**Events Handled**:
- Key press events for movement (left, right)
- Key press events for actions (space, down/S)

### Rendering Interface

**Provider**: Bevy's rendering system

**Consumer**: All visual entities (player, tracks, barricades, props)

**Requirements**: All entities with `Mesh` and `Material` components are rendered.

**Performance**: Must maintain 30+ FPS (SC-002).

