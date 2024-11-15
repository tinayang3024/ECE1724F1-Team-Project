use std::error;
use crate::input::{ InputMode, Page, InputContent, ListType };
// use ratatui::widgets::ListState;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};


/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum Status {
//     Selected,
//     Default,
// }

#[derive(Debug, PartialEq, Clone)]
pub struct TransRecord {
    pub transaction_id: String,
    pub timestamp: String,
    pub trans_type: String, // expense or income
    pub category: String,
    pub description: String,
    pub amount: f64,
    // acct_id: Int, // ? 
    // status: Status, // no need
}

#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    pub acct_id: String, 
    pub acct_name: String,
    pub acct_type: String, // Credit or Chequing or Savings?
    pub user_id: String,
    pub card_limit: f64,
    // pub status: Status
}

struct TransList {
    items: Vec<TransRecord>,
    state: ListState,
}

struct AccountList {
    items: Vec<Account>,
    state: ListState,
}

impl FromIterator<(&'static str, &'static str, &'static str, &'static str, &'static str, f64)> for TransList {
    fn from_iter<I: IntoIterator<Item = (&'static str, &'static str, &'static str, &'static str, &'static str, f64)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(trans_id, timestamp, trans_type, category, descrip, amt)| TransRecord::new(trans_id, timestamp, trans_type, category, descrip, amt))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl TransRecord {
    fn new(trans_id: &str, timestamp: &str, trans_type: &str, category: &str, descrip: &str, amt: f64) -> Self {
        Self {
            transaction_id: trans_id.to_string(),
            timestamp: timestamp.to_string(),
            trans_type: trans_type.to_string(),
            category: category.to_string(),
            description: descrip.to_string(),
            amount: amt,
            // status: Status::Default,
            // acct_id: 
        }
    }
}

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

impl FromIterator<(&'static str, &'static str, &'static str, &'static str, f64)> for AccountList {
    fn from_iter<I: IntoIterator<Item = (&'static str, &'static str, &'static str, &'static str, f64)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(acct_id, acct_name, user_id, acct_type, card_limit)| Account::new(acct_id, acct_name, user_id, acct_type, card_limit))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl Account {
    fn new(acct_id: &str, account_name: &str, user_id: &str, acct_type: &str, card_limit: f64) -> Self {
        Self {
            acct_id: acct_id.to_string(),
            acct_name: account_name.to_string(),
            acct_type: acct_type.to_string(),
            user_id: user_id.to_string(),
            card_limit: card_limit,
            // status: Status::Default,
        }
    }
}

impl From<&Account> for ListItem<'_> {
    fn from(value: &Account) -> Self {
        // let line = match value.status {
        //     // Status::Default => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
        //     // Status::Selected => {
        //     //     Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
        //     // }
        //     Status::Default => Line::styled(format!(" ☐ {}", value.acct_id), TEXT_FG_COLOR),
        //     Status::Selected => {
        //         Line::styled(format!(" ✓ {}", value.acct_id), COMPLETED_TEXT_FG_COLOR)}
        // };
        let line = Line::styled(format!(" - {}: {}, {}", value.acct_id, value.acct_name, value.acct_type), COMPLETED_TEXT_FG_COLOR);
        ListItem::new(line)
    }
}

impl From<&TransRecord> for ListItem<'_> {
    fn from(value: &TransRecord) -> Self {
        // let line = match value.status {
        //     Status::Todo => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
        //     Status::Completed => {
        //         Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
        //     }
        // };
        let line = Line::styled(format!(" - {}: {}, {}", value.transaction_id, value.trans_type, value.amount), COMPLETED_TEXT_FG_COLOR);
        ListItem::new(line)
    }
}

/// Application.
// #[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// Username of the user
    pub username: String,
    /// List of user accounts
    pub accounts: AccountList,
    /// Selected account_id
    // pub account_selected_idx: usize,
    /// List of transaction history
    pub trans_history: TransList,
    /// new accounts
    pub new_account: Account,
    pub new_trans: TransRecord,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current input
    pub input: String,
    /// input index
    pub character_index: usize,

    pub page: Page,

    pub input_content: InputContent,
    pub list_content: ListType,

    pub new_trans_question_list: Vec<InputContent>,
    pub new_acct_question_list: Vec<InputContent>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            username: String::new(), // Default to an empty string
            // account_selected_idx: 0, // Default to an empty string
            accounts: AccountList::from_iter([
                (
                    "tina's account_id",
                    "tina's demo account",
                    "tina's user_id",
                    "Saving",
                    100.0,
                ),
                (
                    "sophie's account_id",
                    "sophie's demo account",
                    "sophie's user_id",
                    "Credit",
                    200.0,
                ),
            ]),   
            trans_history: TransList::from_iter([
                (
                    "transaction_id_1",
                    "2024/01/01",
                    "Expense",
                    "Food",
                    "spent for lunch on 1/1/2024",
                    20.1,
                ),
                (
                    "transaction_id_2",
                    "2024/01/02",
                    "Income",
                    "Salary",
                    "got paycheck on 1/1/2024",
                    3000.0,
                ),
                
            ]),  
            new_account: Account::new(
                "",
                "",
                "",
                "",
                0.0,
            ),
            new_trans: TransRecord::new(
                "",
                "",
                "",
                "",
                "",
                0.0,
            ),
            input_mode: InputMode::Normal,   // Default to not inputting
            input: String::new(), // Default to an empty string
            character_index: 0,
            page: Page::Login,
            input_content: InputContent::Username,
            list_content: ListType::Acct,
            new_trans_question_list: vec![
                InputContent::TransactionDescription,
                InputContent::TransactionType,
                InputContent::TransactionAmount,
                InputContent::TransactionCategory,
            ],
            new_acct_question_list: vec![
                InputContent::AccountName,
                InputContent::AccountType,
                InputContent::AccountLimit,
            ],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn submit_message(&mut self) {
        // self.messages.push(self.input.clone());
        match self.input_content {
            InputContent::Username => {
                self.username = self.input.clone();
                self.input_content = InputContent::AccountName;
                self.input_mode = InputMode::Normal;
                self.page = Page::AccountDetails;
            },
            InputContent::AccountID => self.new_account.acct_id = self.input.clone(),
            InputContent::AccountName => self.new_account.acct_name = self.input.clone(),
            InputContent::AccountType => self.new_account.acct_type = self.input.clone(),
            InputContent::AccountLimit => self.new_account.card_limit = self.input.clone().parse::<f64>().unwrap(),
            InputContent::TransactionAmount => self.new_trans.amount = self.input.clone().parse::<f64>().unwrap(),
            InputContent::TransactionCategory => self.new_trans.category = self.input.clone(),
            InputContent::TransactionDescription => self.new_trans.description = self.input.clone(),
            InputContent::TransactionType => self.new_trans.trans_type = self.input.clone(),
        };
        self.input.clear();
        self.input_mode = InputMode::Normal;
    }

    pub fn select_first(&mut self) {
        match self.list_content {
            ListType::Acct => {
                self.accounts.state.select_first();
            },
            ListType::Trans => {
                self.trans_history.state.select_first();
            }
        }
        self.input_mode = InputMode::ViewAccountList;
    }

    pub fn select_next(&mut self) {
        match self.list_content {
            ListType::Acct => {
                self.accounts.state.select_next();
            },
            ListType::Trans => {
                self.trans_history.state.select_next();
            }
        }
    }

    pub fn select_prev(&mut self) {
        match self.list_content {
            ListType::Acct => {
                self.accounts.state.select_previous();
            },
            ListType::Trans => {
                self.trans_history.state.select_previous();
            }
        }
    }

    pub fn confirm_selection(&mut self) {
        match self.list_content {
            ListType::Acct => {
                let idx = self.accounts.state.selected().unwrap();
                self.new_account = self.accounts.items[idx].clone();
            },
            ListType::Trans => {
                let idx = self.trans_history.state.selected().unwrap();
                self.new_trans = self.trans_history.items[idx].clone();
                self.input_content = InputContent::TransactionDescription;
                self.input_mode = InputMode::Normal;
                self.page = Page::EditTransaction;
            }
        }
    }

    pub fn stop_select(&mut self) {
        match self.list_content {
            ListType::Acct => {
                self.accounts.state.select(None);
            },
            ListType::Trans => {
                self.trans_history.state.select(None);
            }
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn find_index(vec: &Vec<InputContent>, target: InputContent) -> i32 {
        // Find the index of the target element
        if let Some(index) = vec.iter().position(|x| *x == target) {
            // Return the next element, if it exists
            index as i32
        } else {
            // If the target is not found, return None
            -1
        }
    }

    pub fn find_next_index(vec: &Vec<InputContent>, target: InputContent) -> i32 {
        // Find the index of the target element
        let index = App::find_index(vec, target);
        if index == -1 {
            -1
        } else if index ==  (vec.len()-1).try_into().unwrap() {
            index
        } else {
            index + 1
        }

        // if let Some(index) = vec.iter().position(|x| *x == target) {
        //     // Return the next element, if it exists
        //     vec.get(index + 1)
        // } else {
        //     // If the target is not found, return None
        //     None
        // }
    }

    pub fn find_prev_index(vec: &Vec<InputContent>, target: InputContent) -> i32 {
        // Find the index of the target element
        let index = App::find_index(vec, target);
        if index == -1 {
            -1
        } else if index == 0 {
            0
        } else {
            index - 1
        }

        // if let Some(index) = vec.iter().position(|x| *x == target) {
        //     // Return the next element, if it exists
        //     vec.get(index + 1)
        // } else {
        //     // If the target is not found, return None
        //     None
        // }
    }

    pub fn next_input(&mut self) {
        let question_list = match self.page {
            Page::NewAccount | Page::AccountDetails => {
                self.new_acct_question_list.clone()
            }, 
            Page::NewTransaction | Page::EditTransaction => {
                self.new_trans_question_list.clone()
            },
            _ => {Vec::new()}
        };
        self.input_content = question_list[App::find_next_index(&question_list, self.input_content.clone()) as usize].clone();
        // let mut found = false;
        // for window in question_list.windows(2) {
        //     if window[0] == self.input_content {
        //         // return Some(&window[1]); 
        //         self.input_content = window[1].clone();
        //         found = true;
        //     }
        // }
        // if !found {
        //     self.input_content = (*question_list.last().unwrap()).clone();
        // }
    }

    pub fn prev_input(&mut self) {
        let question_list = match self.page {
            Page::NewAccount | Page::AccountDetails  => {
                self.new_acct_question_list.clone()
            }, 
            Page::NewTransaction | Page::EditTransaction => {
                self.new_trans_question_list.clone()
            },
            _ => {Vec::new()}
        };
        self.input_content = question_list[App::find_prev_index(&question_list, self.input_content.clone()) as usize].clone();
        // let mut found = false;
        // for window in question_list.windows(2) {
        //     if window[1] == self.input_content {
        //         // return Some(&window[1]); 
        //         self.input_content = window[1].clone();
        //         found = true;
        //     }
        // }
        // if !found {
        //     self.input_content = question_list[0].clone();
        // }
    }

    const fn alternate_colors(i: usize) -> Color {
        if i % 2 == 0 {
            NORMAL_ROW_BG
        } else {
            ALT_ROW_BG_COLOR
        }
    }

    pub fn render_acct_list(&mut self, area: Rect, buf: &mut Buffer) {
    // pub fn render_acct_list(&mut self, area: Rect) {
        let block = Block::new()
            .title(Line::raw("Associated Account List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .accounts
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = App::alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.accounts.state);
    }

    pub fn render_trans_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Transaction Records").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .trans_history
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = App::alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.trans_history.state);
    }
    

}
