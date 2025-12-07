use std::{f64::consts::PI, time::Duration};

use chrono::Timelike;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::canvas::Canvas,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

mod clock;

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    clock_state: clock::ClockState,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            self.update_chrono();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn update_chrono(&mut self) {
        let now = chrono::Local::now();
        let sec = now.second() as f64;
        let min = now.minute() as f64 + sec / 60.;
        let hour = now.hour12().1 as f64 + min / 60.;
        self.clock_state.sec_rad = sec * PI / 30.;
        self.clock_state.min_rad = min * PI / 30.;
        self.clock_state.hour_rad = hour * PI / 6.;
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(0), Constraint::Length(10)])
            .split(frame.area());

        // 中央の正方形のエリアを取り出す
        let canvas_layout = layout[0];
        let canvas_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(
                    std::cmp::min(canvas_layout.height * 2, canvas_layout.width) / 2,
                ),
                Constraint::Min(0),
            ])
            .split(canvas_layout)[1];
        let canvas_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(std::cmp::min(canvas_layout.height * 2, canvas_layout.width)),
                Constraint::Min(0),
            ])
            .split(canvas_layout)[1];

        frame.render_widget(
            Canvas::default()
                .x_bounds([-1., 1.])
                .y_bounds([-1., 1.])
                .paint(|ctx| clock::draw(ctx, &canvas_layout, &self.clock_state)),
            canvas_layout,
        )
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
