use std::{io::Write, path::PathBuf, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use event::Controls;
use utility::ScreenWriter;

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

const VERSION: &str = "0.7";

const LOGO: &str = r#"     _             _                                  _                    
    | |           | |                                | |                   
 ___| |_ _ __ __ _| |_ ___  __ _  ___ _ __ ___ ______| |__   ___ _ __ ___  
/ __| __| '__/ _` | __/ _ \/ _` |/ _ \ '_ ` _ \______| '_ \ / _ \ '__/ _ \ 
\__ \ |_| | | (_| | ||  __/ (_| |  __/ | | | | |     | | | |  __/ | | (_) |
|___/\__|_|  \__,_|\__\___|\__, |\___|_| |_| |_|     |_| |_|\___|_|  \___/ 
                            __/ |                                          
                           |___/ "#;

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

    let mut screen = ScreenWriter::new();
    writeln!(screen, "{LOGO}")?;
    writeln!(screen, "Press key in [] brackets to choose an action.")?;
    writeln!(screen, "1. [S]tart Game")?;
    writeln!(screen, "2. [L]eaderboard")?;
    writeln!(screen, "3. [D]elete Data")?;
    writeln!(screen, "4. [Q]uit")?;

    while let Event::Key(ev) = crossterm::event::read()? {
        match ev {
            KeyEvent {
                code: KeyCode::Char('s'),
                kind: KeyEventKind::Press,
                ..
            } => {
                let game_timer = GameTimer::start_from(Duration::from_secs(30));
                let penalty = Penalty::new(250, 10);
                let controls = if std::env::args().any(|arg| arg.eq("--wasd")) {
                    Controls::wasd()
                } else {
                    Controls::arrows()
                };
                let mut game = Game::new(store, game_timer, controls, penalty);
                drop(screen);
                ScreenWriter::clear()?;
                game.run()?;
                break;
            }

            KeyEvent {
                code: KeyCode::Char('l'),
                kind: KeyEventKind::Press,
                ..
            } => {
                drop(screen);
                ScreenWriter::clear()?;
                store.select_all()?.iter().enumerate().for_each(|(i, rec)| {
                    println!("  {}. {:<18} {}", i + 1, rec.nickname, rec.score)
                });
                break;
            }

            KeyEvent {
                code: KeyCode::Char('d'),
                kind: KeyEventKind::Press,
                ..
            } => {
                drop(screen);
                ScreenWriter::clear()?;
                store.close()?;
                let datadir = utility::get_app_data_dir()?;
                std::fs::remove_dir_all(datadir)?;
                println!("Deleted all game-related data successfully");
                break;
            }

            KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            } => break,

            _ => (),
        }
    }

    Ok(())
}
