use super::enums::CollectableType;

pub struct Collectable {
    pub position: (f64, f64),
    pub collectable_type: CollectableType,
}
impl Collectable {
    pub fn new(x: f64, y: f64) -> Self {
        Collectable {
            position: (x, y),
            collectable_type: CollectableType::Apple,
        }
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }
}
