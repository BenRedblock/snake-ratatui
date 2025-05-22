use crate::utils::helpers::vec_to_string;

use super::utils::enums::App;
use ratatui::{
    Frame,
    style::{Color, Style},
    symbols::Marker,
    text::Text,
    widgets::{
        Block, Paragraph,
        canvas::{Canvas, Points},
    },
};
pub fn render(frame: &mut Frame, app: &App) {
    let header_block = Block::default()
        .title("Snake Game")
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());
    let width = 100;
    let height = 50;

    let vertical_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            [
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(height as u16),
            ]
            .as_ref(),
        )
        .split(frame.area());

    frame.render_widget(header_block, vertical_chunks[0]);

    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(
            [
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(width as u16),
                ratatui::layout::Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(vertical_chunks[1]);
    let inner_area = chunks[1];

    let canvas = Canvas::default()
        .block(
            Block::default()
                .title("Game Area")
                .borders(ratatui::widgets::Borders::ALL),
        )
        .x_bounds([0.0, width as f64])
        .y_bounds([0.0, height as f64])
        .marker(Marker::HalfBlock)
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &app.snake,
                color: Color::Red,
            })
        });
    frame.render_widget(canvas, inner_area);

    let left_block = Block::default()
        .title("Left Bar")
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());

    let paragraphs = Paragraph::new(Text::from(vec_to_string(&app.snake)))
        .block(left_block)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(paragraphs, chunks[0]);
}
