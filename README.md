# Rust Game Engine

![Rust Logo](https://github.com/rust-lang/rust-artwork/blob/bf0b3272f9ba8d22f7fd45e408496d05621b3b5c/logo/rust-logo-128x128-blk-v2.png)

Welcome to the Rust Game Engine! This game engine is built in Rust and features a decoupled frontend layer utilizing HTML and JavaScript with WebGL capabilities. This README.md file will guide you through the project structure and usage.

## Project Structure

### `webgl/`

This directory contains JavaScript code responsible for the game's frontend. It leverages WebGL features for rendering the game world.

### `src/`

The heart of our game engine resides here, containing the core Rust codebase:

#### `src/scene.rs`

This module creates and manages the game environment. It communicates with the frontend layer to render the game and handles various game-related tasks.

#### `src/kinematics/`

This directory contains two important entities:

##### `src/kinematics/game_object.rs`

The `GameObject` module encapsulates the logic for game objects. This includes their behavior, interactions, and properties within the game world.

##### `src/kinematics/vector3.rs`

The `Vector3` module provides a versatile representation of vectorial quantities, such as positions and velocities. This is essential for physics calculations and game dynamics.

## Getting Started

To begin using the Rust Game Engine, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/rust-game-engine.git
   ```

2. Build the engine:

   ```
   cd rust-game-engine
   cargo build
   ```

3. Run the example:

   ```
   cargo run --example game
   ```

This will execute the example located in `/example/game.rs` and showcase how to use the game engine. Feel free to modify the example to create your own games!

## Dependencies

The Rust Game Engine relies on several external libraries and tools:

- [Rust](https://www.rust-lang.org/): The programming language used for developing the game engine.
- [WebGL](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API): The frontend leverages WebGL for rendering graphics in the browser.

Happy gaming! 🎮✨
