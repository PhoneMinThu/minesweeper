# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

# Minesweeper - Terminal TUI Game

A terminal-based Minesweeper implementation written in Rust using Ratatui for the TUI and Crossterm for input handling.

## Common Development Commands

### Build and Run
```bash
# Standard build
cargo build

# Optimized release build
cargo build --release

# Run the game
cargo run
```

### Testing
```bash
# Run tests (currently fails due to borrow checker issues in board.rs tests)
cargo test

# Run tests with verbose output
cargo test --verbose
```

**Note**: Tests currently have compilation errors due to borrow checker issues in `src/board.rs` (lines 343, 365, 378). These need to be fixed before tests can run.

### Formatting and Linting
```bash
# Check code formatting (uses rustfmt.toml config)
cargo fmt --check

# Auto-format code
cargo fmt

# Run clippy with pedantic lints (expect many warnings)
cargo clippy

# Run clippy with error-level warnings (as used in CI)
cargo clippy -- -D warnings
```

**Note**: The project uses strict linting with `#![deny(clippy::all, clippy::pedantic)]` which currently generates ~50 warnings that should be addressed.

### CI Commands
The GitHub Actions workflow runs:
1. `cargo fmt -- --check` (format validation)
2. `cargo clippy -- -D warnings` (lint with warnings as errors)
3. `cargo test --all --all-features --no-fail-fast` (test execution)

## High-Level Architecture

### Event-Driven TUI Design
The game follows a classic TUI event loop pattern:
1. **Input**: Poll for terminal events (keyboard) via Crossterm
2. **Translation**: Convert raw events to high-level `InputAction` enum values
3. **State Update**: Process actions through `AppState::handle_action()` 
4. **Rendering**: Draw the updated state using Ratatui widgets
5. **Repeat**: Continue until quit signal

### Module Responsibilities

- **`main.rs`**: Entry point with terminal setup, main event loop, and cleanup
- **`app.rs`**: Core game state (`AppState`), action handling, and game logic coordination
- **`board.rs`**: Minesweeper board implementation with cells, mine placement, and game rules
- **`difficulty.rs`**: Game difficulty levels (Easy/Medium/Hard) with board dimensions and mine counts
- **`input.rs`**: Keyboard event translation from Crossterm events to game actions
- **`ui.rs`**: Ratatui-based rendering of header, board, footer, and game-over overlays
- **`error.rs`**: Error types (minimal custom error handling)

### Key Game Mechanics

- **Lazy Mine Placement**: Mines are placed only after the first cell reveal to ensure first click safety
- **Flood Fill**: Revealing a zero-mine cell automatically reveals adjacent zero regions
- **Chording**: Middle-click equivalent - reveals neighbors when flag count matches the cell's number
- **Three Difficulty Levels**: Easy (9×9, 10 mines), Medium (16×16, 40 mines), Hard (30×16, 99 mines)

### Technology Stack

- **Ratatui**: TUI framework for layout, widgets, and rendering
- **Crossterm**: Cross-platform terminal manipulation and input handling
- **Rand**: Randomization for mine placement

## Development Configuration

### Language Settings

- **Rust Edition**: 2024 (latest)
- **Lint Level**: Strict `#![deny(clippy::all, clippy::pedantic)]` in main.rs
- **Binary + Library**: Both `src/main.rs` (executable) and `src/lib.rs` (library) targets

### Configuration Files

#### `rustfmt.toml`
```toml
edition = "2021"
max_width = 100
newline_style = "Unix"
use_field_init_shorthand = true
```

#### `clippy.toml` 
Currently minimal - allows for local rule overrides via `#[allow(...)]` attributes.

### Dependencies

#### Runtime Dependencies
- `anyhow` (1.0.98): Error handling and context
- `crossterm` (0.29.0): Terminal manipulation and input
- `rand` (0.9.2): Random number generation for mine placement
- `ratatui` (0.29.0): TUI framework for rendering
- `thiserror` (2.0.12): Derive macros for error types

#### Development Dependencies  
- `rstest` (0.26.1): Parameterized testing framework

## Current Issues to Address

### Test Compilation Failures
- **Borrow checker errors** in `src/board.rs` tests (lines 343, 365, 378)
- Issue: `cannot borrow \`b\` as immutable because it is also borrowed as mutable`
- Tests cannot run until these are resolved
- Suggested fix: Store `b.idx(x, y)` result in a local variable before using it

### Code Quality Issues
- **~50 Clippy warnings** from pedantic lints that should be addressed
- **Format deviations** detected by `cargo fmt --check` 
- These include:
  - Uninlined format arguments
  - Unnested or-patterns
  - Too many function arguments (>7)
  - Casting issues (possible wrap/truncation)
  - Documentation improvements needed

### Recommendations
- Address borrow checker issues in tests before merging substantial changes
- Run `cargo clippy --fix` to auto-fix simple lints
- Run `cargo fmt` to fix formatting issues
- Consider refactoring functions with too many parameters
- Add missing documentation backticks for better rustdoc output
