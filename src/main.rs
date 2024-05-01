use std::time::Duration;

use chrono::{DateTime, TimeDelta, Utc};
use crossterm::ExecutableCommand;

use event::Key;

mod event;
mod strategem;

struct GameTimer {
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

fn main() -> std::io::Result<()> {
    let game_timer = GameTimer::start_from(Duration::from_secs(60));
    let mut score: usize = 0;
    let mut penalty_ticks = 0;
    let mut strategem = strategem::random();

    std::io::stdout().execute(crossterm::cursor::Hide)?;

    'main_loop: loop {
        if crossterm::event::poll(Duration::from_millis(17))? {
            match event::read()? {
                Some(Key::Escape) => {
                    std::io::stdout()
                        .execute(crossterm::cursor::MoveUp(4))?
                        .execute(crossterm::terminal::Clear(
                            crossterm::terminal::ClearType::FromCursorDown,
                        ))?;
                    break 'main_loop;
                }
                Some(key) => strategem.assert_key(key.into()),
                _ => (),
            }
        } else {
            let time_left = game_timer.remaining();
            std::io::stdout().execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::FromCursorDown,
            ))?;

            if game_timer.is_over() {
                println!("Game Over!");
                println!("Your score: {score}");
                break;
            }

            println!("Score: {score}");
            println!(
                "Time left: {}.{:0>3.2}s",
                time_left.num_seconds(),
                time_left.num_milliseconds() - time_left.num_seconds() * 1000
            );
            println!("{}\n{strategem}", strategem.name());

            std::io::stdout().execute(crossterm::cursor::MoveUp(4))?;

            if strategem.is_completed() {
                strategem = strategem::random();
                score += 100;
            } else if !strategem.is_valid() {
                if penalty_ticks < 150 {
                    penalty_ticks += 10;
                } else {
                    penalty_ticks = 0;
                    strategem.reset();
                }
            }
        }
    }

    std::io::stdout().execute(crossterm::cursor::Show)?;

    Ok(())
}
