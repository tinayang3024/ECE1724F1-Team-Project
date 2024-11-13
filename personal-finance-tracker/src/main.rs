use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};
// use ratatui::crossterm::{self, Event, KeyCode, KeyEventKind};

use crate::{
    app::{App, AppResult},
    // crossterm::event::{self, Event, KeyCode, KeyEventKind},
    // crossterm::event::{KeyCode, KeyEventKind},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
    // input::InputMode,
};

pub mod input;
pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        // if let Event::Key(key) = event::read()? {
        //     match app.input_mode {
        //         InputMode::Normal => match key.code {
        //             KeyCode::Char('e') => {
        //                 app.input_mode = InputMode::Editing;
        //             }
        //             KeyCode::Char('q') => {
        //                 // return Ok(());
        //                 app.running = false;
        //             }
        //             _ => {}
        //         },
        //         // InputMode::Normal => {
        //         //     match tui.events.next().await? {
        //         //         Event::Tick => app.tick(),
        //         //         Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
        //         //         Event::Mouse(_) => {}
        //         //         Event::Resize(_, _) => {}
        //         //     }
        //         // },
        //         InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
        //             KeyCode::Enter => app.submit_message(),
        //             KeyCode::Char(to_insert) => app.enter_char(to_insert),
        //             KeyCode::Backspace => app.delete_char(),
        //             KeyCode::Left => app.move_cursor_left(),
        //             KeyCode::Right => app.move_cursor_right(),
        //             KeyCode::Esc => app.input_mode = InputMode::Normal,
        //             _ => {}
        //         },
        //         InputMode::Editing => {}
        //     }
        // }
        match tui.events.next().await? {
            // Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
