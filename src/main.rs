use std::time::Duration;

use crate::{
    error::Result,
    event::Controls,
    game::Game,
    storage::Leaderboard,
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

    screenln!("{LOGO}")?;

    'game_loop: loop {
        match tui::menu::Menu::builder()
            .add_item("Start Game")
            .add_item("Leaderboard")
            .add_item("Upgrades")
            .add_item("Delete Data")
            .add_item("Quit")
            .build()
            .exec()?
        {
            Some(0) => {
                let secs = if cfg!(debug_assertions) {
                    Duration::from_secs(10)
                } else {
                    Duration::from_secs(30)
                };
                let game_timer = GameTimer::start_from(secs);
                let penalty = Penalty::new(250, 10);
                let controls = if std::env::args().any(|arg| arg.eq("--wasd")) {
                    Controls::wasd()
                } else {
                    Controls::arrows()
                };
                let mut game = Game::new(leaderboard, game_timer, controls, penalty);
                game.run()?;
                break 'game_loop;
            }

            Some(1) => {
                leaderboard
                    .sorted_vec()
                    .iter()
                    .enumerate()
                    .for_each(|(i, rec)| print!("  {}. {:<18} {}\r\n", i + 1, rec.0, rec.1));
            }

            Some(2) => {
                struct UpgradeItem<'a> {
                    name: &'a str,
                    desc: &'a str,
                    price: u32,
                    purchased: bool,
                }

                impl<'a> UpgradeItem<'a> {
                    pub fn new(name: &'a str, desc: &'a str, price: u32) -> Self {
                        Self {
                            name,
                            desc,
                            price,
                            purchased: false,
                        }
                    }
                }

                impl<'a> std::fmt::Display for UpgradeItem<'a> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(
                            f,
                            "{:<32}[{}]\n\t{}",
                            self.name,
                            if self.purchased {
                                "Purchased".to_string()
                            } else {
                                format!("{} DP", self.price)
                            },
                            self.desc
                        )
                    }
                }

                let shrapnel = UpgradeItem::new(
                    "Exploding Shrapnel",
                    "increases all strategem rewards by +100 Democracy Points",
                    2500,
                );
                let lvc = UpgradeItem::new(
                    "Liquid-Ventilated Cockpit",
                    "reduces time penalty after failed strategem",
                    3000,
                );
                let tsu = UpgradeItem::new(
                    "Targeting Software Upgrade",
                    "increases time reward after successfully completing strategem by +0.5s",
                    5000,
                );

                screenln!("Upgrades:")?;
                let _ = tui::menu::Menu::builder()
                    .add_item(shrapnel)
                    .add_item(lvc)
                    .add_item(tsu)
                    .build()
                    .exec()?;
            }

            Some(3) => {
                if let Some(datadir) = utility::data_dir()?.parent() {
                    std::fs::remove_dir_all(datadir)?;
                }
                println!("Deleted all game-related data successfully");
            }

            _ => break 'game_loop,
        }
    }

    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
