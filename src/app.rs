use std::time::Duration;

use crate::{
    error::Result,
    event::Controls,
    game::Game,
    screenln,
    storage::{Leaderboard, PlayerData, Storage, UpgradeItem, Upgrades},
    utility::{GameTimer, Penalty},
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
    player: PlayerData,
    leaderboard: Leaderboard,
    is_running: bool,
    upgrades: Upgrades,
}

impl App {
    pub fn init() -> Result<Self> {
        crate::utility::setup_data_dir()?;

        let player = PlayerData::open()?;
        let leaderboard = Leaderboard::open()?;
        let upgrades = Upgrades::open()?;

        Ok(Self {
            screen: Default::default(),
            player,
            leaderboard,
            is_running: true,
            upgrades,
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

            self.is_running = false;
        }

        Ok(())
    }

    pub fn run(mut self) -> Result<()> {
        let _guard = crate::tui::HideCursor::hide()?;

        crossterm::terminal::enable_raw_mode()?;

        'main_loop: loop {
            let _sc = crate::tui::screen::cleaner();

            if !self.is_running {
                break 'main_loop;
            }

            match self.screen {
                Screen::Main => self.render_main()?,
                Screen::Game => return self.render_game(),
                Screen::Leaderboard => self.render_leaderboard()?,
                Screen::Upgrades => self.render_upgrades()?,
                Screen::DeleteData => return self.render_delete_data(),
            }
        }
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    fn render_main(&mut self) -> Result<()> {
        screenln!("{LOGO}")?;
        match crate::tui::menu::Menu::builder()
            .add_item("Start Game")
            .add_item("Leaderboard")
            .add_item("Upgrades")
            .add_item("Delete Data")
            .add_item("Quit")
            .build()
            .exec("Main Menu")?
        {
            Some(0) => self.screen.set_game(),
            Some(1) => self.screen.set_leaderboard(),
            Some(2) => self.screen.set_upgrades(),
            Some(3) => self.screen.set_delete_data(),
            _ => self.is_running = false,
        }

        Ok(())
    }

    fn render_game(self) -> Result<()> {
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
        let mut game = Game::new(self.player, self.leaderboard, game_timer, controls, penalty);
        game.run()
    }

    fn render_leaderboard(&mut self) -> Result<()> {
        screenln!("{LOGO}")?;
        screenln!("----[ Leaderboard ]----------")?;
        self.leaderboard
            .sorted_vec()
            .iter()
            .enumerate()
            .for_each(|(i, rec)| screenln!("  {}. {:<18} {}", i + 1, rec.0, rec.1).unwrap());

        crate::tui::confirm_quit(Some("return to main menu"))?;
        self.screen.set_main();

        Ok(())
    }

    fn render_upgrades(&mut self) -> Result<()> {
        screenln!("{LOGO}")?;
        let handle_purchase = |player: &mut PlayerData, upgrade: &mut UpgradeItem| -> Result<()> {
            if upgrade.is_purchased() {
                return Ok(());
            }

            if player.wallet() >= upgrade.price() {
                player.write_off_from_wallet(upgrade.price());
                upgrade.set_purchased();
                player.save()?;
            }

            Ok(())
        };

        match crate::tui::menu::Menu::builder()
            .add_item(&self.upgrades[0])
            .add_item(&self.upgrades[1])
            .add_item(&self.upgrades[2])
            .build()
            .exec(&format!("Upgrades (You have {} DP)", self.player.wallet()))?
        {
            Some(0) => handle_purchase(&mut self.player, &mut self.upgrades[0])?,
            Some(1) => handle_purchase(&mut self.player, &mut self.upgrades[1])?,
            Some(2) => handle_purchase(&mut self.player, &mut self.upgrades[2])?,

            None => self.screen.set_main(),
            _ => todo!(),
        }

        self.upgrades.save()
    }

    fn render_delete_data(self) -> Result<()> {
        if let Some(datadir) = crate::utility::data_dir()?.parent() {
            std::fs::remove_dir_all(datadir)?;
        }

        screenln!("Deleted all game-related data successfully")?;
        crate::tui::confirm_quit(Some("return to main menu"))?;

        Ok(())
    }
}
