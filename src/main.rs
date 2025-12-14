use std::{f64::consts::PI, time::Duration};

use chrono::Timelike;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Stylize},
    text,
    widgets::{Block, BorderType, Paragraph, Tabs, canvas::Canvas},
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
    selected_tab: AppTab,
    clock_state: clock::ClockState,
    block_title: String,
    block_content: String,
}

#[derive(Default, Debug, Clone, Copy)]
enum AppTab {
    #[default]
    Clock,
    Timer,
    StopWatch,
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
            match self.selected_tab {
                AppTab::Clock => {
                    self.update_chrono();
                }
                AppTab::Timer => {
                    // TODO!
                    self.block_title = "Timer".to_string();
                    self.block_content = "aaaaa".to_string();
                    self.clock_state.sec_rad = 0.;
                    self.clock_state.min_rad = 0.;
                    self.clock_state.hour_rad = 0.;
                }
                AppTab::StopWatch => {
                    // TODO!
                    self.block_title = "StopWatch".to_string();
                    self.block_content = "aaaaa".to_string();
                    self.clock_state.sec_rad = 0.;
                    self.clock_state.min_rad = 0.;
                    self.clock_state.hour_rad = 0.;
                }
            }
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn update_chrono(&mut self) {
        let now = chrono::Local::now();
        self.block_title = format!("{}", now.format("%Y-%m-%d %a"));
        self.block_content = format!("{}", now.format("%I:%M:%S %p"));
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
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());
        let tabs_area = vertical_layout[0];
        let canvas_area = vertical_layout[1];
        let digit_area = vertical_layout[2];
        // 中央の正方形のエリアを取り出す
        let canvas_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(std::cmp::min(canvas_area.height * 2, canvas_area.width)),
                Constraint::Min(0),
            ])
            .split(canvas_area)[1];
        let canvas_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(std::cmp::min(canvas_area.height * 2, canvas_area.width) / 2),
                Constraint::Min(0),
            ])
            .split(canvas_area)[1];
        let digit_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(18),
                Constraint::Min(0),
            ])
            .split(digit_area)[1];

        let tab_description: String = "Select Mode with [Tab]:".to_string();
        let tabs_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(tab_description.len() as u16),
                Constraint::Min(0),
            ])
            .split(tabs_area);
        let description_area = tabs_layout[0];
        let tabs_area = tabs_layout[1];

        frame.render_widget(
            text::Text::from(tab_description).add_modifier(Modifier::DIM),
            description_area,
        );
        frame.render_widget(
            Tabs::new(vec!["Clock", "Timer", "StopWatch"])
                .highlight_style(Modifier::BOLD | Modifier::ITALIC)
                .select(self.selected_tab as usize),
            tabs_area,
        );
        frame.render_widget(
            Canvas::default()
                .x_bounds([-1., 1.])
                .y_bounds([-1., 1.])
                .paint(|ctx| clock::draw(ctx, &canvas_area, &self.clock_state)),
            canvas_area,
        );
        frame.render_widget(
            Paragraph::new(&self.block_content[..])
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
                .fg(Color::Reset)
                .centered()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title(
                            text::Line::from(&self.block_title[..]).centered(),
                            // ↓不要
                            // .add_modifier(Modifier::ITALIC)
                            // .remove_modifier(Modifier::BOLD)
                            // .add_modifier(Modifier::DIM),
                        )
                        .fg(Color::Reset)
                        .add_modifier(Modifier::ITALIC)
                        .remove_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::DIM),
                ),
            digit_area,
        );
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
            (KeyModifiers::SHIFT, KeyCode::Tab) => {
                self.selected_tab = match self.selected_tab {
                    AppTab::Clock => AppTab::StopWatch,
                    AppTab::Timer => AppTab::Clock,
                    AppTab::StopWatch => AppTab::Timer,
                }
            }
            (_, KeyCode::Tab) => {
                self.selected_tab = match self.selected_tab {
                    AppTab::Clock => AppTab::Timer,
                    AppTab::Timer => AppTab::StopWatch,
                    AppTab::StopWatch => AppTab::Clock,
                }
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
