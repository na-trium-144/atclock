use std::f64::consts::PI;

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Stylize},
    text,
    widgets::canvas::{self, Circle, Context},
};

#[derive(Debug, Default)]
pub struct ClockState {
    pub hour_rad: f64,
    pub min_rad: f64,
    pub sec_rad: f64,
}

pub fn draw(ctx: &mut Context, area: &Rect, state: &ClockState) {
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
            text::Line::from(num.to_string()).fg(Color::Reset).add_modifier(Modifier::DIM),
        );
    }

    ctx.draw(&canvas::Line {
        x1: 0.,
        y1: 0.,
        x2: state.hour_rad.sin() * 0.5,
        y2: state.hour_rad.cos() * 0.5,
        color: Color::Red,
    });
    ctx.draw(&canvas::Line {
        x1: 0.,
        y1: 0.,
        x2: state.min_rad.sin() * 0.7,
        y2: state.min_rad.cos() * 0.7,
        color: Color::Blue,
    });
    ctx.draw(&canvas::Line {
        x1: 0.,
        y1: 0.,
        x2: state.sec_rad.sin() * 0.8,
        y2: state.sec_rad.cos() * 0.8,
        color: Color::Green,
    });
}
