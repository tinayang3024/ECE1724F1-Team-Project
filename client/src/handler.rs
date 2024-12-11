use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEventKind, KeyEvent};
use crate::input::{ 
    InputMode, 
    Page, 
    InputContent,
    ListType,
};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
                    KeyCode::Enter => app.submit_message().await,
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
                    KeyCode::Up => {
                        if app.new_account.acct_id != "" {
                            app.prev_input()
                        }
                    },
                    KeyCode::Down => {
                        if app.new_account.acct_id != "" {
                            app.next_input()
                        }
                    },
                    KeyCode::Char('b') => {
                        app.delete_user().await;
                        app.page = Page::Login;
                    },
                    KeyCode::Char('e') => {
                        if app.new_account.acct_id != "" {
                            app.input_mode = InputMode::Editing;
                        }
                    },
                    KeyCode::Char('a') => {
                        // add new account
                        app.page = Page::NewAccount;
                        app.new_account.acct_id = "".to_string();
                        app.new_account.acct_name = "".to_string();
                        app.new_account.acct_type = "".to_string();
                        app.new_account.user_id = "".to_string();
                        app.new_account.card_limit = 0.0;
                        app.input_content = InputContent::AccountName;
                    },
                    KeyCode::Char('t') => {
                        // add new transaction
                        app.page = Page::NewTransaction;
                        app.input_content = InputContent::TransactionDescription;
                    },
                    KeyCode::Char('l') => {
                        // iterate account list
                        app.list_content = ListType::Acct;
                        app.select_first();
                    },
                    KeyCode::Char('s') => {
                        // iterate transaction list
                        if app.new_account.acct_id != "" {
                            app.list_content = ListType::Trans;
                            app.select_first();
                        }
                    },
                    KeyCode::Char('d') => {
                        app.delete_account().await;
                    }
                    KeyCode::Enter => {
                        app.update_account().await;
                        app.refresh_transactions().await;
                    }
                    _ => {}
                },
                InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
                    KeyCode::Enter => app.submit_message().await,
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
                    KeyCode::Esc => {
                        app.stop_select();
                        app.input_content = InputContent::AccountName;
                    },
                    KeyCode::Enter => {
                        app.confirm_selection();
                        if app.list_content == ListType::Acct {
                            app.filter_trans_category = "".to_string();
                            app.filter_trans_type = "".to_string();
                            app.refresh_transactions().await;
                        }
                    },
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
                    KeyCode::Char('c') => {
                        app.page = Page::AccountDetails;
                    },
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    },
                    KeyCode::Enter => {
                        app.create_new_account().await;
                        app.page = Page::AccountDetails;
                    }
                    _ => {}
                },
                InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
                    KeyCode::Enter => app.submit_message().await,
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
                    KeyCode::Up => app.prev_input(),
                    KeyCode::Down => app.next_input(),
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    },
                    KeyCode::Char('c') => {
                        app.page = Page::AccountDetails;
                        if app.new_account.acct_id != "" {
                            app.list_content = ListType::Trans;
                            app.select_first();
                        }
                    },
                    KeyCode::Char('d') => {
                        if app.page == Page::EditTransaction {
                            app.delete_transaction().await;
                            app.page = Page::AccountDetails;
                            if app.new_account.acct_id != "" {
                                app.list_content = ListType::Trans;
                                app.select_first();
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if app.page == Page::NewTransaction{
                            app.create_or_update_transaction(true).await;
                        } else {
                            app.create_or_update_transaction(false).await;
                        }
                        if app.new_account.acct_id != "" {
                            app.list_content = ListType::Trans;
                            app.select_first();
                        }
                    }
                    _ => {}
                },
                InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
                    KeyCode::Enter => app.submit_message().await,
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
