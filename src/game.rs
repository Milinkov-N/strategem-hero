use std::time::Duration;

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{self, ClearType},
    ExecutableCommand,
};

use crate::{
    event::Key,
    strategem::{Strategem, StrategemDifficulty},
    utility::{GameTimer, HideCursor, Penalty},
};

struct GameState {
    game_timer: GameTimer,
    score: usize,
    best_score: usize,
    strategem: Strategem,
}

impl GameState {
    fn new(game_timer: GameTimer) -> Self {
        Self {
            game_timer,
            score: 0,
            best_score: 0,
            strategem: crate::strategem::random(),
        }
    }

    fn reset(&mut self) {
        self.game_timer.reset();
        self.score = 0;
        self.strategem = crate::strategem::random();
    }
}

pub struct Game {
    state: GameState,
    penalty: Penalty,
    is_running: bool,
}

impl Game {
    pub fn new(game_timer: GameTimer, penalty: Penalty) -> Self {
        Self {
            state: GameState::new(game_timer),
            penalty,
            is_running: true,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let _guard = HideCursor::hide()?;

        while self.is_running {
            let GameState {
                score,
                strategem,
                game_timer,
                ..
            } = &mut self.state;

            if crossterm::event::poll(Duration::from_millis(17))? {
                self.handle_input()?;
            } else {
                print!("Score: {}\n", score);
                print!("Time left: {}\n", game_timer);
                print!("{:32}\n", strategem.name());
                println!("{}", strategem);

                std::io::stdout().execute(crossterm::cursor::MoveUp(4))?;
                if strategem.is_completed() {
                    *score += match strategem.difficulty() {
                        StrategemDifficulty::Easy => 50,
                        StrategemDifficulty::Medium => 75,
                        StrategemDifficulty::Hard => 100,
                    };
                    game_timer.add(Duration::from_secs(1));
                    *strategem = crate::strategem::random();
                } else if !strategem.is_valid() {
                    self.penalty.apply(|| strategem.reset());
                }

                if game_timer.is_over() {
                    self.handle_game_over()?;
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self) -> std::io::Result<()> {
        match crate::event::read()? {
            Some(Key::Escape) => {
                std::io::stdout()
                    .execute(cursor::MoveUp(4))?
                    .execute(terminal::Clear(ClearType::FromCursorDown))?;
                self.is_running = false;
            }
            Some(key) => self.state.strategem.assert_key(key.into()),

            _ => (),
        };

        Ok(())
    }

    fn handle_game_over(&mut self) -> std::io::Result<()> {
        std::io::stdout().execute(terminal::Clear(ClearType::FromCursorDown))?;
        println!("Game Over!");
        println!("Your score: {}", self.state.score);

        self.yes_no_prompt(
            "Restart the game",
            |this| {
                this.state.reset();
                std::io::stdout()
                    .execute(cursor::MoveUp(3))?
                    .execute(terminal::Clear(ClearType::FromCursorDown))?;
                Ok(())
            },
            |this: &mut Game| this.is_running = false,
        )?;

        Ok(())
    }

    fn yes_no_prompt(
        &mut self,
        msg: &str,
        on_yes: impl FnOnce(&mut Game) -> std::io::Result<()>,
        on_no: impl FnOnce(&mut Game),
    ) -> std::io::Result<()> {
        println!("{msg} [y/n]?");
        while let Event::Key(ev) = crossterm::event::read()? {
            match ev {
                KeyEvent {
                    code: KeyCode::Char('y'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    on_yes(self)?;
                    break;
                }
                KeyEvent {
                    code: KeyCode::Char('n'),
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    on_no(self);
                    break;
                }
                _ => (),
            }
        }

        Ok(())
    }
}
