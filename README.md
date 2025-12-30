# Code Typing

A terminal-based code typing game built in Rust using [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI.

## Features

- **Interactive Terminal UI** - Built with Ratatui for a modern terminal experience
- **Code Snippets** - Type real Rust code samples to test your typing speed
- **Real-time Feedback** - See character-by-character validation with color coding
- **Performance Metrics** - Track your WPM (Words Per Minute) and accuracy
- **Smooth Gameplay** - Event-driven architecture with responsive input handling

## Building & Running

### Prerequisites
- Rust 1.70+
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
```

## How to Play

1. Run the game with `cargo run`
2. You'll see a piece of Rust code in the "Code to Type" section
3. Start typing the code - characters will turn **green** for correct and **red** for incorrect
4. When you've typed the entire code sample, the game ends automatically
5. Your **WPM** and **Accuracy** will be displayed
6. Press **ENTER** to play another round or **ESC** to quit

## Controls

- **Type** - Input characters
- **Backspace** - Delete the last character
- **ENTER** - Start a new game (after finishing)
- **ESC** - Quit the game

## Project Structure

```
.
├── Cargo.toml                  # Project dependencies
├── Cargo.lock                  # Dependency lock file
├── README.md                   # This file
└── src/
    ├── main.rs                 # Application entry point
    └── app/
        ├── mod.rs              # Module definitions and public exports
        ├── state.rs            # GameState struct with game logic
        ├── ui.rs               # UI rendering and display logic
        └── game.rs             # Game orchestration and event loop
```

### Module Overview

- **main.rs** - Sets up the terminal, initializes the game, and handles cleanup
- **app/state.rs** - Contains `GameState` with all game mechanics (input handling, stats calculation)
- **app/ui.rs** - Handles all rendering logic with the `draw()` function
- **app/game.rs** - Orchestrates the game loop and input event handling
- **app/mod.rs** - Organizes module exports

## Dependencies

- **ratatui** (0.28) - Terminal UI framework
- **crossterm** (0.28) - Cross-platform terminal manipulation
- **tokio** - Async runtime (for future extensions)
- **serde** - Serialization framework (for future score saving)
- **rand** - Random number generation for code sample selection

## Architecture

The project follows a modular architecture pattern:

1. **State Management** - `GameState` handles all game logic independently
2. **UI Rendering** - `ui` module handles all visual presentation
3. **Game Loop** - `Game` struct orchestrates the event loop and state updates
4. **Clean Separation** - Terminal setup is isolated in `main.rs`

This design makes the code maintainable, testable, and easy to extend with new features.

## Future Enhancements

- Score tracking and leaderboards
- Custom code snippet sets
- Difficulty levels
- Sound effects and feedback
- Multiplayer mode
