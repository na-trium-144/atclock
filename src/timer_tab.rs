use std::f64::consts::PI;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Stylize},
    text::Text,
};

#[derive(Debug)]
pub struct TimerState {
    end_time: Option<chrono::DateTime<chrono::Local>>,
    last_bell_time: Option<chrono::DateTime<chrono::Local>>,
    duration: chrono::TimeDelta,
    remaining: chrono::TimeDelta,
    cursor_pos: CursorPos,
    running: bool,
}
#[derive(Debug, Clone, Copy)]
enum CursorPos {
    Min1,
    Sec10,
    Sec1,
}
impl Default for TimerState {
    fn default() -> TimerState {
        TimerState {
            end_time: None,
            last_bell_time: None,
            duration: chrono::TimeDelta::zero(),
            remaining: chrono::TimeDelta::zero(),
            cursor_pos: CursorPos::Min1,
            running: false,
        }
    }
}

pub fn update_timer(state: &mut TimerState) -> super::DisplayData {
    let block_title = if state.running {
        "Started".to_string()
    } else {
        "Idle".to_string()
    };
    if let Some(t) = state.end_time {
        if t < chrono::Local::now() {
            if state
                .last_bell_time
                .is_none_or(|t| t < chrono::Local::now() - chrono::TimeDelta::milliseconds(250))
            {
                print!("\x07");
                state.last_bell_time = Some(chrono::Local::now());
            }
            state.remaining = chrono::TimeDelta::zero();
        } else {
            state.remaining = t - chrono::Local::now();
        }
    }
    let block_content = format!(
        "{}:{:02}.{:03}",
        state.remaining.num_minutes(),
        state.remaining.num_seconds() % 60,
        state.remaining.num_milliseconds() % 1000
    );
    let ms = (state.remaining.num_milliseconds() % 1000) as f64;
    let sec = (state.remaining.num_seconds() % 60) as f64 + ms / 1000.;
    let min = state.remaining.num_minutes() as f64 + sec / 60.;
    let analog_state = super::analog::ClockState {
        sec_rad: ms * PI / 500.,
        min_rad: sec * PI / 30.,
        hour_rad: min * PI / 30.,
    };
    super::DisplayData {
        block_title,
        block_content,
        analog_state,
    }
}

pub fn handle_key_event(state: &mut TimerState, key: &KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Char(' ')) => {
            if state.running {
                state.running = false;
                state.end_time = None;
            } else {
                state.running = true;
                state.end_time = Some(chrono::Local::now() + state.duration);
            }
        }
        (_, KeyCode::Up) => {
            match state.cursor_pos {
                CursorPos::Min1 => state.duration += chrono::TimeDelta::minutes(1),
                CursorPos::Sec10 => state.duration += chrono::TimeDelta::seconds(10),
                CursorPos::Sec1 => state.duration += chrono::TimeDelta::seconds(1),
            };
        }
        (_, KeyCode::Down) => {
            match state.cursor_pos {
                CursorPos::Min1 => state.duration -= chrono::TimeDelta::minutes(1),
                CursorPos::Sec10 => state.duration -= chrono::TimeDelta::seconds(10),
                CursorPos::Sec1 => state.duration -= chrono::TimeDelta::seconds(1),
            };
            if state.duration < chrono::TimeDelta::zero() {
                state.duration = chrono::TimeDelta::zero();
            }
        }
        (_, KeyCode::Left) => {
            match state.cursor_pos {
                CursorPos::Min1 => {}
                CursorPos::Sec10 => state.cursor_pos = CursorPos::Min1,
                CursorPos::Sec1 => state.cursor_pos = CursorPos::Sec10,
            };
        }
        (_, KeyCode::Right) => {
            match state.cursor_pos {
                CursorPos::Min1 => state.cursor_pos = CursorPos::Sec10,
                CursorPos::Sec10 => state.cursor_pos = CursorPos::Sec1,
                CursorPos::Sec1 => {}
            };
        }
        _ => {}
    }
}

pub fn render_panel(frame: &mut Frame, panel_area: Rect, state: &TimerState) {
    let panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(panel_area);

    frame.render_widget(
        Text::from("[Space]: Start / Stop")
            .add_modifier(Modifier::DIM)
            .centered(),
        panel_layout[1],
    );
    frame.render_widget(
        Text::from("↑↓/←→: Edit time")
            .add_modifier(Modifier::DIM)
            .centered(),
        panel_layout[2],
    );

    match state.cursor_pos {
        CursorPos::Min1 => {
            frame.render_widget(Text::from(" ^      ").centered(), panel_layout[4]);
            frame.render_widget(Text::from(" v      ").centered(), panel_layout[6]);
        }
        CursorPos::Sec10 => {
            frame.render_widget(Text::from("     ^  ").centered(), panel_layout[4]);
            frame.render_widget(Text::from("     v  ").centered(), panel_layout[6]);
        }
        CursorPos::Sec1 => {
            frame.render_widget(Text::from("       ^").centered(), panel_layout[4]);
            frame.render_widget(Text::from("       v").centered(), panel_layout[6]);
        }
    };
    frame.render_widget(
        Text::from(format!(
            "{:2} : {} {}",
            state.duration.num_minutes(),
            state.duration.num_seconds() / 10 % 6,
            state.duration.num_seconds() % 10,
        ))
        .add_modifier(Modifier::BOLD)
        .centered(),
        panel_layout[5],
    );
}
