use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use ratatui::prelude::*;
use crate::app::App;
use crate::input::{ 
    InputMode, 
    Page, 
    InputContent,
};


// not doing input validation for number inputs yet
pub fn render_input_field(app: &mut App, frame: &mut Frame, position: Rect, label: String, content_value: String, content: InputContent) {
    let line;
    let style;
    if app.input_mode == InputMode::Editing && app.input_content == content {
        line = format!("{}: {}", label, app.input);
        style = Style::default().fg(Color::Yellow);
    } else if app.input_content == content {
        line = format!("{}: {}", label, content_value);
        style = Style::default().fg(Color::LightCyan);
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
    let vert_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(25),
            Constraint::Percentage(70),
        ])
        .split(frame.area());

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
        Paragraph::new("Personal Financial Tracker")
        .bold()
        .style(
            Style::new()
                // .fg(Color::Cyan)
                .bg(Color::Cyan),
        )
        .alignment(Alignment::Center)
        .block(Block::bordered()),
        title,
    );

    // debug message + key instruction
    let mut key_instructions: Vec<String> = Vec::new();
    key_instructions.push(String::from("Instructions: "));

    match app.page {
        Page::Login => {
            if app.input_mode == InputMode::Editing {
                key_instructions.push(String::from("Press return to submit the value"));
            } else {
                key_instructions.push(String::from("Press e to enter username"));
                key_instructions.push(String::from("Press esc or q to exit the application"));
            }
        },
        Page::AccountDetails => {
            if app.input_mode == InputMode::Editing {
                key_instructions.push(String::from("Press return to submit the value"));
            } else if app.input_mode == InputMode::ViewAccountList {
                key_instructions.push(String::from("Press up and down to select item"));
                key_instructions.push(String::from("Press return to confirm selection"));
                key_instructions.push(String::from("Press esc to exist selection mode"));
            } else {
                key_instructions.push(String::from("Press l to select account"));
                key_instructions.push(String::from("Press a to create new account"));
                key_instructions.push(String::from("Press b to delete user"));
                if app.new_account.acct_id != "" {
                    key_instructions.push(String::from("Press t to create new transaction"));
                    key_instructions.push(String::from("Press up and down to select account info or filter options"));
                    key_instructions.push(String::from("Press e to overwrite account info or filter options"));
                    key_instructions.push(String::from("Press enter to save the changes to the account or filter transactions"));    
                    key_instructions.push(String::from("Press s to select transaction"));
                    key_instructions.push(String::from("Press d to delete account"));
                } 
            }
        },
        Page::NewAccount => {
            key_instructions.push(String::from("Press up and down to select account info"));
            key_instructions.push(String::from("Press e to enter account info"));
            key_instructions.push(String::from("Press c to to back to account details page"));
            key_instructions.push(String::from("Press enter to create the account"));        
        },
        Page::NewTransaction => {
            key_instructions.push(String::from("Press up and down to select transaction info"));
            key_instructions.push(String::from("Press e to enter transaction info"));
            key_instructions.push(String::from("Press c to to back to account details page"));
            key_instructions.push(String::from("Press enter to create the transaction"));
        },
        Page::EditTransaction => {
            key_instructions.push(String::from("Press up and down to select transaction info"));
            key_instructions.push(String::from("Press e to overwrite transaction info"));
            key_instructions.push(String::from("Press enter to save changes to the transaction"));
            key_instructions.push(String::from("Press c to to back to account details page"));
            key_instructions.push(String::from("Press d to delete the transaction"));
        }
    }     

    frame.render_widget(
        Paragraph::new(key_instructions.join("\n")).block(Block::bordered()),
        subtitle,
    );

    // // debug messages
    // frame.render_widget(
    //     Paragraph::new(format!("input_content {:?}; page {:?}, t_q_list: {:?}, debug_msg: {:?}", 
    //         app.input_content, 
    //         app.page,
    //         app.new_trans_question_list,
    //         app.debug_msg
    //     )).block(Block::bordered()),
    //     subtitle,
    // );


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
            Constraint::Percentage(10), // padding
            Constraint::Percentage(20), // list of transactions
            Constraint::Percentage(10), // padding
        ])
        .split(right_content_inner_layout);
    let right_row_1_position = right_content_inner_layout_sub[1];
    let right_row_2_position = right_content_inner_layout_sub[2];
    let right_row_3_position = right_content_inner_layout_sub[3];
    let right_row_4_position = right_content_inner_layout_sub[4];
    let right_row_5_position = right_content_inner_layout_sub[5];
    let right_row_6_position = right_content_inner_layout_sub[6];
    let trans_his_position = right_content_inner_layout_sub[7];

    // if app.username.is_empty() {
    match app.page {
        Page::Login =>{
        
            // login or register
            let profile_section = Paragraph::new("").block(Block::bordered().title("Login or register a new account"));
            frame.render_widget(profile_section, left_content);

            render_input_field(app, frame, username_position, "Username".to_string(), app.username.to_string(), InputContent::Username);

            frame.render_widget(
                Paragraph::new(format!(
                    r#"
To view accounts for existing user profiles, enter a username that is registered with the system.
To create a new user profile, enter a non-existing username and an empty profile will be created.
                    "#)).block(Block::bordered().title("Account Details")),
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
                    Paragraph::new("Please select an account").block(Block::bordered()),
                    right_content,
                );
            } else {
                render_input_field(app, frame, right_row_1_position, "Account Name".to_string(), app.new_account.acct_name.to_string(), InputContent::AccountName);
                render_input_field(app, frame, right_row_2_position, "Account Type (Chequing/Credit)".to_string(), app.new_account.acct_type.to_string(), InputContent::AccountType);
                render_input_field(app, frame, right_row_3_position, "Card Limit".to_string(), app.new_account.card_limit.to_string(), InputContent::AccountLimit);
                render_input_field(app, frame, right_row_4_position, "Filter Transaction Type (Income/Expenses)".to_string(), app.filter_trans_type.to_string(), InputContent::FilterTransType);
                render_input_field(app, frame, right_row_5_position, "Filter Transaction Category".to_string(), app.filter_trans_category.to_string(), InputContent::FilterTransCategory);

                frame.render_widget(
                    Paragraph::new(format!("Balance: {}", app.acct_balance)),
                    right_row_6_position,
                );

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
            render_input_field(app, frame, right_row_2_position, "Account Type (Chequing/Credit)".to_string(), app.new_account.acct_type.to_string(), InputContent::AccountType);
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
            render_input_field(app, frame, right_row_2_position, "Transaction Type (Income/Expenses)".to_string(), app.new_trans.trans_type.to_string(), InputContent::TransactionType);
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
            render_input_field(app, frame, right_row_1_position, "Transaction Description".to_string(), app.new_trans.description.to_string(), InputContent::TransactionDescription);
            render_input_field(app, frame, right_row_2_position, "Transaction Type (Income/Expenses)".to_string(), app.new_trans.trans_type.to_string(), InputContent::TransactionType);
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
