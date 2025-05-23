use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use crossterm::event::{self, KeyEvent};

use crate::{
    ui,
    utils::enums::{App, CurrentScreen, Direction, Event},
};

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            current_screen: CurrentScreen::Menu,
            menu_cursor: None,
            direction: Direction::Up,
            snake: vec![(21.0, 20.0), (22.0, 20.0), (20.0, 20.0)],
            speed: 4,
            blocked: true,
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let mut terminal = ratatui::init();

        let (event_tx, event_rx) = mpsc::channel::<Event>();

        self.create_threads(event_tx);
        let mut counter = 0;
        while !self.exit {
            let _ = terminal.draw(|frame| {
                ui::render(frame, &self);
            });
            if let Ok(event) = event_rx.try_recv() {
                match event {
                    Event::Input(key_event) => {
                        self.handle_input_events(key_event);
                    }
                    Event::GameTick => {
                        if counter == self.speed {
                            counter = 0;
                            self.on_tick();
                        } else {
                            counter += 1;
                        }
                    }
                }
            }
        }
        ratatui::restore();
        Ok(())
    }

    fn create_threads(&self, event_tx: Sender<Event>) {
        let tx_to_input_events = event_tx.clone();
        thread::spawn(move || {
            loop {
                match crossterm::event::read().unwrap() {
                    crossterm::event::Event::Key(key_event) => {
                        tx_to_input_events.send(Event::Input(key_event)).unwrap()
                    }
                    _ => {}
                }
            }
        });
        let tx_to_background_progress_events = event_tx.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(50));
                tx_to_background_progress_events
                    .send(Event::GameTick)
                    .unwrap();
            }
        });
    }

    fn handle_input_events(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => match key_event.code {
                event::KeyCode::Esc => self.exit = true,
                _ => self.handle_movement_input(&key_event),
            },
            CurrentScreen::Lost => {}
            CurrentScreen::Menu => match key_event.code {
                event::KeyCode::Esc => self.exit = true,
                _ => self.handle_movement_input(&key_event),
            },
        }
    }

    fn handle_movement_input(&mut self, key_event: &KeyEvent) {
        if !self.blocked {
            match key_event.code {
                event::KeyCode::Up => match self.direction {
                    Direction::Down => (),
                    _ => self.direction = Direction::Up,
                },
                event::KeyCode::Down => match self.direction {
                    Direction::Up => (),
                    _ => self.direction = Direction::Down,
                },
                event::KeyCode::Left => match self.direction {
                    Direction::Right => (),
                    _ => self.direction = Direction::Left,
                },
                event::KeyCode::Right => match self.direction {
                    Direction::Left => (),
                    _ => self.direction = Direction::Right,
                },
                _ => {}
            }
            self.blocked = true;
        }
    }

    fn on_tick(&mut self) {
        // Update the snake's position based on the current direction
        let (head_x, head_y) = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y + 0.5),
            Direction::Down => (head_x, head_y - 0.5),
            Direction::Left => (head_x - 1.0, head_y),
            Direction::Right => (head_x + 1.0, head_y),
        };
        self.snake.insert(0, new_head);
        self.snake.pop();
        self.blocked = false;
    }
}
