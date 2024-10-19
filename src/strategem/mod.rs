use std::fmt::Display;

use crossterm::style::Stylize;
use rand::Rng;

use crate::{event::Key, strategem::builder::StrategemBuilder, utility::format_strategem_name};
use collections::ALL_STRATEGEMS;

mod builder;
mod collections;

pub type StrategemCode = [Option<StrategemKey>; 16];

#[derive(Clone, PartialEq, Eq)]
pub enum StrategemKey {
    Up,
    Down,
    Left,
    Right,
}

impl From<Key> for StrategemKey {
    fn from(value: Key) -> Self {
        match value {
            Key::ArrowUp => StrategemKey::Up,
            Key::ArrowDown => StrategemKey::Down,
            Key::ArrowLeft => StrategemKey::Left,
            Key::ArrowRight => StrategemKey::Right,
            unhandled => panic!("Cannot convert {unhandled:#?} to StrategemKey"),
        }
    }
}

impl Display for StrategemKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "🡅"),
            Self::Down => write!(f, "🡇"),
            Self::Left => write!(f, "🡄"),
            Self::Right => write!(f, "🡆"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrategemDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrategemClass {
    Supply,
    Mission,
    Defensive,
    Offensive,
}

#[derive(Clone)]
pub struct Strategem {
    name: &'static str,
    difficulty: StrategemDifficulty,
    class: StrategemClass,
    idx: usize,
    valid: bool,
    completed: bool,
    code: StrategemCode,
}

impl Strategem {
    const fn builder(class: StrategemClass) -> StrategemBuilder {
        StrategemBuilder::new(class)
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn difficulty(&self) -> &StrategemDifficulty {
        &self.difficulty
    }

    pub const fn class(&self) -> &StrategemClass {
        &self.class
    }

    pub fn assert_key(&mut self, key: StrategemKey) {
        if self.is_completed() || !self.is_valid() {
            return;
        }

        if let Some(code_key) = &self.code[self.idx] {
            self.idx += 1;
            self.valid = code_key.eq(&key);
        }
    }

    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_completed(&self) -> bool {
        self.valid && self.code[self.idx].is_none()
    }

    pub fn reset(&mut self) {
        self.idx = 0;
        self.valid = true;
        self.completed = false;
    }
}

impl Display for Strategem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_name = format_strategem_name(self);
        writeln!(f, "\x1b[K{}", fmt_name)?;

        self.code.iter().enumerate().for_each(|(i, code)| {
            if let Some(key) = code {
                if !self.is_valid() {
                    write!(f, "{} ", key.to_string().dark_red()).unwrap();
                } else if i < self.idx {
                    write!(f, "{} ", key.to_string().yellow()).unwrap();
                } else {
                    write!(f, "{key} ").unwrap();
                }
            } else {
                write!(f, " ").unwrap();
            }
        });

        Ok(())
    }
}

pub fn random() -> Strategem {
    ALL_STRATEGEMS[rand::thread_rng().gen::<usize>() % ALL_STRATEGEMS.len()].clone()
}
