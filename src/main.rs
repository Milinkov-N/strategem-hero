use app::App;
use error::Result;

mod app;
mod error;
mod event;
mod game;
mod storage;
mod strategem;
mod tui;
mod utility;

fn main() -> Result<()> {
    let mut app = App::init()?;
    app.handle_args()?;
    app.run()
}
