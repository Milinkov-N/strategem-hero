use std::{io::Write, time::Duration};

use crate::{
    error::Result,
    event::{Controls, Key},
    storage::LeaderboardStorage,
    strategem::Strategem,
    tui::{self, HideCursor, ScreenWriter},
    utility::{self, GameTimer, Multiplier, Penalty},
};

struct GameState {
    game_timer: GameTimer,
    score: usize,
    streak: usize,
    strategem: Strategem,
}

impl GameState {
    fn new(game_timer: GameTimer) -> Self {
        Self {
            game_timer,
            score: 0,
            streak: 0,
            strategem: crate::strategem::random(),
        }
    }

    fn reset(&mut self) {
        self.game_timer.reset();
        self.score = 0;
        self.streak = 0;
        self.strategem = crate::strategem::random();
    }
}

pub struct Game {
    state: GameState,
    store: LeaderboardStorage,
    penalty: Penalty,
    controls: Controls,
    is_running: bool,
}

impl Game {
    pub fn new(
        store: LeaderboardStorage,
        game_timer: GameTimer,
        controls: Controls,
        penalty: Penalty,
    ) -> Self {
        Self {
            state: GameState::new(game_timer),
            store,
            penalty,
            controls,
            is_running: true,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let _guard = HideCursor::hide()?;

        while self.is_running {
            if crossterm::event::poll(Duration::from_millis(17))? {
                self.handle_input()?;
            } else {
                self.print_frame()?;
                self.update_state();

                if self.state.game_timer.is_over() {
                    self.handle_game_over()?;
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        match crate::event::read(&self.controls)? {
            Some(Key::Escape) => {
                ScreenWriter::clear()?;
                self.is_running = false;
            }
            Some(key) => self.state.strategem.assert_key(key.into()),

            _ => (),
        };

        Ok(())
    }

    fn print_frame(&mut self) -> Result<()> {
        let mut screen = ScreenWriter::new();
        writeln!(
            screen,
            "Score: {} {:>5}",
            self.state.score,
            Multiplier::get(self.state.streak)
        )?;
        writeln!(screen, "{}", self.state.game_timer)?;
        writeln!(screen, "{}", self.state.strategem)?;
        writeln!(screen, "Controls: {}", self.controls)?;
        Ok(())
    }

    fn update_state(&mut self) {
        let GameState {
            score,
            strategem,
            game_timer,
            streak,
            ..
        } = &mut self.state;

        if strategem.is_completed() {
            *streak += 1;
            *score += utility::get_score_value(strategem.difficulty(), Multiplier::get(*streak));
            game_timer.add(Duration::from_millis(1500));
            *strategem = crate::strategem::random();
        } else if !strategem.is_valid() {
            *streak = 0;
            self.penalty.apply(|| strategem.reset());
        }
    }

    fn handle_game_over(&mut self) -> Result<()> {
        let mut screen = ScreenWriter::new();
        let player = self.store.find_by_name("You").ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Player not found in database",
        ))?;

        ScreenWriter::clear()?;
        writeln!(
            screen,
            "Game Over! You scored {} Democracy Points",
            self.state.score
        )?;

        if self.state.score > player.score {
            self.store
                .insert_or_update(&player.nickname, self.state.score)
                .unwrap();
        }

        self.print_leaderboard(&mut screen, player.score)?;

        writeln!(screen, "Restart the game [y/n]?")?;

        if tui::confirm_action()? {
            self.state.reset();
        } else {
            self.is_running = false;
        }

        drop(screen);
        ScreenWriter::clear()
    }

    fn print_leaderboard(&mut self, screen: &mut ScreenWriter, curr_score: usize) -> Result<()> {
        writeln!(screen, "Leaderboard:")?;
        self.store
            .select_all()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, rec)| {
                if rec.nickname.eq("You") && rec.score > curr_score {
                    writeln!(
                        screen,
                        "  {}. {:<18} {} New record!",
                        i + 1,
                        rec.nickname,
                        rec.score
                    )
                    .unwrap();
                } else {
                    writeln!(screen, "  {}. {:<18} {}", i + 1, rec.nickname, rec.score).unwrap();
                }
            });

        Ok(())
    }
}
