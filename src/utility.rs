use std::{
    cell::Cell,
    fmt::Display,
    path::{Path, PathBuf},
    time::Duration,
};

use chrono::{DateTime, TimeDelta, Utc};
use crossterm::{
    cursor,
    style::Stylize,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use crate::{
    error::Result,
    strategem::{Strategem, StrategemClass, StrategemDifficulty},
};

pub struct GameTimer {
    initial_duration: Duration,
    game_over_time: DateTime<Utc>,
}

impl GameTimer {
    pub fn start_from(dur: Duration) -> Self {
        Self {
            initial_duration: dur,
            game_over_time: chrono::Utc::now() + dur,
        }
    }

    pub fn remaining(&self) -> TimeDelta {
        self.game_over_time - chrono::Utc::now()
    }

    pub fn is_over(&self) -> bool {
        self.game_over_time - chrono::Utc::now() <= TimeDelta::zero()
    }

    pub fn add(&mut self, dur: Duration) {
        self.game_over_time += dur;
    }

    pub fn reset(&mut self) {
        self.game_over_time = chrono::Utc::now() + self.initial_duration;
    }
}

impl Display for GameTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let step = self.initial_duration.as_secs() / 10;
        let remaining_steps = self.remaining().num_seconds() / step as i64 + 1;
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
            time_left.num_seconds(),
            (time_left.num_milliseconds() - time_left.num_seconds() * 1000) / 100
        )
    }
}

pub struct Penalty {
    counter: Cell<u32>,
    max_penalty: u32,
    step: u32,
}

impl Penalty {
    pub fn new(max_penalty: u32, step: u32) -> Self {
        Self {
            counter: Cell::new(0),
            max_penalty,
            step,
        }
    }

    pub fn apply(&self, on_done: impl FnOnce()) {
        if self.counter.get() < self.max_penalty {
            self.counter.set(self.counter.get() + self.step);
        } else {
            self.counter.set(0);
            on_done();
        }
    }
}

pub struct HideCursor;

impl HideCursor {
    pub fn hide() -> Result<HideCursorGuard> {
        std::io::stdout().execute(crossterm::cursor::Hide)?;
        Ok(HideCursorGuard)
    }
}

pub struct HideCursorGuard;

impl Drop for HideCursorGuard {
    fn drop(&mut self) {
        std::io::stdout().execute(crossterm::cursor::Show).unwrap();
    }
}

pub enum Multiplier {
    FirstTier,
    SecondTier,
    ThirdTier,
}

impl Multiplier {
    pub fn get(streak: usize) -> Multiplier {
        match streak {
            0..=5 => Multiplier::FirstTier,
            6..=20 => Multiplier::SecondTier,
            21.. => Multiplier::ThirdTier,
            _ => unreachable!(),
        }
    }
}

impl Display for Multiplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Multiplier::FirstTier => "  ".black(),
                Multiplier::SecondTier => "x2".green(),
                Multiplier::ThirdTier => "x3".dark_magenta(),
            }
        )
    }
}

pub struct ScreenWriter {
    lines_count: u16,
}

impl ScreenWriter {
    pub fn new() -> Self {
        Self { lines_count: 0 }
    }

    pub fn clear() -> Result<()> {
        std::io::stdout().execute(terminal::Clear(ClearType::FromCursorDown))?;
        Ok(())
    }
}

impl std::io::Write for ScreenWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.lines_count += 1;
        std::io::stdout().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        fmt.to_string()
            .chars()
            .filter(|ch| ch.eq(&'\n'))
            .for_each(|_| self.lines_count += 1);

        std::io::stdout().write_fmt(fmt)
    }
}

impl Drop for ScreenWriter {
    fn drop(&mut self) {
        std::io::stdout()
            .execute(cursor::MoveUp(self.lines_count))
            .unwrap();
    }
}

pub fn get_score_value(difficulty: &StrategemDifficulty, tier: Multiplier) -> usize {
    use Multiplier::*;
    use StrategemDifficulty::*;

    match (difficulty, tier) {
        (Easy, FirstTier) => 50,
        (Medium, FirstTier) => 75,
        (Hard, FirstTier) => 100,
        (Easy, SecondTier) => 100,
        (Medium, SecondTier) => 150,
        (Hard, SecondTier) => 200,
        (Easy, ThirdTier) => 125,
        (Medium, ThirdTier) => 190,
        (Hard, ThirdTier) => 250,
    }
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

pub fn get_app_data_dir() -> Result<PathBuf> {
    const GAME_DIR: &str = "strategem-hero";

    #[cfg(target_os = "windows")]
    {
        // C:\Users\<Account>\AppData\Roaming\<AppName>
        let appdata = std::env::var("APPDATA")?;
        let appdata_path = Path::new(&appdata);
        Ok(appdata_path.join(GAME_DIR))
    }

    #[cfg(target_os = "linux")]
    {
        // /home/<account>/.local/share/<AppName>
        let home = std::env::var("HOME")?;
        let homepath = Path::new(&home);
        Ok(homepath.join(".local").join("share").join(GAME_DIR))
    }

    #[cfg(target_os = "macos")]
    {
        // /Users/<Account>/Library/Application Support/<AppName>
        let home = std::env::var("HOME")?;
        let homepath = Path::new(&home);
        Ok(homepath
            .join("Library")
            .join("Application Support")
            .join(GAME_DIR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn windows_app_data_dir() {
        let username = std::env::var("USERNAME").unwrap();
        let path = get_app_data_dir();

        assert_eq!(
            Path::new("C:\\Users")
                .join(&username)
                .join("AppData")
                .join("Roaming")
                .join("strategem-hero"),
            path.unwrap()
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn linux_app_data_dir() {
        let homepath = std::env::var("HOME").unwrap();
        let path = get_app_data_dir();

        assert_eq!(
            Path::new(&homepath)
                .join(".local")
                .join("share")
                .join("strategem-hero"),
            path.unwrap()
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn macos_app_data_dir() {
        let homepath = std::env::var("HOME").unwrap();
        let path = get_app_data_dir();

        assert_eq!(
            Path::new(&homepath)
                .join("Library")
                .join("Application Support")
                .join("strategem-hero"),
            path.unwrap()
        );
    }
}
