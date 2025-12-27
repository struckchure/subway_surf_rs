# Quickstart Guide: Train Track and Player Character

**Feature**: Train Track and Player Character  
**Date**: 2025-01-27

## Prerequisites

- Rust 1.75+ installed
- Cargo (Rust package manager)
- Bevy 0.17.3 dependency (specified in Cargo.toml)

## Running the Game

```bash
# From repository root
cargo run --release
```

## Controls

- **Left/Right Movement**: Arrow keys (← →) or A/D keys
- **Jump**: Spacebar
- **Slide**: Down arrow (↓) or S key
- **Restart** (on game over): Spacebar or Enter

## Gameplay

1. **Start**: Character begins on the middle track, running forward automatically
2. **Avoid Obstacles**: Use left/right to switch tracks, space to jump, or down/S to slide under barricades
3. **Speed Increases**: Character speed gradually increases over time, increasing difficulty
4. **Game Over**: Colliding with a barricade stops gameplay and shows game over screen
5. **Restart**: Press spacebar or enter on game over screen to start a new game

## Verification Checklist

After running the game, verify:

- [ ] Three parallel train tracks are visible
- [ ] Character (made of cubes, cylinders, circles) is visible on middle track
- [ ] Character runs forward continuously without input
- [ ] Left/right arrow keys (or A/D) switch tracks
- [ ] Spacebar makes character jump
- [ ] Down arrow (or S) makes character slide
- [ ] Barricades appear on tracks ahead of character
- [ ] Colliding with barricade shows game over screen
- [ ] Game over screen has restart option
- [ ] Character speed increases over time
- [ ] Tracks extend infinitely forward
- [ ] Memory usage remains bounded during extended play

## Expected Behavior

- **Track Switching**: Smooth transition between tracks within 0.5 seconds (SC-006)
- **Barricade Timing**: Barricades appear at least 2 seconds ahead at current speed (SC-008)
- **Performance**: Minimum 30 FPS, target 60 FPS (SC-002)
- **Infinite Progression**: Game supports at least 5 minutes of continuous play without degradation (SC-009)

## Troubleshooting

- **Low FPS**: Check system requirements; reduce prop density if needed (FR-019)
- **Character not visible**: Verify character is on one of the three tracks
- **Input not responding**: Ensure game window has focus
- **Memory growth**: Verify track segment recycling is working (FR-016)

## Development Mode

For development with debug information:

```bash
cargo run
```

Release mode (optimized) is recommended for performance testing:

```bash
cargo run --release
```

