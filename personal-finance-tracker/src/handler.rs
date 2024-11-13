use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEventKind, KeyEvent, KeyModifiers};
use crate::input::InputMode;

// pub mod input;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
                // Exit application on `ESC` or `q`
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                },
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                },
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Editing;
                },
                // Counter handlers
                KeyCode::Right => {
                    app.increment_counter();
                },
                KeyCode::Left => {
                    app.decrement_counter();
                },
                // Other handlers you could add here.
                _ => {}
        },
        InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => app.submit_message(),
            KeyCode::Char(to_insert) => app.enter_char(to_insert),
            KeyCode::Backspace => app.delete_char(),
            KeyCode::Left => app.move_cursor_left(),
            KeyCode::Right => app.move_cursor_right(),
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
        },
        InputMode::Editing => {}
    }
    // match key_event.code {
    //     // Exit application on `ESC` or `q`
    //     KeyCode::Esc | KeyCode::Char('q') => {
    //         app.quit();
    //     }
    //     // Exit application on `Ctrl-C`
    //     KeyCode::Char('c') | KeyCode::Char('C') => {
    //         if key_event.modifiers == KeyModifiers::CONTROL {
    //             app.quit();
    //         }
    //     }
    //     // Counter handlers
    //     KeyCode::Right => {
    //         app.increment_counter();
    //     }
    //     KeyCode::Left => {
    //         app.decrement_counter();
    //     }
    //     // Other handlers you could add here.
    //     _ => {}
    // }
    Ok(())
}
