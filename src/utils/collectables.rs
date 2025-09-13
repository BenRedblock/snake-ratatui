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
    pub fn from_random_special() -> Self {
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
    fn on_second_update(&mut self) {}
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
            remaining_time: None,
        }
    }

    fn get_position(&self) -> (f64, f64) {
        self.position
    }

    fn on_game_update(&mut self, app: &mut App) -> bool {
        if let Some(remaining_time) = self.remaining_time {
            if remaining_time > 0 {
                return false;
            } else {
                app.game_speed -= 1;
                return true;
            }
        }
        false
    }

    fn on_collect(&mut self, app: &mut App) -> bool {
        app.game_speed += 1;
        self.remaining_time = Some(rand::random_range(10..20));
        false
    }

    fn on_second_update(&mut self) {
        if let Some(remaining_time) = self.remaining_time {
            if remaining_time > 0 {
                self.remaining_time = Some(remaining_time - 1);
            }
        }
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

    /// Should be called on every game update
    ///
    /// Returns true if the item should be removed from the game
    pub fn on_game_update(&mut self, app: &mut App) -> bool {
        match self {
            AnyCollectable::Apple(a) => a.on_game_update(app),
            AnyCollectable::Speed(s) => s.on_game_update(app),
            AnyCollectable::Reverse(r) => r.on_game_update(app),
        }
    }

    /// Should be called every second
    pub fn on_second_update(&mut self) {
        match self {
            AnyCollectable::Apple(a) => a.on_second_update(),
            AnyCollectable::Speed(s) => s.on_second_update(),
            AnyCollectable::Reverse(r) => r.on_second_update(),
        }
    }

    /// Should be called when the player collects the item
    ///
    /// Returns true if the item should be removed from the game
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
