use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Stylize},
    text::Line,
    widgets::{
        Block, Paragraph,
        canvas::{Canvas, Circle, Context},
    },
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
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
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
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
                .paint(|ctx| self.draw_clock(ctx)),
            canvas_layout,
        )
    }

    fn draw_clock(&self, ctx: &mut Context) {
        ctx.draw(&Circle {
            x: 0.,
            y: 0.,
            radius: 1.,
            color: Color::DarkGray,
        });
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
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
