use std::{
    fmt::Display,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use crossterm::style::Stylize;

use crate::{
    error::Result,
    strategem::{Strategem, StrategemClass, StrategemDifficulty},
};

const VERSION: &str = "0.8";

pub struct GameTimer {
    initial_duration: Duration,
    game_over_time: std::time::Instant,
}

impl GameTimer {
    pub fn start_from(dur: Duration) -> Self {
        Self {
            initial_duration: dur,
            game_over_time: std::time::Instant::now() + dur,
        }
    }

    pub fn remaining(&self) -> Duration {
        self.game_over_time - Instant::now()
    }

    pub fn is_over(&self) -> bool {
        self.game_over_time - Instant::now() <= Duration::ZERO
    }

    pub fn add(&mut self, dur: Duration) {
        self.game_over_time += dur;
    }

    pub fn sub(&mut self, dur: Duration) {
        self.game_over_time -= dur;
    }

    pub fn reset(&mut self) {
        self.game_over_time = Instant::now() + self.initial_duration;
    }
}

impl Display for GameTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let step = self.initial_duration.as_secs() / 10;
        let remaining_steps = self.remaining().as_secs() / step + 1;
        let time_left = self.remaining();
        let steps_str = "#".repeat(remaining_steps.min(10) as usize);

        write!(
            f,
            "[{}{}] {:02}.{:.1}s",
            match remaining_steps {
                1..=2 => steps_str.red(),
                3..=5 => steps_str.dark_yellow(),
                _ => steps_str.green(),
            },
            " ".repeat(10 - remaining_steps.min(10) as usize),
            time_left.as_secs(),
            time_left.subsec_millis() / 100
        )
    }
}

#[derive(Default, Clone, Copy)]
pub enum FreezeState {
    #[default]
    NotActivated,
    Freezed,
    Completed,
}

pub struct InputFreeze {
    counter: u32,
    frames: u32,
    state: FreezeState,
}

impl InputFreeze {
    pub fn new(frames: u32) -> Self {
        Self {
            counter: 0,
            frames,
            state: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.state = FreezeState::NotActivated;
        self.counter = 0;
    }

    pub fn ping(&mut self) -> FreezeState {
        match self.state {
            FreezeState::NotActivated => {
                self.counter += 1;
                self.state = FreezeState::Freezed;
            }

            FreezeState::Freezed => {
                if self.counter < self.frames {
                    self.counter += 1;
                } else {
                    self.counter = 0;
                    self.state = FreezeState::NotActivated;
                    return FreezeState::Completed;
                }
            }

            _ => (),
        }

        self.state
    }
}

pub enum Multiplier {
    First,
    Second,
    Third,
}

impl Multiplier {
    pub fn get(streak: usize) -> Multiplier {
        match streak {
            0..=5 => Multiplier::First,
            6..=20 => Multiplier::Second,
            _ => Multiplier::Third,
        }
    }
}

impl Display for Multiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Multiplier::First => "  ".black(),
                Multiplier::Second => "x2".green(),
                Multiplier::Third => "x3".dark_magenta(),
            }
        )
    }
}

pub fn get_score_value(difficulty: &StrategemDifficulty, tier: Multiplier, bonus: usize) -> usize {
    use Multiplier::*;
    use StrategemDifficulty::*;

    let base = match (difficulty, tier) {
        (Easy, First) => 50,
        (Medium, First) => 75,
        (Hard, First) => 100,
        (Easy, Second) => 100,
        (Medium, Second) => 150,
        (Hard, Second) => 200,
        (Easy, Third) => 125,
        (Medium, Third) => 190,
        (Hard, Third) => 250,
    };

    base + bonus
}

pub fn format_strategem_name(strategem: &Strategem) -> String {
    match strategem.class() {
        StrategemClass::Supply => {
            format!(
                "|{}{}{}|",
                " ".on_cyan(),
                strategem.name().on_cyan().black(),
                " ".on_cyan(),
            )
        }
        StrategemClass::Mission => {
            format!(
                "|{}{}{}|",
                " ".on_yellow(),
                strategem.name().on_yellow().black(),
                " ".on_yellow(),
            )
        }
        StrategemClass::Defensive => {
            format!(
                "|{}{}{}|",
                " ".on_green(),
                strategem.name().on_green().white(),
                " ".on_green()
            )
        }
        StrategemClass::Offensive => {
            format!(
                "|{}{}{}|",
                " ".on_red(),
                strategem.name().on_red().white(),
                " ".on_red()
            )
        }
    }
}

pub fn data_dir() -> Result<PathBuf> {
    const GAME_DIR: &str = "strategem-hero";

    #[cfg(target_os = "windows")]
    {
        // C:\Users\<Account>\AppData\Roaming\<AppName>
        let appdata = std::env::var("APPDATA")?;
        let appdata_path = Path::new(&appdata);
        Ok(appdata_path.join(GAME_DIR).join(VERSION))
    }

    #[cfg(target_os = "linux")]
    {
        // /home/<account>/.local/share/<AppName>
        let home = std::env::var("HOME")?;
        let homepath = Path::new(&home);
        Ok(homepath
            .join(".local")
            .join("share")
            .join(GAME_DIR)
            .join(VERSION))
    }

    #[cfg(target_os = "macos")]
    {
        // /Users/<Account>/Library/Application Support/<AppName>
        let home = std::env::var("HOME")?;
        let homepath = Path::new(&home);
        Ok(homepath
            .join("Library")
            .join("Application Support")
            .join(GAME_DIR)
            .join(VERSION))
    }
}

pub fn setup_data_dir() -> Result<()> {
    let datadir = data_dir()?;
    if !datadir.exists() {
        std::fs::create_dir_all(&datadir)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn windows_app_data_dir() {
        let username = std::env::var("USERNAME").unwrap();
        let path = data_dir();

        assert_eq!(
            Path::new("C:\\Users")
                .join(&username)
                .join("AppData")
                .join("Roaming")
                .join("strategem-hero")
                .join(VERSION),
            path.unwrap()
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn linux_app_data_dir() {
        let homepath = std::env::var("HOME").unwrap();
        let path = data_dir();

        assert_eq!(
            Path::new(&homepath)
                .join(".local")
                .join("share")
                .join("strategem-hero")
                .join(VERSION),
            path.unwrap()
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn macos_app_data_dir() {
        let homepath = std::env::var("HOME").unwrap();
        let path = data_dir();

        assert_eq!(
            Path::new(&homepath)
                .join("Library")
                .join("Application Support")
                .join("strategem-hero")
                .join(VERSION),
            path.unwrap()
        );
    }
}
