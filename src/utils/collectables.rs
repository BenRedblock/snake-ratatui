use crate::game::App;

use super::enums::Direction;
#[derive(Clone)]
pub enum CollectableType {
    Apple,
    Speed,
    Reverse,
}

impl CollectableType {
    pub fn from_random() -> Self {
        if rand::random::<bool>() {
            CollectableType::Reverse
        } else {
            CollectableType::Speed
        }
    }
}

pub trait Collectable {
    fn new(x: f64, y: f64) -> Self;
    fn get_position(&self) -> (f64, f64);
    fn on_game_update(&mut self, _app: &mut App) -> bool {
        return false;
    }
    fn on_collect(&mut self, app: &mut App) -> bool;
    fn is_visible(&self) -> bool {
        return true;
    }
}

pub struct AppleCollectable {
    position: (f64, f64),
}

impl Collectable for AppleCollectable {
    fn new(x: f64, y: f64) -> Self {
        AppleCollectable { position: (x, y) }
    }
    fn get_position(&self) -> (f64, f64) {
        self.position
    }
    fn on_collect(&mut self, app: &mut App) -> bool {
        app.increase_lenght();
        app.spawn_item(CollectableType::Apple);
        return true;
    }
}

pub struct SpeedCollectable {
    position: (f64, f64),
    remaining_time: Option<u32>,
}

impl SpeedCollectable {
    pub fn get_remaining_time(&self) -> Option<u32> {
        self.remaining_time
    }
}

impl Collectable for SpeedCollectable {
    fn new(x: f64, y: f64) -> Self {
        SpeedCollectable {
            position: (x, y),
            remaining_time: None
        }
    }

    fn get_position(&self) -> (f64, f64) {
        self.position
    }

    fn on_game_update(&mut self, app: &mut App) -> bool {
        if let Some(remaining_time) = self.remaining_time {
            if remaining_time > 0 {
                self.remaining_time = Some(remaining_time - 1);
                return false;
            } else {
                app.game_speed -= 1;
                return true;
            }
        }
        false
    }

    fn on_collect(&mut self, app: &mut App) -> bool {
        app.game_speed = 1;
        self.remaining_time = Some(rand::random_range(50..200));
        false
    }

    fn is_visible(&self) -> bool {
        self.remaining_time.is_none()
    }
}

pub struct ReverseCollectable {
    position: (f64, f64),
}

impl Collectable for ReverseCollectable {
    fn new(x: f64, y: f64) -> Self {
        ReverseCollectable { position: (x, y) }
    }
    fn get_position(&self) -> (f64, f64) {
        self.position
    }
    fn on_collect(&mut self, app: &mut App) -> bool {
        app.snake.reverse();
        match app.direction {
            Direction::Up => app.direction = Direction::Down,
            Direction::Down => app.direction = Direction::Up,
            Direction::Left => app.direction = Direction::Right,
            Direction::Right => app.direction = Direction::Left,
        }
        true
    }
}

pub enum AnyCollectable {
    Apple(AppleCollectable),
    Speed(SpeedCollectable),
    Reverse(ReverseCollectable),
}

impl AnyCollectable {
    pub fn new(x: f64, y: f64, collectable_type: CollectableType) -> Self {
        match collectable_type {
            CollectableType::Apple => AnyCollectable::Apple(AppleCollectable::new(x, y)),
            CollectableType::Speed => AnyCollectable::Speed(SpeedCollectable::new(x, y)),
            CollectableType::Reverse => AnyCollectable::Reverse(ReverseCollectable::new(x, y)),
        }
    }

    pub fn get_position(&self) -> (f64, f64) {
        match self {
            AnyCollectable::Apple(a) => a.get_position(),
            AnyCollectable::Speed(s) => s.get_position(),
            AnyCollectable::Reverse(r) => r.get_position(),
        }
    }

    pub fn on_game_update(&mut self, app: &mut App) -> bool {
        match self {
            AnyCollectable::Apple(a) => a.on_game_update(app),
            AnyCollectable::Speed(s) => s.on_game_update(app),
            AnyCollectable::Reverse(r) => r.on_game_update(app),
        }
    }

    pub fn on_collect(&mut self, app: &mut App) -> bool {
        match self {
            AnyCollectable::Apple(a) => a.on_collect(app),
            AnyCollectable::Speed(s) => s.on_collect(app),
            AnyCollectable::Reverse(r) => r.on_collect(app),
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            AnyCollectable::Apple(a) => a.is_visible(),
            AnyCollectable::Speed(s) => s.is_visible(),
            AnyCollectable::Reverse(r) => r.is_visible(),
        }
    }
}
