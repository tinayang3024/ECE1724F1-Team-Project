use std::error;
use crate::input::{ 
    InputMode, 
    Page, 
    InputContent,
    ListType,
    TransRecord,
    Account,
    TransList,
    AccountList,
    TODO_HEADER_STYLE,
    NORMAL_ROW_BG,
    ALT_ROW_BG_COLOR,
    SELECTED_STYLE,
 };
use crate::client::{
    query_or_create_user,
    create_or_update_account,
    query_account,
    create_or_update_transaction,
    delete_account,
    delete_transaction,
    delete_user
};
use chrono::Local;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, StatefulWidget},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

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


    /// new or selected account/transaction
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
    pub debug_msg: String,

    pub acct_balance: String,
    pub filter_trans_type: String,
    pub filter_trans_category: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            username: String::new(), // Default to an empty string
            accounts: AccountList::from_iter([]),   
            trans_history: TransList::from_iter([]),  
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
                InputContent::FilterTransType,
                InputContent::FilterTransCategory,
            ],
            debug_msg: String::new(),
            acct_balance: String::new(),
            filter_trans_type: String::new(),
            filter_trans_category: String::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn quit(&mut self) {
        self.running = false;
    }

    // TEXT INPUT RELATED FUNCTIONS
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

    pub async fn refresh_user_data(&mut self) {
        let accounts =
            if let Ok(accts) = query_or_create_user(&self.username).await {
                accts
            } else {
                return;
            };

        // clear the current account list
        self.accounts.items.clear();

        // populate loaded accounts
        for account in accounts.iter() {
            self.accounts.items.push(Account::new(
                &account.acct_id,
                &account.acct_name,
                &account.user_id,
                &account.acct_type,
                account.card_limit,
            ));
        }
    }

    pub async fn delete_user(&mut self) {
        self.debug_msg = format!("{:?} deleting", self.username);

        if self.username != "" {
            let deletion_status =
                if let Ok(ds) = delete_user(&self.username).await {
                    ds
                } else {
                    return;
                };
            self.debug_msg = format!("{:?} deleted", deletion_status);
            self.username = "".to_string();        
        } else {
            self.debug_msg = format!("deletion not triggered");
        }
    }

    pub async fn submit_message(&mut self) {
        match self.input_content {
            InputContent::Username => {
                self.username = self.input.clone();

                // query to create to load user data
                self.refresh_user_data().await;

                // rerouting
                self.input_content = InputContent::AccountName;
                self.input_mode = InputMode::Normal;
                self.page = Page::AccountDetails;
            },
            InputContent::AccountID => self.new_account.acct_id = self.input.clone(),
            InputContent::AccountName => self.new_account.acct_name = self.input.clone(),
            InputContent::AccountType => self.new_account.acct_type = self.input.clone(),
            InputContent::AccountLimit => self.new_account.card_limit = self.input.clone().parse::<f64>().unwrap_or_else(|_| 0.0),
            InputContent::TransactionAmount => self.new_trans.amount = self.input.clone().parse::<f64>().unwrap_or_else(|_| 0.0),
            InputContent::TransactionCategory => self.new_trans.category = self.input.clone(),
            InputContent::TransactionDescription => self.new_trans.description = self.input.clone(),
            InputContent::TransactionType => self.new_trans.trans_type = self.input.clone(),
            InputContent::FilterTransType => self.filter_trans_type = self.input.clone(),
            InputContent::FilterTransCategory => self.filter_trans_category = self.input.clone(),
        };
        self.input.clear();
        self.input_mode = InputMode::Normal;
    }

    pub async fn create_new_account(&mut self) {
        let _ = create_or_update_account(
            None,
            self.username.as_str(),
            self.new_account.acct_name.as_str(),
            self.new_account.acct_type.as_str(),
            self.new_account.card_limit
        )
        .await;
        
        // reload profile data after creating new account
        self.refresh_user_data().await;
    }

    pub async fn update_account(&mut self) {
        let _ = create_or_update_account(
            Some(self.new_account.acct_id.clone()),
            self.username.as_str(),
            self.new_account.acct_name.as_str(),
            self.new_account.acct_type.as_str(),
            self.new_account.card_limit
        )
        .await;

        // reload profile data after creating new account
        self.refresh_user_data().await;
    }

    pub async fn delete_account(&mut self) {
        if self.new_account.acct_id != "" {
            let _ = delete_account(
                self.new_account.acct_id.parse().unwrap(),
            )
            .await;
            self.new_account.acct_id = "".to_string();
        }
        // reload profile data after creating new account
        self.refresh_user_data().await;
    }

    pub async fn refresh_transactions(&mut self) {
        if self.new_account.acct_id == "" {
            return;
        }
        let transactions = 
            if let Ok(trans) = query_account(
                self.new_account.acct_id.parse().unwrap(),
                if self.filter_trans_type == "" { None } else { Some(self.filter_trans_type.clone()) },
                if self.filter_trans_category == "" { None } else { Some(self.filter_trans_category.clone()) },
            ).await {
                trans
            } else {
                return;
            };

        // populate loaded transactions
        self.trans_history.items.clear();
        self.trans_history.items = transactions.0;
        self.acct_balance = format!("{:.2}", transactions.1);
    }

    pub async fn create_or_update_transaction(&mut self, create: bool) {
        let timestamp = if create {
            Local::now().date_naive().to_string()
        } else {
            self.new_trans.timestamp.clone()
        };
    
        let trans_id = 
            if let Ok(tid) = create_or_update_transaction(
                if create { None } else { Some(self.new_trans.transaction_id.clone()) },
                &timestamp,
                &self.new_trans.trans_type,
                &self.new_trans.category,
                self.new_trans.amount,
                &self.new_trans.description,
                &self.new_account.acct_id,
            ).await {
                tid
            } else {
                return;
            };
    
        self.new_trans.transaction_id = trans_id;
    
        self.refresh_transactions().await;
        self.page = Page::AccountDetails;
    }

    pub async fn delete_transaction(&mut self) {
        if self.new_trans.transaction_id != "" {
            let _ = delete_transaction(
                self.new_trans.transaction_id.parse().unwrap(),
            )
            .await;
            self.new_trans.transaction_id = "".to_string();
        }
        // reload profile data after creating new account
        self.refresh_transactions().await;
    }


    // LIST RELATED FUNCTIONS
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
        } else if index == <usize as TryInto<i32>>::try_into(vec.len() - 1).unwrap() {
            index
        } else {
            index + 1
        }
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
    }

    pub fn next_input(&mut self) {
        let question_list = match self.page {
            Page::NewAccount => {
                Vec::from_iter(self.new_acct_question_list[..3].iter().cloned())
            }, 
            Page::AccountDetails => {
                self.new_acct_question_list.clone()
            },
            Page::NewTransaction | Page::EditTransaction => {
                self.new_trans_question_list.clone()
            },
            _ => Vec::new()
        };
        let index = App::find_next_index(&question_list, self.input_content.clone());
        self.input_content = question_list[index as usize].clone();
    }

    pub fn prev_input(&mut self) {
        let question_list = match self.page {
            Page::NewAccount => {
                Vec::from_iter(self.new_acct_question_list[..3].iter().cloned())
            }, 
            Page::AccountDetails => {
                self.new_acct_question_list.clone()
            },
            Page::NewTransaction | Page::EditTransaction => {
                self.new_trans_question_list.clone()
            },
            _ => Vec::new()
        };
        let index = App::find_prev_index(&question_list, self.input_content.clone());
        self.input_content = question_list[index as usize].clone();
    }

    const fn alternate_colors(i: usize) -> Color {
        if i % 2 == 0 {
            NORMAL_ROW_BG
        } else {
            ALT_ROW_BG_COLOR
        }
    }

    // COMPONENT RENDERING FUNCTIONS
    // account list 
    pub fn render_acct_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Associated Accounts").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

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

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.accounts.state);
    }

    // transaction list 
    pub fn render_trans_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Transaction Records").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

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

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.trans_history.state);
    }
    

}
