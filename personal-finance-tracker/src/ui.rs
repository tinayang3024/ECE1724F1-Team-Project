use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratatui::prelude::*;
use crate::app::App;
use crate::input::{ InputMode, InputContent, Page };


// not doing input validation for number inputs yet
pub fn render_input_field(app: &mut App, frame: &mut Frame, position: Rect, label: String, content_value: String, content: InputContent) {
    let line;
    let style;
    if app.input_mode == InputMode::Editing && app.input_content == content {
        line = format!("{}: {}", label, app.input);
        style = Style::default().fg(Color::Yellow);
    } else if app.input_content == content {
        line = format!("{}: {}", label, content_value);
        style = Style::default().fg(Color::Red);
    } else {
        line = format!("{}: {}", label, content_value);
        style = Style::default();
    }
    let input_field = Paragraph::new(line)
        .style(style)
        .block(Block::bordered());
    frame.render_widget(input_field, position);
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    // let vertical = Layout::vertical([
    //     Constraint::Length(1),
    //     Constraint::Max(3),
    //     Constraint::Length(1),
    //     Constraint::Min(0), // ignore remaining space
    // ]);
    let vert_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(70),
        ])
        .split(frame.area());

    // let hor_layout = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints(vec![
    //         Constraint::Percentage(10),
    //         Constraint::Percentage(10),
    //         Constraint::Percentage(80),
    //     ])
    //     .split(frame.area());

    let title = vert_layout[0];
    let subtitle = vert_layout[1];
    let content = vert_layout[2];

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(content);

    let left_content = content_layout[0];
    let right_content = content_layout[1];

    frame.render_widget(
        Paragraph::new("Personal Financial Tracker").block(Block::bordered()),
        title,
    );

    frame.render_widget(
        Paragraph::new(format!("input_content {:?}; page {:?}, t_q_list: {:?}", 
            app.input_content, 
            app.page,
            app.new_trans_question_list
        )).block(Block::bordered()),
        subtitle,
    );


    let left_content_inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10), // padding
            Constraint::Percentage(80), // content
            Constraint::Percentage(10), // padding
        ])
        .split(left_content)[1];

    let left_content_inner_layout_sub = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10), // padding
            Constraint::Percentage(15), // username
            Constraint::Percentage(10), // padding
            Constraint::Percentage(55), // list of accounts
            Constraint::Percentage(10), // list of accounts
        ])
        .split(left_content_inner_layout);
    let username_position = left_content_inner_layout_sub[1];
    let account_position = left_content_inner_layout_sub[3];

    let right_content_inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10), // padding
            Constraint::Percentage(80), // content
            Constraint::Percentage(10), // padding
        ])
        .split(right_content)[1];

    let right_content_inner_layout_sub = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10), // padding
            Constraint::Percentage(10), // account id
            Constraint::Percentage(10), // account name
            Constraint::Percentage(10), // account type
            Constraint::Percentage(10), // card limit
            Constraint::Percentage(10), // padding
            Constraint::Percentage(30), // list of transactions
            Constraint::Percentage(10), // padding
        ])
        .split(right_content_inner_layout);
    let right_row_1_position = right_content_inner_layout_sub[1];
    let right_row_2_position = right_content_inner_layout_sub[2];
    let right_row_3_position = right_content_inner_layout_sub[3];
    let right_row_4_position = right_content_inner_layout_sub[4];
    let trans_his_position = right_content_inner_layout_sub[6];

    // if app.username.is_empty() {
    match app.page {
        Page::Login =>{
        
            // login or register
            let profile_section = Paragraph::new("").block(Block::bordered().title("Login or register a new account"));
            frame.render_widget(profile_section, left_content);

            render_input_field(app, frame, username_position, "Username".to_string(), app.username.to_string(), InputContent::Username);

            frame.render_widget(
                Paragraph::new(format!("... login first to see account details ...")).block(Block::bordered().title("Account Details")),
                right_content,
            );
        },
        Page::AccountDetails => {
            // left profile
            let profile_section = Paragraph::new("").block(Block::bordered().title("Profile Data"));
            frame.render_widget(profile_section, left_content);

            // user name
            frame.render_widget(
                Paragraph::new(format!("username: {}", app.username)).block(Block::bordered()),
                username_position,
            );

            // accounts
            app.render_acct_list(account_position, frame.buffer_mut());


            if app.new_account.acct_id == "" {
                frame.render_widget(
                    Paragraph::new("loading...").block(Block::bordered()),
                    right_content,
                );
            } else {
                render_input_field(app, frame, right_row_1_position, "Account Name".to_string(), app.new_account.acct_name.to_string(), InputContent::AccountName);
                render_input_field(app, frame, right_row_2_position, "Account Type".to_string(), app.new_account.acct_type.to_string(), InputContent::AccountType);
                render_input_field(app, frame, right_row_3_position, "Card Limit".to_string(), app.new_account.card_limit.to_string(), InputContent::AccountLimit);

                app.render_trans_list(trans_his_position, frame.buffer_mut());

                frame.render_widget(
                    Paragraph::new("").block(Block::bordered()).block(Block::bordered().title(
                        format!("Account Details of {}", app.new_account.acct_id))
                    ),
                    right_content,
                );
            }
        },
        Page::NewAccount => {
            // left profile
            let profile_section = Paragraph::new("").block(Block::bordered().title("Profile Data"));
            frame.render_widget(profile_section, left_content);
            // user name
            frame.render_widget(
                Paragraph::new(format!("username: {}", app.username)).block(Block::bordered()),
                username_position,
            );
            // accounts
            app.render_acct_list(account_position, frame.buffer_mut());


            // right data
            render_input_field(app, frame, right_row_1_position, "Account Name".to_string(), app.new_account.acct_name.to_string(), InputContent::AccountName);
            render_input_field(app, frame, right_row_2_position, "Account Type".to_string(), app.new_account.acct_type.to_string(), InputContent::AccountType);
            render_input_field(app, frame, right_row_3_position, "Card Limit".to_string(), app.new_account.card_limit.to_string(), InputContent::AccountLimit);

            frame.render_widget(
                Paragraph::new("").block(Block::bordered()).block(Block::bordered().title("Register New Account")),
                right_content,
            );
        },
        Page::NewTransaction => {
            // left profile
            let profile_section = Paragraph::new("").block(Block::bordered().title("Profile Data"));
            frame.render_widget(profile_section, left_content);
            // user name
            frame.render_widget(
                Paragraph::new(format!("username: {}", app.username)).block(Block::bordered()),
                username_position,
            );
            // accounts
            app.render_acct_list(account_position, frame.buffer_mut());
            

            // right form
            render_input_field(app, frame, right_row_1_position, "Transaction Description".to_string(), app.new_trans.description.to_string(), InputContent::TransactionDescription);
            render_input_field(app, frame, right_row_2_position, "Transaction Type".to_string(), app.new_trans.trans_type.to_string(), InputContent::TransactionType);
            render_input_field(app, frame, right_row_3_position, "Transaction Amount".to_string(), app.new_trans.amount.to_string(), InputContent::TransactionAmount);
            render_input_field(app, frame, right_row_4_position, "Transaction Category".to_string(), app.new_trans.category.to_string(), InputContent::TransactionCategory);

            frame.render_widget(
                Paragraph::new("").block(Block::bordered()).block(Block::bordered().title("Record New Transaction")),
                right_content,
            );

        },
        Page::EditTransaction => {
            // left profile
            let profile_section = Paragraph::new("").block(Block::bordered().title("Profile Data"));
            frame.render_widget(profile_section, left_content);
            // user name
            frame.render_widget(
                Paragraph::new(format!("username: {}", app.username)).block(Block::bordered()),
                username_position,
            );
            // accounts
            app.render_acct_list(account_position, frame.buffer_mut());


            // right form
            // render_input_field(app, frame, right_row_1_position, "Transaction ID".to_string(), app.new_trans.transaction_id.to_string(), InputContent::TransactionDescription);
            render_input_field(app, frame, right_row_1_position, "Transaction Description".to_string(), app.new_trans.description.to_string(), InputContent::TransactionDescription);
            render_input_field(app, frame, right_row_2_position, "Transaction Type".to_string(), app.new_trans.trans_type.to_string(), InputContent::TransactionType);
            render_input_field(app, frame, right_row_3_position, "Transaction Amount".to_string(), app.new_trans.amount.to_string(), InputContent::TransactionAmount);
            render_input_field(app, frame, right_row_4_position, "Transaction Category".to_string(), app.new_trans.category.to_string(), InputContent::TransactionCategory);

            frame.render_widget(
                Paragraph::new("").block(Block::bordered()).block(Block::bordered().title(
                    format!("Edit Transaction {}", app.new_trans.transaction_id))
                ),
                right_content,
            );
            
        }

    }
}
