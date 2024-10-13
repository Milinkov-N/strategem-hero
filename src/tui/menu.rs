use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEventKind};

use crate::{error::Result, screenln};

pub struct Menu<T: Display> {
    items: Vec<T>,
}

impl<T: Display> Menu<T> {
    pub fn builder() -> MenuBuilder<T> {
        MenuBuilder::new()
    }

    pub fn exec(&self, prompt: &str) -> Result<Option<usize>> {
        let mut idx: usize = 0;

        screenln!("{prompt}")?;
        loop {
            let _screen_scope = crate::tui::screen::scope();

            self.items.iter().enumerate().for_each(|(i, item)| {
                screenln!("  [{}] {}", if i == idx { '*' } else { ' ' }, item).unwrap();
            });

            if let crossterm::event::Event::Key(ev) = crossterm::event::read()? {
                match ev {
                    crossterm::event::KeyEvent {
                        code: KeyCode::Up,
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        idx = idx.saturating_sub(1);
                    }
                    crossterm::event::KeyEvent {
                        code: KeyCode::Down,
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        let next = idx + 1;
                        idx = next.min(self.items.len() - 1);
                    }
                    crossterm::event::KeyEvent {
                        code: KeyCode::Enter,
                        kind: KeyEventKind::Press,
                        ..
                    } => return Ok(Some(idx)),
                    crossterm::event::KeyEvent {
                        code: KeyCode::Char('q'),
                        kind: KeyEventKind::Press,
                        ..
                    } => return Ok(None),

                    _ => (),
                }
            } else {
                break;
            }
        }

        Ok(None)
    }
}

pub struct MenuBuilder<T: Display> {
    items: Vec<T>,
}

impl<T: Display> MenuBuilder<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(mut self, value: T) -> Self {
        self.items.push(value);
        self
    }

    pub fn build(self) -> Menu<T> {
        Menu { items: self.items }
    }
}
