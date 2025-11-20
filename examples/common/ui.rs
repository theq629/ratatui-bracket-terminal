use rand::Rng;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Sparkline};
use ratatui::{symbols, Frame};
use std::collections::VecDeque;

const FULL: &str = "Û";
const HALF: &str = "Ü";
const EMPTY: &str = " ";
const BAR: symbols::bar::Set = symbols::bar::Set {
    full: FULL,
    seven_eighths: FULL,
    three_quarters: HALF,
    five_eighths: HALF,
    half: HALF,
    three_eighths: HALF,
    one_quarter: HALF,
    one_eighth: EMPTY,
    empty: EMPTY,
};

pub fn update_data(data: &mut VecDeque<u64>, rng: &mut impl Rng) {
    data.push_back(rng.random_range(0..100));
    while data.len() > 50 {
        data.pop_front();
    }
}

pub fn render_ui(frame: &mut Frame, data: &mut VecDeque<u64>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(7),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(frame.area());

    let greeting = Paragraph::new("Hello world");
    frame.render_widget(greeting, chunks[0]);

    let data = data.make_contiguous();
    let sparkline = Sparkline::default()
        .block(Block::default().title("Data"))
        .style(Style::default().fg(Color::Yellow))
        .data(&*data)
        .bar_set(BAR);
    frame.render_widget(sparkline, chunks[1]);
}
