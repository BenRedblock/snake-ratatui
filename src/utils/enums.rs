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

pub enum CollectableType {
    Apple,
}

pub struct Collectable {
    pub position: (f64, f64),
    pub collectable_type: CollectableType,
}

pub struct App {
    pub exit: bool,
    pub current_screen: CurrentScreen,
    pub menu_cursor: Option<usize>,
    pub direction: Direction,
    pub snake: Vec<(f64, f64)>,
    pub blocked: bool,
    pub field_size: (u32, u32),
    pub tick: bool,
    pub collectables: Vec<Collectable>,
    pub game_speed: u32,
    pub round_time: u64,
}
