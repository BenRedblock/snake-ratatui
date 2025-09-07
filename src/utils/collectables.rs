use crate::{
    game::App,
    utils::helpers::{get_direction_from_vector, get_directionvector_from_snake},
};

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
    active_time: u32,
    active: bool,
}

impl Collectable for SpeedCollectable {
    fn new(x: f64, y: f64) -> Self {
        SpeedCollectable {
            position: (x, y),
            active_time: rand::random_range(50..200),
            active: false,
        }
    }

    fn get_position(&self) -> (f64, f64) {
        self.position
    }

    fn on_game_update(&mut self, app: &mut App) -> bool {
        if self.active {
            self.active_time -= 1;
            if self.active_time <= 0 {
                app.game_speed = 0;
                self.active = false;
                return true;
            }
        }
        false
    }

    fn on_collect(&mut self, app: &mut App) -> bool {
        app.game_speed = 1;
        self.active = true;
        false
    }

    fn is_visible(&self) -> bool {
        !self.active
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
        let vector: (f64, f64) = get_directionvector_from_snake(&app.snake);
        app.direction = get_direction_from_vector(&vector);
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
