use std::f64::consts::PI;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Stylize},
    text::Text,
};

#[derive(Debug)]
pub struct StopWatchState {
    start_time: Option<chrono::DateTime<chrono::Local>>,
    stop_time: Option<chrono::DateTime<chrono::Local>>,
    lap_time: Vec<chrono::DateTime<chrono::Local>>,
    running: bool,
}
impl Default for StopWatchState {
    fn default() -> StopWatchState {
        StopWatchState {
            start_time: None,
            stop_time: None,
            lap_time: vec![],
            running: false,
        }
    }
}

pub fn update_sw(state: &StopWatchState) -> super::DisplayData {
    let stop_time = state.stop_time.or(Some(chrono::Local::now())).unwrap();
    let elapsed = match state.start_time {
        Some(t) => stop_time - t,
        _ => chrono::TimeDelta::zero(),
    };
    let block_title = if state.running {
        "Started".to_string()
    } else {
        "Idle".to_string()
    };
    let block_content = format!(
        "{}:{:02}.{:03}",
        elapsed.num_minutes(),
        elapsed.num_seconds() % 60,
        elapsed.num_milliseconds() % 1000
    );
    let ms = (elapsed.num_milliseconds() % 1000) as f64;
    let sec = (elapsed.num_seconds() % 60) as f64 + ms / 1000.;
    let min = elapsed.num_minutes() as f64 + sec / 60.;
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

pub fn handle_key_event(state: &mut StopWatchState, key: &KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Char(' ')) => {
            if state.running {
                state.running = false;
                state.stop_time = Some(chrono::Local::now());
            } else {
                state.running = true;
                state.stop_time = None;
                state.start_time = Some(chrono::Local::now());
                state.lap_time = vec![];
            }
        }
        (_, KeyCode::Char('m')) => {
            if state.running {
                state.lap_time.push(chrono::Local::now());
            }
        }
        _ => {}
    }
}

pub fn render_panel(frame: &mut Frame, panel_area: Rect, state: &StopWatchState) {
    let panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length((state.lap_time.len() + 1) as u16),
            Constraint::Min(0),
        ])
        .split(panel_area);
    let lap_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); state.lap_time.len() + 1])
        .split(panel_layout[4]);

    frame.render_widget(
        Text::from("[Space]: Start / Stop")
            .add_modifier(Modifier::DIM)
            .centered(),
        panel_layout[1],
    );
    frame.render_widget(
        Text::from("[m]: Lap")
            .add_modifier(Modifier::DIM)
            .centered(),
        panel_layout[2],
    );
    for i in 0..state.lap_time.len() {
        let elapsed = if i >= 1 {
            state.lap_time[i] - state.lap_time[i - 1]
        } else {
            match state.start_time {
                Some(t) => state.lap_time[i] - t,
                _ => chrono::TimeDelta::zero(),
            }
        };
        render_lap_line(frame, lap_layout[i], i + 1, elapsed);
    }
    if state.start_time.is_some() {
        let stop_time = state.stop_time.or(Some(chrono::Local::now())).unwrap();
        let elapsed = if state.lap_time.len() >= 1 {
            stop_time - state.lap_time[state.lap_time.len() - 1]
        } else {
            match state.start_time {
                Some(t) => stop_time - t,
                _ => chrono::TimeDelta::zero(),
            }
        };
        render_lap_line(
            frame,
            lap_layout[lap_layout.len() - 1],
            lap_layout.len(),
            elapsed,
        );
    }
}

fn render_lap_line(frame: &mut Frame, lap_area: Rect, lap_num: usize, elapsed: chrono::TimeDelta) {
    let lap_line_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(7),
            Constraint::Length(9),
            Constraint::Min(0),
        ])
        .split(lap_area);
    frame.render_widget(
        Text::from(format!("Lap {}:", lap_num)).add_modifier(Modifier::ITALIC),
        lap_line_layout[1],
    );
    frame.render_widget(
        Text::from(format!(
            "{}:{:02}.{:03}",
            elapsed.num_minutes(),
            elapsed.num_seconds() % 60,
            elapsed.num_milliseconds() % 1000
        ))
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC)
        .right_aligned(),
        lap_line_layout[2],
    );
}
