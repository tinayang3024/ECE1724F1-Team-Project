use ratatui::{
    style::{
        palette::tailwind::SLATE,
        Color, Modifier, Style,
    },
    text::Line,
    widgets::{ListItem, ListState},
};

pub const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(Color::Cyan);
pub const NORMAL_ROW_BG: Color = Color::Black;
pub const ALT_ROW_BG_COLOR: Color = Color::Black;
pub const SELECTED_STYLE: Style = Style::new().bg(Color::Yellow).add_modifier(Modifier::BOLD);
pub const TEXT_FG_COLOR: Color = Color::Cyan;
pub const COMPLETED_TEXT_FG_COLOR: Color = Color::Cyan;


#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
    ViewAccountList
}

#[derive(Debug, PartialEq)]
pub enum Page {
    Login,
    AccountDetails,
    NewAccount,
    NewTransaction,
    EditTransaction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputContent {
    Username,
    AccountID,
    AccountName,
    AccountType,
    AccountLimit,
    TransactionType,
    TransactionAmount,
    TransactionCategory,
    TransactionDescription,
    FilterTransType,
    FilterTransCategory,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListType {
    Acct,
    Trans,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TransRecord {
    pub transaction_id: String,
    pub timestamp: String,
    pub trans_type: String, // expense or income
    pub category: String,
    pub description: String,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    pub acct_id: String, 
    pub acct_name: String,
    pub acct_type: String, // Credit or Chequing or Savings?
    pub user_id: String,
    pub card_limit: f64,
}

pub struct TransList {
    pub items: Vec<TransRecord>,
    pub state: ListState,
}

pub struct AccountList {
    pub items: Vec<Account>,
    pub state: ListState,
}

impl TransRecord {
    pub fn new(trans_id: &str, timestamp: &str, trans_type: &str, category: &str, descrip: &str, amt: f64) -> Self {
        Self {
            transaction_id: trans_id.to_string(),
            timestamp: timestamp.to_string(),
            trans_type: trans_type.to_string(),
            category: category.to_string(),
            description: descrip.to_string(),
            amount: amt,

        }
    }
}

impl Account {
    pub fn new(acct_id: &str, account_name: &str, user_id: &str, acct_type: &str, card_limit: f64) -> Self {
        Self {
            acct_id: acct_id.to_string(),
            acct_name: account_name.to_string(),
            acct_type: acct_type.to_string(),
            user_id: user_id.to_string(),
            card_limit: card_limit,
        }
    }
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

impl From<&Account> for ListItem<'_> {
    fn from(value: &Account) -> Self {
        let line = Line::styled(format!(" - {}: {}, {}", value.acct_id, value.acct_name, value.acct_type), COMPLETED_TEXT_FG_COLOR);
        ListItem::new(line)
    }
}

impl From<&TransRecord> for ListItem<'_> {
    fn from(value: &TransRecord) -> Self {
        let line = Line::styled(format!(" - {}: {}, {}", value.transaction_id, value.trans_type, value.amount), COMPLETED_TEXT_FG_COLOR);
        ListItem::new(line)
    }
}