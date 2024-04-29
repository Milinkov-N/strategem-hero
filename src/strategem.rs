use std::fmt::Display;

use crossterm::style::Stylize;
use rand::Rng;

pub type StrategemCode = [Option<StrategemKey>; 16];

#[derive(Clone, PartialEq, Eq)]
pub enum StrategemKey {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Clone)]
pub struct Strategem {
    name: &'static str,
    idx: usize,
    valid: bool,
    completed: bool,
    code: StrategemCode,
}

impl Strategem {
    pub const fn builder() -> StrategemBuilder {
        StrategemBuilder::new()
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub fn assert_key(&mut self, key: StrategemKey) -> bool {
        if self.is_completed() || !self.is_valid() {
            return false;
        }

        if let Some(code_key) = &self.code[self.idx] {
            self.idx += 1;
            self.valid = code_key.eq(&key);
            return self.valid;
        }

        false
    }

    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_completed(&self) -> bool {
        self.valid && self.code[self.idx] == None
    }

    pub fn reset(&mut self) {
        self.idx = 0;
        self.valid = true;
        self.completed = false;
    }
}

impl Display for Strategem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code.iter().enumerate().for_each(|(i, code)| {
            if let Some(key) = code {
                if !self.is_valid() {
                    write!(f, "{}", key.to_string().dark_red()).unwrap();
                } else if i < self.idx {
                    write!(f, "{}", key.to_string().yellow()).unwrap();
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

pub struct StrategemBuilder {
    idx: usize,
    code: StrategemCode,
}

impl StrategemBuilder {
    pub const fn new() -> Self {
        Self {
            idx: 0,
            code: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
        }
    }

    pub const fn up(self) -> Self {
        self.insert(StrategemKey::Up)
    }

    pub const fn down(self) -> Self {
        self.insert(StrategemKey::Down)
    }

    pub const fn left(self) -> Self {
        self.insert(StrategemKey::Left)
    }

    pub const fn right(self) -> Self {
        self.insert(StrategemKey::Right)
    }

    pub const fn build(self, name: &'static str) -> Strategem {
        Strategem {
            name,
            idx: 0,
            valid: true,
            completed: false,
            code: self.code,
        }
    }

    const fn insert(mut self, value: StrategemKey) -> Self {
        if self.idx < 16 {
            self.code[self.idx] = Some(value);
            self.idx += 1;
        }

        self
    }
}

const ALL_STRATEGEMS: [Strategem; 7] = [
    eagle_strafing_run(),
    eagle_air_strike(),
    eagle_cluster_bomb(),
    eagle_napalm_airstrike(),
    eagle_smoke_strike(),
    eagle_110mm_rocket_pods(),
    eagle_500kg_bomb(),
];

pub const fn eagle_strafing_run() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .right()
        .build("Eagle Strafing Run")
}

pub const fn eagle_air_strike() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .right()
        .build("Eagle Air Strike")
}

pub const fn eagle_cluster_bomb() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .down()
        .right()
        .build("Eagle Cluster Bomb")
}

pub const fn eagle_napalm_airstrike() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .up()
        .build("Eagle Napalm Airstrike")
}

pub const fn eagle_smoke_strike() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .up()
        .down()
        .build("Eagle Smoke Strike")
}

pub const fn eagle_110mm_rocket_pods() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .up()
        .left()
        .build("Eagle 110MM Rocket Pods")
}

pub const fn eagle_500kg_bomb() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .down()
        .down()
        .build("Eagle 500KG Bomb")
}

pub fn random() -> Strategem {
    ALL_STRATEGEMS[rand::thread_rng().gen::<usize>() % ALL_STRATEGEMS.len()].clone()
}
