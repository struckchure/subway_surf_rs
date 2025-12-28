# Subway Surf RS 🚇🏃

A Subway Surfers-inspired endless runner game built with Rust and Bevy game engine.

## About

Subway Surf RS is a 3D endless runner game where you control a character running through subway tracks, avoiding obstacles, collecting coins, and dodging trains. Built using the Bevy game engine and Avian physics engine, this project showcases modern Rust game development practices.

## Features

- 🎮 Smooth 3D gameplay with physics-based movement
- 🏃 Character animations and limb movements
- 🚂 Procedurally generated trains and obstacles
- 💰 Coin collection system with score tracking
- 🎯 Dynamic difficulty scaling
- 🎨 Track switching mechanics
- ⚡ Optimized performance with multi-threaded ECS architecture

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

## Building

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

## Running

### Run in Debug Mode

```bash
cargo run
```

### Run in Release Mode

```bash
cargo run --release
```

## Controls

- **Arrow Keys / WASD**: Move left/right
- **Space / Up Arrow**: Jump
- **Down Arrow**: Slide
- **R**: Restart (when game over)

## Project Structure

```
subway_surf_rs/
├── src/
│   ├── game/           # Game logic modules
│   │   ├── barricade.rs    # Obstacle generation
│   │   ├── camera.rs       # Camera systems
│   │   ├── coin.rs         # Coin collection
│   │   ├── game_state.rs   # Game state management
│   │   ├── input.rs        # Input handling
│   │   ├── player.rs       # Player entity
│   │   ├── props.rs        # Environmental props
│   │   ├── systems.rs      # Core game systems
│   │   ├── track.rs        # Track generation
│   │   └── train.rs        # Train obstacles
│   ├── resources/      # Game resources
│   └── main.rs         # Application entry point
├── assets/             # Game assets
└── Cargo.toml         # Project dependencies
```

## Dependencies

- **[Bevy](https://bevyengine.org/)** (v0.17.3) - A refreshingly simple data-driven game engine built in Rust
- **[Avian3D](https://github.com/Jondolf/avian)** (v0.4.1) - A 3D physics engine for Bevy

## Development

The project uses optimized build profiles for better performance:

- **Dev Profile**: Light optimization (opt-level = 1) for faster compilation
- **Release Profile**: Full optimization with LTO for production builds
- **WASM Release Profile**: Size-optimized builds for web deployment

## Releases

Releases are automatically created when a new version tag is pushed to the repository. Each release includes pre-built executables for Linux, Windows, and macOS platforms.

### Creating a Release

1. Update the `CHANGELOG.md` file with changes for the new version
2. Create and push a version tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. The GitHub Actions workflow will automatically build and create a release with downloadable executables

### Downloading Pre-built Binaries

Visit the [Releases](https://github.com/struckchure/subway_surf_rs/releases) page to download pre-built binaries for your platform.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## Acknowledgments

- Inspired by the popular mobile game Subway Surfers
- Built with the amazing Bevy game engine community
- Physics powered by Avian3D

---

Made with ❤️ and Rust 🦀
