use std::{cell::Cell, fmt::Display, time::Duration};

use chrono::{DateTime, TimeDelta, Utc};
use crossterm::ExecutableCommand;

pub struct GameTimer {
    inital_duration: Duration,
    game_over_time: DateTime<Utc>,
}

impl GameTimer {
    pub fn start_from(dur: Duration) -> Self {
        Self {
            inital_duration: dur,
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
        self.game_over_time = chrono::Utc::now() + self.inital_duration;
    }
}

impl Display for GameTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time_left = self.remaining();
        write!(
            f,
            "{:02}.{:0>3.2}s",
            time_left.num_seconds(),
            time_left.num_milliseconds() - time_left.num_seconds() * 1000
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
    pub fn hide() -> std::io::Result<HideCursorGuard> {
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
