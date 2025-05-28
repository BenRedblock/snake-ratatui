use crate::game::App;

use super::enums::CollectableType;

pub trait Collectable {
    fn new(x: f64, y: f64, collectable_type: CollectableType) -> Self;
    fn get_position(&self) -> (f64, f64);
    fn on_game_update(&mut self, _app: &mut App) -> bool {
        return false;
    }
    fn on_collect(&mut self, app: &mut App) -> bool;
    fn is_visible(&self) -> bool {
        return true;
    }
    fn get_type(&self) -> &CollectableType;
}

pub struct AppleCollectable {
    position: (f64, f64),
    collectable_type: CollectableType,
}

impl Collectable for AppleCollectable {
    fn new(x: f64, y: f64, collectable_type: CollectableType) -> Self {
        AppleCollectable {
            position: (x, y),
            collectable_type,
        }
    }
    fn get_position(&self) -> (f64, f64) {
        self.position
    }
    fn on_collect(&mut self, app: &mut App) -> bool {
        app.increase_lenght();
        app.spawn_item(CollectableType::Apple);
        return true;
    }
    fn get_type(&self) -> &CollectableType {
        &self.collectable_type
    }
}

pub struct SpeedCollectable {
    position: (f64, f64),
    active_time: u32,
    active: bool,
    collectable_type: CollectableType,
}

impl Collectable for SpeedCollectable {
    fn new(x: f64, y: f64, collectable_type: CollectableType) -> Self {
        SpeedCollectable {
            position: (x, y),
            active_time: rand::random_range(10..50),
            active: false,
            collectable_type,
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
    fn get_type(&self) -> &CollectableType {
        &self.collectable_type
    }
}

pub enum AnyCollectable {
    Apple(AppleCollectable),
    Speed(SpeedCollectable),
}

impl Collectable for AnyCollectable {
    fn new(x: f64, y: f64, collectable_type: CollectableType) -> Self {
        match collectable_type {
            CollectableType::Apple => {
                AnyCollectable::Apple(AppleCollectable::new(x, y, collectable_type))
            }
            CollectableType::Speed => {
                AnyCollectable::Speed(SpeedCollectable::new(x, y, collectable_type))
            }
        }
    }

    fn get_position(&self) -> (f64, f64) {
        match self {
            AnyCollectable::Apple(a) => a.get_position(),
            AnyCollectable::Speed(s) => s.get_position(),
        }
    }

    fn on_game_update(&mut self, app: &mut App) -> bool {
        match self {
            AnyCollectable::Apple(a) => a.on_game_update(app),
            AnyCollectable::Speed(s) => s.on_game_update(app),
        }
    }

    fn on_collect(&mut self, app: &mut App) -> bool {
        match self {
            AnyCollectable::Apple(a) => a.on_collect(app),
            AnyCollectable::Speed(s) => s.on_collect(app),
        }
    }

    fn is_visible(&self) -> bool {
        match self {
            AnyCollectable::Apple(a) => a.is_visible(),
            AnyCollectable::Speed(s) => s.is_visible(),
        }
    }

    fn get_type(&self) -> &CollectableType {
        match self {
            AnyCollectable::Apple(a) => a.get_type(),
            AnyCollectable::Speed(s) => s.get_type(),
        }
    }
}
