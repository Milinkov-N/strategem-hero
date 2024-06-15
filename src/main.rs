use std::{path::PathBuf, time::Duration};

use event::Controls;

use crate::{
    error::Result,
    game::Game,
    storage::LeaderboardStorage,
    utility::{GameTimer, Penalty},
};

mod error;
mod event;
mod game;
mod storage;
mod strategem;
mod utility;

const VERSION: &str = "0.6";

fn setup_data_dir() -> Result<PathBuf> {
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

fn main() -> Result<()> {
    let mut store = match setup_data_dir() {
        Ok(dir) => LeaderboardStorage::open(dir.join("db.sqlite3")),
        Err(_e) => {
            println!("warning: unable to locate leaderboard data.");
            println!("warning: all progress will be lost after this session.");

            #[cfg(debug_assertions)]
            {
                eprintln!("{_e}");
            }

            LeaderboardStorage::open_in_memory()
        }
    };

    #[cfg(debug_assertions)]
    {
        store.drop_schema()?;
    }
    store.init_schema()?;
    store.seed_schema()?;

    if let Some(arg) = std::env::args().nth(1) {
        if arg.eq("leaderboard") {
            store
                .select_all()?
                .iter()
                .enumerate()
                .for_each(|(i, rec)| println!("  {}. {:<18} {}", i + 1, rec.nickname, rec.score));
            return Ok(());
        } else if arg.eq("delete-data") {
            store.close()?;
            let datadir = utility::get_app_data_dir()?;
            std::fs::remove_dir_all(datadir)?;
            println!("Deleted all game-related data successfully");
            return Ok(());
        }
    }

    let game_timer = GameTimer::start_from(Duration::from_secs(30));
    let penalty = Penalty::new(250, 10);
    let controls = if std::env::args().any(|arg| arg.eq("--wasd")) {
        Controls::wasd()
    } else {
        Controls::arrows()
    };
    let mut game = Game::new(store, game_timer, controls, penalty);

    Ok(game.run()?)
}
