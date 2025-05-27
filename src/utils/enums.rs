pub enum CurrentScreen {
    Main,
    Menu,
    Lost,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Event {
    Input(crossterm::event::KeyEvent),
    GameTick,
}
#[derive(Clone)]
pub enum CollectableType {
    Apple,
    Speed,
}
