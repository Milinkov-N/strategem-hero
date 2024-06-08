use std::{error::Error, path::PathBuf, time::Duration};

use crate::{
    game::Game,
    storage::LeaderboardStorage,
    utility::{GameTimer, Penalty},
};

mod event;
mod game;
mod storage;
mod strategem;
mod utility;

const VERSION: &str = "0.6";

fn setup_data_dir() -> Result<PathBuf, Box<dyn Error>> {
    let datadir = utility::get_app_data_dir()?;
    if !datadir.exists() {
        std::fs::create_dir(&datadir)?;
    }

    let datadir = datadir.join(VERSION);
    if !datadir.exists() {
        std::fs::create_dir(&datadir)?;
    }

    Ok(datadir)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut store = match setup_data_dir() {
        Ok(dir) => LeaderboardStorage::open(dir.join("db.sqlite3")),
        Err(e) => {
            println!("warning: unable to locate leaderboard data.");
            println!("warning: all progress will be lost after this session.");

            #[cfg(debug_assertions)]
            {
                eprintln!("{e}");
            }

            LeaderboardStorage::open_in_memory()
        }
    };

    let game_timer = GameTimer::start_from(Duration::from_secs(30));
    let penalty = Penalty::new(250, 10);
    let mut game = Game::new(game_timer, penalty);

    #[cfg(debug_assertions)]
    {
        store.delete_schema()?;
    }
    store.init_schema()?;

    store.insert_or_update(450)?;

    let (name, score) = store.select_best_score()?;
    println!("Best score {score} by {name}");

    Ok(game.run()?)
}
