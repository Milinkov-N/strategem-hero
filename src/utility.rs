use std::{cell::Cell, fmt::Display, time::Duration};

use chrono::{DateTime, TimeDelta, Utc};

pub struct GameTimer {
    game_over_time: DateTime<Utc>,
}

impl GameTimer {
    pub fn start_from(dur: Duration) -> Self {
        Self {
            game_over_time: chrono::Utc::now() + dur,
        }
    }

    pub fn remaining(&self) -> TimeDelta {
        self.game_over_time - chrono::Utc::now()
    }

    pub fn is_over(&self) -> bool {
        self.game_over_time - chrono::Utc::now() <= TimeDelta::zero()
    }
}

impl Display for GameTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time_left = self.remaining();
        write!(
            f,
            "{}.{:0>3.2}s",
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
