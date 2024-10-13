use std::time::Duration;

use crate::{
    error::Result,
    event::Controls,
    game::Game,
    screenln,
    storage::{Leaderboard, UpgradeItem},
    utility::GameTimer,
    utility::Penalty,
};

pub const LOGO: &str = r#"     _             _                                  _                    
    | |           | |                                | |                   
 ___| |_ _ __ __ _| |_ ___  __ _  ___ _ __ ___ ______| |__   ___ _ __ ___  
/ __| __| '__/ _` | __/ _ \/ _` |/ _ \ '_ ` _ \______| '_ \ / _ \ '__/ _ \ 
\__ \ |_| | | (_| | ||  __/ (_| |  __/ | | | | |     | | | |  __/ | | (_) |
|___/\__|_|  \__,_|\__\___|\__, |\___|_| |_| |_|     |_| |_|\___|_|  \___/ 
                            __/ |                                          
                           |___/ "#;

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Game,
    Leaderboard,
    Upgrades,
    DeleteData,
}

impl Screen {
    pub fn set_main(&mut self) {
        *self = Self::Main;
    }

    pub fn set_game(&mut self) {
        *self = Self::Game;
    }

    pub fn set_leaderboard(&mut self) {
        *self = Self::Leaderboard;
    }

    pub fn set_upgrades(&mut self) {
        *self = Self::Upgrades;
    }

    pub fn set_delete_data(&mut self) {
        *self = Self::DeleteData;
    }
}

pub struct App {
    screen: Screen,
    leaderboard: Leaderboard,
}

impl App {
    pub fn init() -> Result<Self> {
        crate::utility::setup_data_dir()?;
        let leaderboard = Leaderboard::open()?;

        Ok(Self {
            screen: Default::default(),
            leaderboard,
        })
    }

    pub fn handle_args(&mut self) -> Result<()> {
        if let Some(arg) = std::env::args().nth(1) {
            if arg.eq("leaderboard") {
                self.leaderboard
                    .sorted_vec()
                    .iter()
                    .enumerate()
                    .for_each(|(i, rec)| println!("  {}. {:<18} {}", i + 1, rec.0, rec.1));
                return Ok(());
            } else if arg.eq("delete-data") {
                if let Some(datadir) = crate::utility::data_dir()?.parent() {
                    std::fs::remove_dir_all(datadir)?;
                }
                println!("Deleted all game-related data successfully");
                return Ok(());
            }
        }

        Ok(())
    }

    pub fn run(mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        let mut shrapnel = UpgradeItem::new(
            "Exploding Shrapnel",
            "increases all strategem rewards by +100 Democracy Points",
            2500,
        );
        let mut lvc = UpgradeItem::new(
            "Liquid-Ventilated Cockpit",
            "reduces time penalty after failed strategem",
            3000,
        );
        let mut tsu = UpgradeItem::new(
            "Targeting Software Upgrade",
            "increases time reward after successfully completing strategem by +0.5s",
            5000,
        );

        'main_loop: loop {
            let _sc = crate::tui::screen::cleaner();

            match self.screen {
                Screen::Main => {
                    screenln!("{LOGO}")?;
                    match crate::tui::menu::Menu::builder()
                        .add_item("Start Game")
                        .add_item("Leaderboard")
                        .add_item("Upgrades")
                        .add_item("Delete Data")
                        .add_item("Quit")
                        .build()
                        .exec("Main Menu:")?
                    {
                        Some(0) => self.screen.set_game(),
                        Some(1) => self.screen.set_leaderboard(),
                        Some(2) => self.screen.set_upgrades(),
                        Some(3) => self.screen.set_delete_data(),
                        _ => break 'main_loop,
                    }
                }

                Screen::Game => {
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
                    let mut game = Game::new(self.leaderboard, game_timer, controls, penalty);
                    game.run()?;
                    break 'main_loop;
                }

                Screen::Leaderboard => {
                    self.leaderboard
                        .sorted_vec()
                        .iter()
                        .enumerate()
                        .for_each(|(i, rec)| {
                            screenln!("  {}. {:<18} {}", i + 1, rec.0, rec.1).unwrap()
                        });

                    crate::tui::confirm_action()?;
                    self.screen.set_main();
                }

                Screen::Upgrades => {
                    screenln!("{LOGO}")?;
                    match crate::tui::menu::Menu::builder()
                        .add_item(&shrapnel)
                        .add_item(&lvc)
                        .add_item(&tsu)
                        .build()
                        .exec("Upgrades:")?
                    {
                        Some(0) => shrapnel.set_purchased(),
                        Some(1) => lvc.set_purchased(),
                        Some(2) => tsu.set_purchased(),

                        None => self.screen.set_main(),
                        _ => todo!(),
                    }
                }

                Screen::DeleteData => {
                    if let Some(datadir) = crate::utility::data_dir()?.parent() {
                        std::fs::remove_dir_all(datadir)?;
                    }
                    screenln!("Deleted all game-related data successfully")?;
                    crate::tui::confirm_action()?;
                }
            }
        }
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }
}
