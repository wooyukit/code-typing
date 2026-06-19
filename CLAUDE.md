# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`code-typing` is a terminal (TUI) typing game for practicing programming syntax: the player picks a language from a startup menu, then types curated snippets while WPM and accuracy are tracked in real time. Ten languages ship today (Rust, Python, JavaScript, TypeScript, Go, Java, Kotlin, Swift, C, C++); Rust has ~100 snippets and the others a curated starter set each. Built on `ratatui` (rendering) and `crossterm` (terminal control + input). Published to crates.io as `code-typing`.

## Commands

```bash
cargo run                 # Build + launch the game (takes over the terminal: raw mode + alternate screen)
cargo build               # Debug build — use this for iteration
cargo build --release     # Release is heavily optimized (lto + codegen-units=1 + strip) and slow; avoid for quick checks
cargo test                # Unit + headless render tests (see below)
cargo test one_token      # Run a single test by name substring
cargo clippy --all-targets  # Lint (kept warning-clean)
cargo fmt                 # Format
cargo install --path .    # Install the binary locally
```

`cargo run` enters raw mode and the alternate screen, so it needs an interactive terminal; `main.rs` installs a panic hook that restores the terminal on crash. The UI itself is tested **headlessly** via `ratatui`'s `TestBackend` (no tty needed) — `ui::tests` renders the menu and every language's typing/completion screen and asserts `draw` never panics.

## Architecture

A data-driven, model-view-controller TUI event loop split across `src/app/`. Language support is **config-driven**: a `LanguageSpec` (in `language.rs`) feeds both the highlighter and the game logic, so adding a language is almost entirely data.

- **`main.rs`** — terminal lifecycle only (enable raw mode, enter alternate screen, panic hook to restore, construct `Game`, run, tear down).
- **`game.rs` (`Game`)** — the controller. Owns `GameState` + the ratatui `Terminal`. `run()` is the loop: `ui::draw` → poll input on a 50ms tick → `handle_input`. Input is **routed by `GameState.screen`**: `handle_menu_input` (↑↓/jk navigate, Enter select, Esc quit) vs. `handle_typing_input`. The quit-confirmation intercept runs first, before either.
- **`state.rs` (`GameState`)** — the model. Holds the active `Language`, the `Screen` (`Menu`/`Typing`), the `menu_index`, and all typing stats. The view is a pure function of this struct. Sample loading is centralized in `load_sample`/`load_random_sample`.
- **`language.rs`** — the heart of multi-language support. `Language` enum + `ALL` (menu order) + `LanguageSpec` (display `name`/`emoji`, keyword/type tables, comment delimiters, single-quote/backtick/lifetime/macro/attribute/preprocessor/decorator flags, `indent_spaces`, and the language's `samples`). `Language::spec()` returns a `&'static LanguageSpec`.
- **`ui.rs`** — the view. `draw` dispatches to `draw_menu` or `draw_typing`. Holds no state. Typing screen uses the original ornate layout: rainbow wordmark title, a `📝 Code · <language>` panel, three emoji stat cards (`render_stat_card`), an emoji completion bar, and a `⚠ QUIT? ⚠` popup.
- **`syntax.rs`** — a hand-written, **spec-driven** tokenizer used only for coloring. `highlight(code, &LanguageSpec)` returns one `TokenType` per char.
- **`samples/`** — one module per language (`rust.rs`, `python.rs`, …), each exposing `pub const SAMPLES: &[(&str, &str)]` = `(code, expected_output)`; pure data.

### Core invariant: character-index alignment

Three parallel structures are indexed by the same character position and must stay in lockstep:
- `GameState.current_code_chars` — target code as `Vec<char>`
- `GameState.user_input_chars` — what the user typed, as `Vec<char>`
- the `Vec<TokenType>` from `syntax::highlight(&current_code, language.spec())` — one token per code char

`GameState` deliberately keeps both a `String` and a pre-computed `Vec<char>` for code and input so per-keystroke comparison and per-render coloring are O(1) by index (a Rust `String` can't be indexed by char position). Preserve this alignment when touching input handling or rendering. In `ui.rs` the render loop walks `current_code.lines()` and increments a `char_index` once per character **and once more per line** to account for the stripped `\n` — that bookkeeping is what keeps the cursor and coloring aligned with `user_input_chars`. The test `syntax::tests::one_token_per_char_for_all_samples` guards the highlighter side across every sample in every language.

### Input model (state.rs)

- The timer starts on the first keystroke (`first_input_time`), not at launch.
- **Tab** expands to `language.spec().indent_spaces` spaces (per-language; e.g. 4 for Rust/Python, 2 for JS/TS); **Enter** auto-indents (newline + the leading whitespace of the next target line) and counts as a single keystroke that is "correct" only if every inserted char matches.
- `update_stats()` recomputes correctness from scratch each call (zips input vs. code chars), so backspace self-corrects the count. WPM = (chars / 5) / minutes; accuracy divides by input length so it never exceeds 100%.
- Sample selection (`random_sample`, `reset`, `restart_current`) operates within the active language and avoids immediately repeating the current sample. `select_menu_language` switches language and loads a fresh sample; `open_menu` returns to the picker (bound to ↑↓ before typing starts or after completion).

### Coloring (syntax.rs + language.rs + ui.rs)

`syntax::highlight` classifies each char from the spec: comments (`line_comment`/`block_comment`), strings (`"`, plus `` ` `` when `backtick_string`, and `'` per `single_quote`), numbers, and identifiers → keyword (`spec.keywords`) / type (`spec.types` or any uppercase-initial name) / function (followed by `(`, or after `::`). Language-specific quirks are gated by flags: Rust `lifetimes`/`macros`/`rust_attributes`, C/C++ `preprocessor` (`#include`), and `decorators` (`@…`). `ui.rs` styles each code char by state: correctly typed → its syntax color, wrong → red + underline, cursor position → yellow background, untyped → dimmed. On completion it shows an output panel (only if `expected_output` is non-empty) and a rating (LEGENDARY / EXCELLENT / GOOD / COMPLETE) gated on WPM + accuracy thresholds.

## Adding a code sample

Append a `(code, expected_output)` tuple to the relevant `SAMPLES` in `src/app/samples/<language>.rs`. Use `\n` for newlines (and `\\n`, `\"` etc. when those characters must appear *inside* the snippet's own strings). Indent to match that language's `indent_spaces`. Set `expected_output` to `""` unless the output is trivially certain — empty output suppresses the output panel on completion (most non-Rust samples ship with `""` because their output isn't verified here).

## Adding a language

1. Create `src/app/samples/<lang>.rs` with `pub const SAMPLES` and register it in `samples/mod.rs`.
2. Add a `LanguageSpec` (`name`, `emoji`, keyword/type tables + rule flags + `indent_spaces` + `samples`) in `language.rs`.
3. Add the `Language` enum variant, its `spec()` arm, and an entry in `ALL` (which sets menu order).

No highlighter or UI changes are needed — both are driven by the spec.
