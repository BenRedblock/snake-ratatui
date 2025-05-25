use crate::utils::{enums::CurrentScreen, helpers::vec_to_string};

use super::utils::enums::App;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::Text,
    widgets::{
        Block, BorderType, Paragraph,
        canvas::{Canvas, Points},
    },
};
pub fn render(frame: &mut Frame, app: &App) {
    let header_block = Block::default()
        .title("Snake Game")
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());

    let vertical_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            [
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length((app.field_size.1 + 2) as u16),
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
                ratatui::layout::Constraint::Length((app.field_size.0 + 2) as u16),
                ratatui::layout::Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(vertical_chunks[1]);
    let inner_area = chunks[1];

    let left_block = Block::default()
        .title("Left Bar")
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());

    let right_block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());

    let paragraphs = Paragraph::new(Text::from(vec_to_string(&app.snake)))
        .block(left_block)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(paragraphs, chunks[0]);
    frame.render_widget(right_block, chunks[2]);

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
            ctx.draw(&Points {
                coords: &app.snake[1..],
                color: Color::Red,
            });
            ctx.draw(&Points {
                coords: &app.snake[..1],
                color: Color::Green,
            });
        });
    match app.current_screen {
        CurrentScreen::Main => {
            frame.render_widget(canvas, inner_area);
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
        }
        CurrentScreen::Lost => {
            frame.render_widget(canvas, inner_area);
            let [inner_area] = Layout::horizontal([Constraint::Length(50)])
                .flex(Flex::Center)
                .areas(inner_area);

            let [inner_area] = Layout::vertical([Constraint::Length(3)])
                .flex(Flex::Center)
                .areas(inner_area);

            let lost_block = Block::default()
                .title("Game Over")
                .borders(ratatui::widgets::Borders::ALL)
                .style(Style::default().fg(Color::Red));

            let lost_text =
                Paragraph::new(Text::from("You lost! Press Enter to return to the menu."))
                    .style(Style::default().fg(Color::White))
                    .centered()
                    .block(lost_block);
            frame.render_widget(lost_text, inner_area);
        }
    }
}
