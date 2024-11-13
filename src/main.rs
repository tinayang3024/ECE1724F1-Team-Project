// ANCHOR: imports
use std::io;
use image;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint::{self, Fill, Length, Max, Min, Percentage, Ratio},
        Direction,
        Layout, Rect,
    },
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Borders, Widget},
    DefaultTerminal, Frame,
};
// use ratatui_image::{picker::Picker, StatefulImage, protocol::StatefulProtocol};
use ratatui_image::{
    picker::Picker,
    protocol::{Protocol, StatefulProtocol},
    Image, Resize, StatefulImage,
};
// use ratatui::{
//     buffer::Buffer,
//     crossterm::event::{self, Event, KeyCode, KeyEventKind},
//     layout::{
//         Constraint::{self, Fill, Length, Max, Min, Percentage, Ratio},
//         Layout, Rect,
//     },
//     style::{palette::tailwind, Color, Modifier, Style, Stylize},
//     symbols,
//     text::Line,
//     widgets::{
//         Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
//         Tabs, Widget,
//     },
//     DefaultTerminal,
// };
// ANCHOR_END: imports

fn main() -> Result<(),Box<dyn std::error::Error>> {

    let mut terminal = ratatui::init();




    let path = "./assets/test_image.png";
    let image_source = image::io::Reader::open(path).unwrap().decode().unwrap();

    let mut picker = Picker::from_query_stdio().unwrap();

    let image_static = picker
        .new_protocol(image_source.clone(), Rect::new(0, 0, 30, 16), Resize::Fit(None))
        .unwrap();

    let image_fit_state = picker.new_resize_protocol(image_source.clone());
    // let image_crop_state = picker.new_resize_protocol(image_source.clone());

    let mut background = String::new();


    // let mut app = App { exit: false, image: image_fit_state };
    let mut app = App { exit: false };
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    // app_result
    Ok(())
}

// ANCHOR: app
#[derive(Debug, Default)]
pub struct App {
    // counter: u8,
    exit: bool,
    // image: StatefulProtocol,

    username: String,
    account: String,
}
// ANCHOR_END: app

// ANCHOR: impl App
impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {

        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
        

        let image_widget = StatefulImage::new(None).resize(Resize::Fit(None));
        let block_right_top = Block::default().borders(Borders::ALL).title("Fit");

        match self.username {
            Some(username) => {
                println!("logged in");
            },
            _ => {
                println!("not logged in");
                frame.render_widget(&mut *self, left)
            }
        }

    
        // Render the widgets
        frame.render_widget(&mut *self, left);
        // frame.render_stateful_widget(image_widget, block_right_top.inner(right_chunks[0]), &mut self.image);
        // frame.render_widget(&mut *self, right_chunks[0]);
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    // ANCHOR: handle_key_event fn
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }
    // ANCHOR_END: handle_key_event fn

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        // self.counter += 1;
        self.exit = true;

    }

    fn decrement_counter(&mut self) {
        // self.counter -= 1;
        self.exit = true;

    }
}
// ANCHOR_END: impl App

// ANCHOR: impl Widget
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            // self.counter.to_string().yellow(),
            "fixed".to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
// ANCHOR_END: impl Widget
