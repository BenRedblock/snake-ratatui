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
    pub menu_cursor: Option<usize>,
    pub direction: Direction,
    pub snake: Vec<(f64, f64)>,
    pub tick_rate: u32,
    pub blocked: bool,
    pub field_size: (u32, u32),
    pub tick: bool,
}
