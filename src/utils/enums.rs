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

pub struct App {
    pub exit: bool,
    pub current_screen: CurrentScreen,
    pub direction: Direction,
    pub snake: Vec<(f64, f64)>,
    pub speed: u32,
    pub blocked: bool,
}
