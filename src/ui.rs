use crate::utils::{
    enums::{CollectableType, CurrentScreen},
    helpers::convert_ms_to_string,
};

use super::utils::enums::App;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Clear, Paragraph,
        canvas::{Canvas, Points},
    },
};
pub fn render(frame: &mut Frame, app: &App) {
    let vertical_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            [
                ratatui::layout::Constraint::Length((app.field_size.1 + 2) as u16),
                ratatui::layout::Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(frame.area());

    let horizontal_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(
            [
                ratatui::layout::Constraint::Max(30),
                ratatui::layout::Constraint::Length((app.field_size.0 + 2) as u16),
                ratatui::layout::Constraint::Max(30),
            ]
            .as_ref(),
        )
        .split(vertical_chunks[0]);
    let inner_area = horizontal_chunks[1];

    let right_block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());

    let left_block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());

    // Game area canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .title("Game Area")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(BorderType::QuadrantInside),
        )
        .x_bounds([0.0, app.field_size.0 as f64])
        .y_bounds([0.0, app.field_size.1 as f64])
        .marker(Marker::HalfBlock)
        .paint(|ctx| {
            for collectable in &app.collectables {
                ctx.draw(&Points {
                    coords: &[collectable.get_position()],
                    color: match collectable.collectable_type {
                        CollectableType::Apple => Color::Red,
                    },
                });
            }
            ctx.draw(&Points {
                coords: &app.snake[1..],
                color: Color::LightGreen,
            });
            ctx.draw(&Points {
                coords: &app.snake[..1],
                color: Color::Green,
            });
        });

    // Score and time display
    let mut score_lines = vec![];
    let score_span = Span::from(format!("Score: {}", app.snake.len() as i32 - 5));
    let time_span = Span::from(format!("Time: {}", convert_ms_to_string(&app.round_time)));
    score_lines.push(Line::from(score_span));
    score_lines.push(Line::from(time_span));
    let speed_color = match app.game_speed {
        speed if speed <= 0 => Color::Green,
        speed if speed <= 1 => Color::Yellow,
        _ => Color::Red,
    };
    let speed_text =
        Span::from(format!("Speed: {:.2}", app.game_speed)).style(Style::default().fg(speed_color));
    score_lines.push(Line::from(speed_text));

    match app.current_screen {
        CurrentScreen::Main => {
            frame.render_widget(canvas, inner_area);

            let paragraphs = Paragraph::new(score_lines).block(left_block.title("Game Info"));
            frame.render_widget(paragraphs, horizontal_chunks[0]);
            frame.render_widget(right_block, horizontal_chunks[2]);
        }
        CurrentScreen::Menu => {
            let start_game_text = match app.menu_cursor {
                Some(0) => {
                    if app.tick {
                        Text::from("Start Game")
                    } else {
                        Text::from("-> Start Game <-")
                    }
                }
                _ => Text::from("Start Game"),
            };
            let quit_game_text = match app.menu_cursor {
                Some(1) => {
                    if app.tick {
                        Text::from("Quit")
                    } else {
                        Text::from("-> Quit <-")
                    }
                }
                _ => Text::from("Quit"),
            };
            let start_paragraph = Paragraph::new(start_game_text)
                .style(Style::default().fg(Color::Green))
                .centered()
                .block(Block::default().borders(ratatui::widgets::Borders::ALL));
            let quit_paragraph = Paragraph::new(quit_game_text)
                .style(Style::default().fg(Color::Red))
                .centered()
                .block(Block::default().borders(ratatui::widgets::Borders::ALL));
            let menu_block = Block::default()
                .title("Menu")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(BorderType::QuadrantInside);
            let [button_area] = Layout::horizontal([Constraint::Length(20)])
                .flex(Flex::Center)
                .areas(inner_area);

            let menu_layout = Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
                .flex(Flex::Center)
                .split(button_area);
            frame.render_widget(menu_block, inner_area);
            frame.render_widget(start_paragraph, menu_layout[0]);
            frame.render_widget(quit_paragraph, menu_layout[1]);
            // Left and right blocks
            frame.render_widget(left_block, horizontal_chunks[0]);
            frame.render_widget(right_block, horizontal_chunks[2]);
        }
        CurrentScreen::Lost => {
            frame.render_widget(canvas, inner_area);
            let [inner_area] = Layout::horizontal([Constraint::Length(40)])
                .flex(Flex::Center)
                .areas(inner_area);

            let [inner_area] = Layout::vertical([Constraint::Length(4)])
                .flex(Flex::Center)
                .areas(inner_area);

            let lost_block = Block::default()
                .title("Game Over")
                .borders(ratatui::widgets::Borders::ALL)
                .style(Style::default().fg(Color::Red));

            let lost_text = Paragraph::new(vec![
                Line::from("You lost!"),
                Line::from("Press Enter to return to the menu."),
            ])
            .style(Style::default().fg(Color::White))
            .centered()
            .block(lost_block);
            frame.render_widget(Clear::default(), inner_area);
            frame.render_widget(lost_text, inner_area);
            // Left and right blocks
            let paragraphs = Paragraph::new(score_lines).block(left_block.title("Game Info"));
            frame.render_widget(paragraphs, horizontal_chunks[0]);
            frame.render_widget(right_block, horizontal_chunks[2]);
        }
    }
}
