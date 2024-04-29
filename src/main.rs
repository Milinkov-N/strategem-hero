use std::time::Duration;

use chrono::TimeDelta;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    ExecutableCommand,
};

mod strategem;

use strategem::StrategemKey;

fn main() -> std::io::Result<()> {
    let game_over_time = chrono::Utc::now() + Duration::from_secs(60);
    let mut score: usize = 0;
    let mut penalty_ticks = 0;
    let mut strategem = strategem::random();

    std::io::stdout().execute(crossterm::cursor::Hide)?;

    'main_loop: loop {
        if game_over_time - chrono::Utc::now() <= TimeDelta::zero() {
            std::io::stdout()
                .execute(crossterm::cursor::MoveUp(3))?
                .execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::FromCursorDown,
                ))?;
            println!("Game Over!");
            println!("Your score: {score}");
            break;
        }

        if crossterm::event::poll(Duration::from_millis(17))? {
            match crossterm::event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    strategem.assert_key(StrategemKey::Up);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    strategem.assert_key(StrategemKey::Down);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    strategem.assert_key(StrategemKey::Left);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    strategem.assert_key(StrategemKey::Right);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    std::io::stdout().execute(crossterm::terminal::Clear(
                        crossterm::terminal::ClearType::FromCursorDown,
                    ))?;
                    break 'main_loop;
                }
                _ => (),
            }
        } else {
            let time_left = game_over_time - chrono::Utc::now();

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
                if penalty_ticks < 100 {
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
