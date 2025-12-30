# Code Typing

A terminal-based typing game designed to help programmers improve their coding speed and accuracy. Practice typing real Rust code snippets while tracking your performance.

## Why Code Typing?

Traditional typing tests use regular English text, but programming requires typing special characters like `{}`, `=>`, `::`, and proper indentation. **Code Typing** bridges this gap by letting you practice with actual code, helping you:

- Build muscle memory for programming syntax
- Get faster at typing brackets, operators, and symbols
- Improve accuracy with code-specific patterns
- Track your progress with WPM and accuracy metrics

## Quick Start

```bash
# Clone and run
git clone <repo-url>
cd code-typing
cargo run
```

That's it! Start typing the code you see on screen.

## Gameplay

When you launch the game, you'll see a Rust code snippet. Simply start typing to begin - the timer starts on your first keystroke.

- **Green** characters = correct
- **Red** characters = incorrect  
- **Yellow cursor** = current position

The progress bar at the bottom shows how much you've completed. When finished, press `Enter` for a new snippet or `Esc` to quit.

## Controls

| Key | Action |
|-----|--------|
| `Tab` | Insert indentation |
| `Backspace` | Delete last character |
| `Enter` | Next snippet (after completing) |
| `Esc` | Quit |

## Requirements

- Rust 1.70+
- A terminal that supports Unicode

## Building from Source

```bash
cargo build --release
./target/release/code-typing
```

## License

MIT
