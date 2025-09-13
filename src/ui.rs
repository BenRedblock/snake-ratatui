use crate::{
    game::App,
    utils::{
        collectables::AnyCollectable, enums::CurrentScreen, helpers::convert_seconds_to_string,
    },
};

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Margin},
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, Paragraph, Scrollbar, ScrollbarState,
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

    if frame.area().height < 27 || frame.area().width < 100 {
        let increase_size_paragraph = Paragraph::new(Line::from("min 100 x 27").centered()).block(
            Block::default()
                .style(Style::new().fg(Color::Red))
                .title(Line::from("The screen is to small"))
                .borders(Borders::ALL),
        );
        let increase_size_block = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Fill(1),
                ratatui::layout::Constraint::Max(30),
                ratatui::layout::Constraint::Fill(1),
            ])
            .split(
                Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([
                        ratatui::layout::Constraint::Fill(1),
                        ratatui::layout::Constraint::Max(3),
                        ratatui::layout::Constraint::Fill(1),
                    ])
                    .split(frame.area())[1],
            )[1];
        frame.render_widget(increase_size_paragraph, increase_size_block);
        return;
    }

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
                if !collectable.is_visible() {
                    continue;
                }
                ctx.draw(&Points {
                    coords: &[collectable.get_position()],
                    color: match collectable {
                        AnyCollectable::Apple(_) => Color::Red,
                        AnyCollectable::Speed(_) => Color::Yellow,
                        AnyCollectable::Reverse(_) => Color::Blue,
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
    // Highscpre display
    //
    let vertical_scroll = 0;
    let scrollbar = Scrollbar::default();
    let mut scores: Vec<Line> = app
        .get_highscores()
        .iter()
        .enumerate()
        .map(|(i, s)| {
            return Line::from(format!("# {}: {} - {}", i + 1, s.player_name, s.score));
        })
        .collect();
    scores.insert(0, Line::from("Highscores:"));
    let highscore_paragraph = Paragraph::new(scores.clone())
        .scroll((vertical_scroll as u16, 0))
        .block(right_block);

    let mut scrollbar_state = ScrollbarState::new(scores.len()).position(vertical_scroll);
    frame.render_widget(highscore_paragraph, horizontal_chunks[2]);
    frame.render_stateful_widget(
        scrollbar,
        horizontal_chunks[2].inner(Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
    // Score and time display
    let mut score_lines = vec![];
    let score_span = Span::from(format!("Score: {}", app.get_score()));
    let time_span = Span::from(format!(
        "Time: {}",
        convert_seconds_to_string(&app.round_time)
    ));
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
    let left_vertical_chunks = Layout::default()
        .constraints([
            ratatui::layout::Constraint::Percentage(30),
            ratatui::layout::Constraint::Percentage(70),
        ])
        .split(horizontal_chunks[0]);

    // item Info section
    let invisible_collectables = app.collectables.iter().filter(|collectable| {
        return !collectable.is_visible();
    });
    let mut collectable_lines = vec![];
    for collectable in invisible_collectables {
        let collectable_text = match collectable {
            AnyCollectable::Apple(_apple) => String::from("Apple"),
            AnyCollectable::Speed(speed) => String::from(format!(
                "Speed: {} seconds",
                speed.get_remaining_time().unwrap_or(0)
            )),
            AnyCollectable::Reverse(_reverse) => String::from("Reverse"),
        };
        collectable_lines.push(Line::from(collectable_text));
    }

    match app.current_screen {
        CurrentScreen::Main => {
            frame.render_widget(canvas, inner_area);

            let score_paragraph =
                Paragraph::new(score_lines).block(left_block.clone().title("Game Info"));
            frame.render_widget(score_paragraph, left_vertical_chunks[0]);
            let collectable_paragraph = Paragraph::new(collectable_lines).block(left_block.title(
                format!("Collectables - next in {} seconds", app.random_item_timer),
            ));
            frame.render_widget(collectable_paragraph, left_vertical_chunks[1]);
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
            // Left block

            let score_paragraph =
                Paragraph::new(score_lines).block(left_block.clone().title("Game Info"));
            frame.render_widget(score_paragraph, left_vertical_chunks[0]);

            let collectable_paragraph =
                Paragraph::new(collectable_lines).block(left_block.title("Collectables"));
            frame.render_widget(collectable_paragraph, left_vertical_chunks[1]);
        }
    }
}
