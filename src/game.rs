use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use crate::{
    ui,
    utils::{
        collectables::{AnyCollectable, CollectableType},
        enums::{CurrentScreen, Direction, Event},
        scores::{Score, ScoreManager},
    },
};
use crossterm::event::{self, KeyEvent};
use rand::random_range;

pub struct App {
    pub exit: bool,
    pub current_screen: CurrentScreen,
    pub menu_cursor: Option<usize>,
    pub direction: Direction,
    pub snake: Vec<(f64, f64)>,
    blocked: bool,
    pub field_size: (u32, u32),
    pub tick: bool,
    pub collectables: Vec<AnyCollectable>,
    pub game_speed: u32,
    pub round_time: u64,
    pub random_item_timer: u32,
    score_manager: ScoreManager,
}
impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            current_screen: CurrentScreen::Menu,
            menu_cursor: Some(0),
            direction: Direction::Up,
            snake: vec![(21.0, 20.0), (22.0, 20.0), (20.0, 20.0)],
            game_speed: 0,
            blocked: true,
            field_size: (50, 25),
            tick: false,
            collectables: vec![],
            round_time: 0,
            random_item_timer: 50,
            score_manager: ScoreManager::new(),
        }
    }

    pub fn get_score(&self) -> i32 {
        return self.snake.len() as i32 - 5;
    }

    pub fn get_highscores(&self) -> &Vec<Score> {
        return self.score_manager.get_scores();
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let mut terminal = ratatui::init();

        let (event_tx, event_rx) = mpsc::channel::<Event>();

        self.create_threads(event_tx);
        let mut counter = 3;
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
                        counter -= 1;
                        self.on_tick(&mut counter);
                        if counter == 0 {
                            counter = 3;
                            self.tick = !self.tick;
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
            CurrentScreen::Lost => match key_event.code {
                event::KeyCode::Esc => self.exit = true,
                event::KeyCode::Enter => {
                    self.current_screen = CurrentScreen::Menu;
                    self.menu_cursor = Some(0);
                }
                _ => {}
            },
            CurrentScreen::Menu => match key_event.code {
                event::KeyCode::Esc => self.exit = true,

                _ => self.handle_menu_input(&key_event),
            },
        }
    }

    fn start_game(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.snake = vec![
            (23.0, 20.0),
            (22.0, 20.0),
            (21.0, 20.0),
            (20.0, 20.0),
            (19.0, 20.0),
        ];
        self.direction = Direction::Right;
        self.menu_cursor = None;
        self.collectables = vec![];
        self.game_speed = 0;
        self.spawn_item(CollectableType::Apple);
        self.round_time = 0;
    }

    fn handle_menu_input(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            event::KeyCode::Up => {
                if let Some(cursor) = self.menu_cursor {
                    self.menu_cursor = Some(if cursor == 0 { 0 } else { cursor - 1 });
                } else {
                    self.menu_cursor = Some(0);
                }
            }
            event::KeyCode::Down => {
                if let Some(cursor) = self.menu_cursor {
                    self.menu_cursor = Some(if cursor == 1 { 1 } else { cursor + 1 });
                } else {
                    self.menu_cursor = Some(0);
                }
            }
            event::KeyCode::Enter => {
                if let Some(cursor) = self.menu_cursor {
                    match cursor {
                        0 => self.start_game(),
                        1 => self.exit = true,
                        _ => {}
                    }
                }
            }
            _ => {}
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

    fn on_tick(&mut self, counter: &mut u32) {
        match self.current_screen {
            CurrentScreen::Menu => {}
            CurrentScreen::Lost => {}
            CurrentScreen::Main => {
                self.round_time += 50;
                if *counter == self.game_speed || *counter == 0 {
                    self.game_update();
                    *counter = 3;
                }
            }
        }
    }

    pub fn spawn_item(&mut self, collectable_type: CollectableType) {
        let x: f64 = random_range(1..self.field_size.0 - 1) as f64;
        let y: f64 = random_range(1..self.field_size.1 - 1) as f64;
        let new_collectable = AnyCollectable::new(x, y, collectable_type.clone());
        if self.snake.contains(&new_collectable.get_position()) {
            self.spawn_item(collectable_type);
        } else {
            self.collectables.push(new_collectable);
        }
    }

    fn game_update(&mut self) {
        self.check_collectable_collision();
        self.update_snake_position();

        // Spawn Item
        if self.random_item_timer == 0 {
            let items = self.collectables.iter().filter(|ele| {
                if let AnyCollectable::Apple(_) = ele {
                    return false;
                }
                true
            });
            if items.count() <= 0 {
                self.spawn_item(CollectableType::from_random());
            }
            self.random_item_timer = rand::random_range(100..300);
        } else {
            self.random_item_timer -= 1;
        }

        // Items
        if self.has_snake_collision() {
            self.current_screen = CurrentScreen::Lost;
            self.score_manager
                .add_score(String::from("you"), self.get_score());
        }
    }

    fn check_collectable_collision(&mut self) {
        let mut i = 0;
        while i < self.collectables.len() {
            let mut collectable = self.collectables.remove(i);

            let mut should_remove = false;
            if self.snake[0] == collectable.get_position() {
                if collectable.on_collect(self) {
                    should_remove = true;
                }
            }

            if !should_remove && collectable.on_game_update(self) {
                should_remove = true;
            }

            if !should_remove {
                self.collectables.insert(i, collectable);
                i += 1;
            }
        }
    }

    pub fn increase_lenght(&mut self) {
        let (tail_x, tail_y) = self.snake[self.snake.len() - 1];
        let new_tail = match self.direction {
            Direction::Up => (tail_x, tail_y - 0.5),
            Direction::Down => (tail_x, tail_y + 0.5),
            Direction::Left => (tail_x + 1.0, tail_y),
            Direction::Right => (tail_x - 1.0, tail_y),
        };
        self.snake.push(new_tail);
    }

    fn update_snake_position(&mut self) {
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
    fn has_snake_collision(&self) -> bool {
        let head = self.snake[0];
        for segment in &self.snake[1..] {
            if head == *segment {
                return true;
            }
        }
        if head.0 > self.field_size.0 as f64
            || head.0 <= 0.0
            || head.1 >= self.field_size.1 as f64
            || head.1 < 0.0
        {
            return true;
        }

        false
    }
}
