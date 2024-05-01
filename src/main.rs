use std::time::Duration;

use crossterm::ExecutableCommand;

use event::Key;
use utility::{GameTimer, Penalty};

use crate::strategem::StrategemDifficulty;

mod event;
mod strategem;
mod utility;

fn main() -> std::io::Result<()> {
    let game_timer = GameTimer::start_from(Duration::from_secs(60));
    let penalty = Penalty::new(150, 10);
    let mut score: usize = 0;
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
            if game_timer.is_over() {
                std::io::stdout().execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::FromCursorDown,
                ))?;
                println!("Game Over!");
                println!("Your score: {score}");
                break;
            }

            print!("Score: {score}\n");
            print!("Time left: {game_timer}\n");
            print!("{:32}\n", strategem.name());
            println!("{strategem}");

            std::io::stdout().execute(crossterm::cursor::MoveUp(4))?;
            if strategem.is_completed() {
                score += match strategem.difficulty() {
                    StrategemDifficulty::Easy => 50,
                    StrategemDifficulty::Medium => 75,
                    StrategemDifficulty::Hard => 100,
                };
                strategem = strategem::random();
            } else if !strategem.is_valid() {
                penalty.apply(|| strategem.reset());
            }
        }
    }

    std::io::stdout().execute(crossterm::cursor::Show)?;

    Ok(())
}
