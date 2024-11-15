use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEventKind, KeyEvent, KeyModifiers};
use crate::input::{ InputMode, Page, InputContent, ListType };

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // todo: need to remove eventually***
    if key_event.code == KeyCode::Char('q') {
        app.quit();
    }
    match app.page {
        Page::Login => {
            match app.input_mode {
                InputMode::Normal => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.quit();
                    },
                    KeyCode::Char('e') => {
                        // insert username
                        app.input_content = InputContent::Username;
                        app.input_mode = InputMode::Editing;
                    },
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
                InputMode::Editing => {},
                InputMode::ViewAccountList => {}
            }
        },
        Page::AccountDetails => {
            match app.input_mode {
                InputMode::Normal => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.quit();
                    },
                    KeyCode::Up => app.prev_input(),
                    KeyCode::Down => app.next_input(),
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    },
                    KeyCode::Char('a') => {
                        // add new account
                        app.page = Page::NewAccount;
                        app.input_content = InputContent::AccountName;
                    },
                    KeyCode::Char('t') => {
                        // add new account
                        app.page = Page::NewTransaction;
                        app.input_content = InputContent::TransactionDescription;
                    },
                    KeyCode::Char('l') => {
                        app.list_content = ListType::Acct;
                        app.select_first();
                    },
                    KeyCode::Char('s') => {
                        app.list_content = ListType::Trans;
                        app.select_first();
                    },
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
                InputMode::Editing => {},
                InputMode::ViewAccountList if key_event.kind == KeyEventKind::Press => match key_event.code {
                    KeyCode::Up => app.select_prev(),
                    KeyCode::Down => app.select_next(),
                    KeyCode::Esc => app.stop_select(),
                    KeyCode::Enter => app.confirm_selection(),
                    _ => {}
                },
                InputMode::ViewAccountList => {}
            }
        },
        Page::NewAccount => {
            match app.input_mode {
                InputMode::Normal => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.quit();
                    },
                    KeyCode::Up => app.prev_input(),
                    KeyCode::Down => app.next_input(),
                    KeyCode::Char('b') => {
                        app.page = Page::AccountDetails;
                    },
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    },
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
                InputMode::Editing => {},
                InputMode::ViewAccountList => {}
            }
        },
        Page::NewTransaction | Page::EditTransaction => {
            match app.input_mode {
                InputMode::Normal => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.quit();
                    },
                    KeyCode::Char('b') => {
                        app.page = Page::AccountDetails;
                    },
                    KeyCode::Up => app.prev_input(),
                    KeyCode::Down => app.next_input(),
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    },
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
                InputMode::Editing => {},
                InputMode::ViewAccountList => {}
            }
        },
    }
    Ok(())
}
