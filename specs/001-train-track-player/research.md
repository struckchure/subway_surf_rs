# Research: Train Track and Player Character

**Feature**: Train Track and Player Character  
**Date**: 2025-01-27  
**Phase**: 0 - Technical Research

## Technical Decisions

### Decision: Bevy ECS Architecture

**Decision**: Use Bevy's Entity Component System (ECS) architecture for game logic.

**Rationale**: 
- Bevy 0.17.3 provides built-in ECS which is ideal for game development
- ECS pattern separates data (components) from behavior (systems), improving maintainability (P1)
- Bevy's ECS is performant and well-suited for real-time game updates
- Simplifies entity management (player, tracks, barricades, props) as separate components

**Alternatives considered**:
- Object-oriented approach: Rejected - less flexible, harder to compose behaviors
- Data-oriented design without ECS: Rejected - Bevy's ECS provides better tooling and patterns

### Decision: Track Segment Pooling for Infinite Generation

**Decision**: Implement object pooling pattern for track segments to enable infinite generation with bounded memory.

**Rationale**:
- Required by FR-016 and SC-010 to prevent unbounded memory growth
- Pooling allows reusing track segments that have moved behind the camera
- Standard pattern in game development for infinite/procedural content
- Maintains simplicity (P2) while solving concrete memory constraint

**Alternatives considered**:
- Generate new segments indefinitely: Rejected - violates memory constraint
- Fixed-length track: Rejected - violates infinite progression requirement

### Decision: Procedural Barricade Generation with Difficulty Scaling

**Decision**: Use hybrid approach combining procedural generation with difficulty scaling for barricade placement.

**Rationale**:
- Clarified requirement: hybrid generation with difficulty scaling
- Procedural generation ensures variety and prevents repetition
- Difficulty scaling increases challenge as game progresses, matching accelerating speed
- Spacing rules ensure playability (minimum 2 seconds advance notice per SC-008)

**Alternatives considered**:
- Fixed patterns: Rejected - lacks variety, becomes predictable
- Pure random: Rejected - may create impossible sequences, no difficulty progression

### Decision: Keyboard Input Handling

**Decision**: Use Bevy's keyboard input system for player controls (arrow keys/WASD, space, down/S).

**Rationale**:
- Clarified requirement: keyboard input method
- Bevy provides built-in keyboard event handling
- Simple and direct for prototype (P2)
- Standard PC game input pattern

**Alternatives considered**:
- Gamepad support: Deferred - can be added later if needed
- Touch input: Not applicable - desktop target platform

### Decision: Game State Management

**Decision**: Implement game state system (Playing, GameOver) using Bevy's state management.

**Rationale**:
- Required for game over screen and restart functionality (FR-025, FR-026)
- Bevy's state system provides clean transitions between game states
- Simplifies UI management and game loop control
- Maintainable separation of concerns (P1)

**Alternatives considered**:
- Manual state flags: Rejected - Bevy's state system is more robust and idiomatic

### Decision: Camera Follow Pattern

**Decision**: Use fixed camera that follows character's forward progress (not character position).

**Rationale**:
- Character runs forward continuously, camera should follow progression
- Fixed relative position maintains consistent view
- Simpler than dynamic camera positioning (P2)
- Standard pattern for endless runner games

**Alternatives considered**:
- Static camera: Rejected - character would move out of view
- Character-relative camera: Considered but fixed forward-following is simpler for prototype

## Implementation Patterns

### Track Generation Pattern
- Generate initial track segments ahead of character
- As segments move behind camera, recycle them to front
- Maintain pool of reusable segment entities
- Procedurally vary segment appearance to avoid visible repetition

### Barricade Spawning Pattern
- Calculate spawn distance based on current speed (minimum 2 seconds ahead)
- Use random selection with spacing constraints
- Increase spawn frequency and complexity based on game progression time
- Associate each barricade with specific track to avoid ambiguity

### Character Movement Pattern
- Continuous forward movement via system that updates position each frame
- Track switching: smooth interpolation between track positions
- Slide/jump: animation states with duration, prevent track switching during animation
- Speed acceleration: gradually increase forward speed over time

## Bevy-Specific Considerations

### ECS Component Design
- Player: Transform, Velocity, CurrentTrack, AnimationState
- Track: Transform, TrackIndex (0, 1, 2), SegmentId
- Barricade: Transform, TrackIndex, CollisionShape
- Props: Transform, PropType

### System Organization
- Input system: reads keyboard events, updates player intent
- Movement system: applies forward velocity, handles track switching
- Animation system: manages slide/jump states
- Collision system: detects character-barricade collisions
- Track generation system: spawns/recycles track segments
- Barricade spawning system: generates barricades with difficulty scaling

### Resource Management
- GameConfig: stores speed, acceleration rate, difficulty parameters
- TrackPool: manages reusable track segment entities
- GameState: tracks current game state (Playing, GameOver)

## Performance Considerations

- Target: 30+ FPS minimum, 60 FPS preferred
- Track recycling prevents memory growth
- Limit active barricades/props in viewport
- Use Bevy's built-in culling for off-screen entities
- Batch rendering where possible via Bevy's render pipeline

## Unresolved Items

None - all technical decisions made based on spec clarifications and Bevy best practices.

