use std::time::Duration;

use crate::{
    error::Result,
    event::{Controls, Key},
    screenln,
    storage::{Leaderboard, PlayerData, Storage},
    strategem::Strategem,
    tui,
    utility::{self, FreezeState, GameTimer, InputFreeze, Multiplier},
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
    player: PlayerData,
    leaderboard: Leaderboard,
    freeze: InputFreeze,
    controls: Controls,
    is_running: bool,
}

impl Game {
    pub fn new(
        player: PlayerData,
        leaderboard: Leaderboard,
        game_timer: GameTimer,
        controls: Controls,
        freeze: InputFreeze,
    ) -> Self {
        Self {
            state: GameState::new(game_timer),
            player,
            leaderboard,
            freeze,
            controls,
            is_running: true,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        tui::screen::full_clear()?;

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
                tui::screen::clear()?;
                self.is_running = false;
            }
            Some(key) => self.state.strategem.assert_key(key.into()),

            _ => (),
        };

        Ok(())
    }

    fn print_frame(&mut self) -> Result<()> {
        screenln!(
            "Score: {} {:>5}",
            self.state.score,
            Multiplier::get(self.state.streak)
        )?;
        screenln!("{}", self.state.game_timer)?;
        screenln!("{}", self.state.strategem)?;
        screenln!("Controls: {}", self.controls)?;

        tui::screen::move_back()
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
            let base_reward = Duration::from_millis(1000);
            *streak += 1;
            *score += utility::get_score_value(
                strategem.difficulty(),
                Multiplier::get(*streak),
                self.player.bonus_score(),
            );
            game_timer.add(base_reward + self.player.time_reward_dur());
            *strategem = crate::strategem::random();
        } else if !strategem.is_valid() {
            *streak = 0;
            if let FreezeState::Completed = self.freeze.ping() {
                strategem.reset();
                game_timer.sub(self.player.penalty_debuff_dur());
            };
        }
    }

    fn handle_game_over(&mut self) -> Result<()> {
        let mut _sc = tui::screen::cleaner();
        let (_, score) =
            self.leaderboard
                .iter()
                .find(|rec| rec.0.eq("You"))
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Player not found in database",
                ))?;

        tui::screen::clear()?;
        screenln!(
            "Game Over! You scored {} Democracy Points",
            self.state.score
        )?;

        if &self.state.score > score {
            self.leaderboard.insert("You", self.state.score);
        }

        self.print_leaderboard(self.state.score)?;

        screenln!("Restart the game [y/n]?")?;
        if tui::confirm_action()? {
            self.state.reset();
            self.freeze.reset();
        } else {
            self.is_running = false;
        }

        self.player.add_to_wallet(self.state.score);
        self.player.save()?;
        self.leaderboard.save()
    }

    fn print_leaderboard(&mut self, curr_score: usize) -> Result<()> {
        screenln!("Leaderboard:")?;
        self.leaderboard
            .sorted_vec()
            .iter()
            .enumerate()
            .for_each(|(i, rec)| {
                if rec.0.eq("You") && rec.1 > &curr_score {
                    screenln!("  {}. {:<18} {} New record!", i + 1, rec.0, rec.1).unwrap();
                } else {
                    screenln!("  {}. {:<18} {}", i + 1, rec.0, rec.1).unwrap();
                }
            });

        Ok(())
    }
}
