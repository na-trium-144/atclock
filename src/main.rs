use std::f64::consts::PI;

use chrono::Timelike;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    text::{self, Text},
    widgets::{
        Block, Paragraph,
        canvas::{self, Canvas, Circle, Context},
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
                .paint(|ctx| self.draw_clock(ctx, &canvas_layout)),
            canvas_layout,
        )
    }

    fn draw_clock(&self, ctx: &mut Context, area: &Rect) {
        ctx.draw(&Circle {
            x: 0.,
            y: 0.,
            radius: 1.,
            color: Color::DarkGray,
        });
        let w = area.width as f64;
        let h = area.height as f64;
        // for num in 0..4 {
        //     ctx.print((-(w / 2.).floor() + num as f64) / (w / 2.), 0., num.to_string());
        //     ctx.print(((w / 2.).floor() - num as f64) / (w / 2.), 0., num.to_string());
        //     ctx.print(0., (-(h / 2.).floor() + num as f64) / (h / 2.), num.to_string());
        //     ctx.print(0., ((h / 2.).floor() - num as f64) / (h / 2.), num.to_string());
        // }
        for num in 1..13 {
            // -w/2+1 <= x <= w/2 の範囲のみ正しく表示される
            // 四捨五入の境界を回避するために0.5の代わりに0.4999
            let x = (num as f64 * PI / 6.).sin() * ((w / 2.).ceil() - 2.5) + 0.4999
                - (if num >= 10 { 0.5 } else { 0. });
            // -h/2 <= y <= h/2-1 の範囲のみ正しく表示される
            let y = (num as f64 * PI / 6.).cos() * ((h / 2.).ceil() - 1.5) - 0.4999;
            ctx.print(
                x.round() / (w / 2.),
                y.round() / (h / 2.),
                text::Line::from(num.to_string()).fg(Color::DarkGray),
            );
        }

        let now = chrono::Local::now();
        let sec = now.second() as f64;
        let min = now.minute() as f64 + sec / 60.;
        let hour = now.hour12().1 as f64 + min / 60.;
        ctx.draw(&canvas::Line {
            x1: 0.,
            y1: 0.,
            x2: (hour * PI / 6.).sin() * 0.5,
            y2: (hour * PI / 6.).cos() * 0.5,
            color: Color::Red,
        });
        ctx.draw(&canvas::Line {
            x1: 0.,
            y1: 0.,
            x2: (min * PI / 30.).sin() * 0.7,
            y2: (min * PI / 30.).cos() * 0.7,
            color: Color::Blue,
        });
        ctx.draw(&canvas::Line {
            x1: 0.,
            y1: 0.,
            x2: (sec * PI / 30.).sin() * 0.8,
            y2: (sec * PI / 30.).cos() * 0.8,
            color: Color::Green,
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
