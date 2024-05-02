use std::time::Duration;

use crate::{
    game::Game,
    utility::{GameTimer, Penalty},
};

mod event;
mod game;
mod strategem;
mod utility;

fn main() -> std::io::Result<()> {
    let game_timer = GameTimer::start_from(Duration::from_secs(30));
    let penalty = Penalty::new(250, 10);
    let mut game = Game::new(game_timer, penalty);

    game.run()
}
