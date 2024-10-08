use std::io::Write;
use std::time::Duration;

use crate::{
    error::Result,
    event::Controls,
    game::Game,
    storage::Leaderboard,
    tui::ScreenWriter,
    utility::{GameTimer, Penalty},
};

mod error;
mod event;
mod game;
mod storage;
mod strategem;
mod tui;
mod utility;

const LOGO: &str = r#"     _             _                                  _                    
    | |           | |                                | |                   
 ___| |_ _ __ __ _| |_ ___  __ _  ___ _ __ ___ ______| |__   ___ _ __ ___  
/ __| __| '__/ _` | __/ _ \/ _` |/ _ \ '_ ` _ \______| '_ \ / _ \ '__/ _ \ 
\__ \ |_| | | (_| | ||  __/ (_| |  __/ | | | | |     | | | |  __/ | | (_) |
|___/\__|_|  \__,_|\__\___|\__, |\___|_| |_| |_|     |_| |_|\___|_|  \___/ 
                            __/ |                                          
                           |___/ "#;

fn setup_data_dir() -> Result<()> {
    let datadir = utility::data_dir()?;
    if !datadir.exists() {
        std::fs::create_dir(&datadir)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    setup_data_dir()?;
    let leaderboard = Leaderboard::open()?;

    if let Some(arg) = std::env::args().nth(1) {
        if arg.eq("leaderboard") {
            leaderboard
                .sorted_vec()
                .iter()
                .enumerate()
                .for_each(|(i, rec)| println!("  {}. {:<18} {}", i + 1, rec.0, rec.1));
            return Ok(());
        } else if arg.eq("delete-data") {
            if let Some(datadir) = utility::data_dir()?.parent() {
                std::fs::remove_dir_all(datadir)?;
            }
            println!("Deleted all game-related data successfully");
            return Ok(());
        }
    }

    crossterm::terminal::enable_raw_mode()?;

    let mut screen = ScreenWriter::new();
    writeln!(screen, "{LOGO}")?;

    match tui::select_from_list(
        Some(screen),
        Some("Press key in [] brackets to choose an action."),
        vec![
            ("[S]tart Game", 's'),
            ("[L]eaderboard", 'l'),
            ("[D]elete Data", 'd'),
            ("[Q]uit", 'q'),
        ],
    )? {
        0 => {
            let game_timer = GameTimer::start_from(Duration::from_secs(10));
            let penalty = Penalty::new(250, 10);
            let controls = if std::env::args().any(|arg| arg.eq("--wasd")) {
                Controls::wasd()
            } else {
                Controls::arrows()
            };
            let mut game = Game::new(leaderboard, game_timer, controls, penalty);
            game.run()?;
        }

        1 => {
            leaderboard
                .sorted_vec()
                .iter()
                .enumerate()
                .for_each(|(i, rec)| print!("  {}. {:<18} {}\r\n", i + 1, rec.0, rec.1));
        }

        2 => {
            if let Some(datadir) = utility::data_dir()?.parent() {
                std::fs::remove_dir_all(datadir)?;
            }
            println!("Deleted all game-related data successfully");
        }

        3 => (),

        _ => (),
    }

    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
