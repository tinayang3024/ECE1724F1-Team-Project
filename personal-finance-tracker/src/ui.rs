use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratatui::prelude::*;
use crate::app::App;
use crate::input::InputMode;

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
    let acct_id_position = right_content_inner_layout_sub[1];
    let acct_name_position = right_content_inner_layout_sub[2];
    let acct_type_position = right_content_inner_layout_sub[3];
    let card_limit_position = right_content_inner_layout_sub[4];
    let trans_his_position = right_content_inner_layout_sub[6];

    if app.username.is_empty() {
        // login or register
        let profile_section = Paragraph::new("").block(Block::bordered().title("Login or register a new account"));
        frame.render_widget(profile_section, left_content);

        let input = Paragraph::new(format!("username: {}", app.input))
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::ViewAccountList => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Register/Login"));
        frame.render_widget(input, username_position);
        frame.render_widget(
            Paragraph::new(format!("... login first to see account details ...")).block(Block::bordered().title("Account Details")),
            right_content,
        );
    } else {
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
        frame.render_widget(
            Paragraph::new(format!("Account ID: {}", "todo")).block(Block::bordered()),
            acct_id_position,
        );
        frame.render_widget(
            Paragraph::new(format!("Account Name: {}", "todo")).block(Block::bordered()),
            acct_name_position,
        );
        frame.render_widget(
            Paragraph::new(format!("Account Type: {}", "todo")).block(Block::bordered()),
            acct_type_position,
        );
        frame.render_widget(
            Paragraph::new(format!("Card Limit: {}", "todo")).block(Block::bordered()),
            card_limit_position,
        );
        app.render_trans_list(trans_his_position, frame.buffer_mut());

        frame.render_widget(
            Paragraph::new("").block(Block::bordered()).block(Block::bordered().title("Account Details")),
            right_content,
        );

    }
}
