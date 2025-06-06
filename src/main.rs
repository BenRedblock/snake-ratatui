use std::io;

use game::App;

mod game;
mod ui;
mod utils;

fn main() -> io::Result<()> {
    let app_result = App::new().run();
    return app_result;
}
