# Data Model: Train Track and Player Character

**Feature**: Train Track and Player Character  
**Date**: 2025-01-27  
**Phase**: 1 - Design

## Overview

This game uses Bevy's ECS architecture, so the "data model" consists of Entities, Components, and Resources rather than traditional data structures. This document describes the ECS structure.

## Entities

### Player Character Entity
**Purpose**: Represents the player's avatar in the game world.

**Components**:
- `Transform` (Bevy built-in): Position, rotation, scale in 3D space
- `Player`: Marker component identifying player entity
- `CurrentTrack`: Enum { Left(0), Middle(1), Right(2) } - which track player is on
- `Velocity`: Forward velocity vector (magnitude increases over time per FR-027)
- `AnimationState`: Enum { Running, Sliding, Jumping } - current animation state
- `Mesh` + `Material`: Visual representation (composed of cubes, cylinders, circles per FR-003)

**State Transitions**:
- Running → Sliding: On down arrow/S key press
- Running → Jumping: On spacebar press
- Sliding → Running: After slide animation completes
- Jumping → Running: After jump animation completes
- Any state → GameOver: On collision with barricade

### Train Track Entity
**Purpose**: Represents a segment of one of the three parallel tracks.

**Components**:
- `Transform`: Position in 3D space
- `TrackSegment`: Contains track_index (0, 1, 2), segment_id for pooling
- `Mesh` + `Material`: Visual representation of track segment

**Lifecycle**:
- Created: When new segment needed ahead of character
- Recycled: When segment moves behind camera (per FR-016)
- Destroyed: Only if pool exceeds maximum size (rare)

### Barricade Entity
**Purpose**: Obstacle that blocks character's path on a track.

**Components**:
- `Transform`: Position in 3D space
- `Barricade`: Contains track_index (0, 1, 2), spawn_time for difficulty calculation
- `CollisionShape`: Bounding box for collision detection
- `Mesh` + `Material`: Visual representation

**Lifecycle**:
- Created: Procedurally spawned ahead of character (per FR-028, FR-029)
- Destroyed: When moves behind camera or character passes it

### Prop Entity
**Purpose**: Environmental decoration around tracks.

**Components**:
- `Transform`: Position in 3D space
- `Prop`: Contains prop_type enum (minimum 3 types per SC-005)
- `Mesh` + `Material`: Visual representation

**Lifecycle**:
- Created: Spawned around tracks for environmental context
- Destroyed: When moves behind camera

## Resources

### GameConfig
**Purpose**: Stores game configuration parameters.

**Fields**:
- `base_speed: f32` - Initial forward running speed
- `speed_acceleration_rate: f32` - Rate at which speed increases over time
- `current_speed: f32` - Current forward speed (updated over time)
- `barricade_spawn_base_interval: f32` - Base time between barricade spawns
- `difficulty_scale: f32` - Multiplier for barricade frequency (increases over time)
- `track_spacing: f32` - Distance between parallel tracks
- `barricade_advance_time: f32` - Minimum seconds ahead for barricade spawn (2.0 per SC-008)

### TrackPool
**Purpose**: Manages reusable track segment entities for memory efficiency.

**Fields**:
- `available_segments: Vec<Entity>` - Pool of unused track segment entities
- `active_segments: Vec<Entity>` - Currently active track segments
- `max_pool_size: usize` - Maximum pool size to prevent unbounded growth

### GameState
**Purpose**: Tracks current game state for state machine.

**Values**:
- `Playing` - Normal gameplay state
- `GameOver` - Game over screen displayed, waiting for restart

## Relationships

- **Player to Track**: Player has `CurrentTrack` component indicating which track (0, 1, or 2) they're on
- **Barricade to Track**: Each barricade has `track_index` indicating which track it's on (per FR-024)
- **Track Segments**: Multiple track segment entities form continuous infinite tracks (per FR-008)

## Validation Rules

- `CurrentTrack` must be 0, 1, or 2 (three tracks only)
- Barricade `track_index` must be 0, 1, or 2 (per FR-022)
- Character cannot move beyond track boundaries (per FR-011)
- Track segments must be recycled when behind camera (per FR-016)
- Barricades must spawn at least 2 seconds ahead at current speed (per SC-008)

## State Management

Game state transitions:
- `Playing` → `GameOver`: On collision detection (per FR-025)
- `GameOver` → `Playing`: On restart action (per FR-026)

Player animation state transitions:
- Controlled by input system and animation duration
- Track switching prevented during Sliding/Jumping (per FR-021)

