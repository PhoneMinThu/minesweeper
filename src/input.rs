use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

/// Direction for cursor movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

/// High-level input actions translated from terminal events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    Move(Dir),
    Reveal,
    Flag,
    Chord,
    Restart,
    ChangeDifficulty,
    Quit,
}

/// Translate a crossterm Event into an optional InputAction.
///
/// Supported bindings:
/// - Movement: Arrow keys, WASD (W/A/S/d). Note: uppercase 'D' is reserved for ChangeDifficulty.
/// - Reveal: Enter or Space
/// - Flag: F/f
/// - Chord: C/c
/// - Restart: R/r
/// - ChangeDifficulty: D (uppercase)
/// - Quit: Q/q or Ctrl-C
pub fn translate_event(ev: Event) -> Option<InputAction> {
    match ev {
        Event::Key(KeyEvent { code, modifiers, .. }) => {
            // Handle Ctrl-C as Quit regardless of code case
            if modifiers.contains(KeyModifiers::CONTROL) {
                if matches!(code, KeyCode::Char('c') | KeyCode::Char('C')) {
                    return Some(InputAction::Quit);
                }
            }

            match code {
                // Movement via arrows
                KeyCode::Left => Some(InputAction::Move(Dir::Left)),
                KeyCode::Right => Some(InputAction::Move(Dir::Right)),
                KeyCode::Up => Some(InputAction::Move(Dir::Up)),
                KeyCode::Down => Some(InputAction::Move(Dir::Down)),

                // Reveal via Enter/Space
                KeyCode::Enter | KeyCode::Char(' ') => Some(InputAction::Reveal),

                // Chord
                KeyCode::Char('c') | KeyCode::Char('C') => Some(InputAction::Chord),

                // Flag
                KeyCode::Char('f') | KeyCode::Char('F') => Some(InputAction::Flag),

                // Restart
                KeyCode::Char('r') | KeyCode::Char('R') => Some(InputAction::Restart),

                // Change difficulty (upper-case D)
                KeyCode::Char('D') => Some(InputAction::ChangeDifficulty),

                // Quit
                KeyCode::Char('q') | KeyCode::Char('Q') => Some(InputAction::Quit),

                // Movement via WASD (lowercase/uppercase except 'D' uppercase)
                KeyCode::Char('a') | KeyCode::Char('A') => Some(InputAction::Move(Dir::Left)),
                KeyCode::Char('d') /* lowercase only */ => Some(InputAction::Move(Dir::Right)),
                KeyCode::Char('w') | KeyCode::Char('W') => Some(InputAction::Move(Dir::Up)),
                KeyCode::Char('s') | KeyCode::Char('S') => Some(InputAction::Move(Dir::Down)),

                _ => None,
            }
        }
        _ => None,
    }
}

