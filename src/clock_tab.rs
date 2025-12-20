use std::f64::consts::PI;

use chrono::Timelike;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, calendar},
};
use time::OffsetDateTime;

pub fn update_chrono() -> super::DisplayData {
    let now = chrono::Local::now();
    let block_title = format!("{}", now.format("%Y-%m-%d %a"));
    let block_content = format!("{}", now.format("%I:%M:%S %p"));
    let sec = now.second() as f64;
    let min = now.minute() as f64 + sec / 60.;
    let hour = now.hour12().1 as f64 + min / 60.;
    let analog_state = super::analog::ClockState {
        sec_rad: sec * PI / 30.,
        min_rad: min * PI / 30.,
        hour_rad: hour * PI / 6.,
    };
    super::DisplayData {
        block_title,
        block_content,
        analog_state,
    }
}

pub fn render_panel(frame: &mut Frame, panel_area: Rect) {
    let panel_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(panel_area)[1];
    let panel_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(24),
            Constraint::Min(0),
        ])
        .split(panel_area)[1];

    frame.render_widget(
        calendar::Monthly::new(
            OffsetDateTime::now_local().unwrap().date(),
            calendar::CalendarEventStore::today(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::REVERSED),
            ),
        )
        .default_style(Style::default().remove_modifier(Modifier::DIM))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .add_modifier(Modifier::DIM),
        )
        .show_month_header(
            Style::default()
                .remove_modifier(Modifier::DIM)
                .add_modifier(Modifier::BOLD),
        )
        .show_weekdays_header(
            Style::default()
                .add_modifier(Modifier::DIM)
                .add_modifier(Modifier::ITALIC),
        )
        .show_surrounding(Style::default().add_modifier(Modifier::DIM)),
        panel_area,
    );
}
