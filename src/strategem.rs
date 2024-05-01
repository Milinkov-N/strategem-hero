use std::fmt::Display;

use crossterm::style::Stylize;
use rand::Rng;

use crate::event::Key;

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

#[derive(Clone)]
pub struct Strategem {
    name: &'static str,
    difficulty: StrategemDifficulty,
    idx: usize,
    valid: bool,
    completed: bool,
    code: StrategemCode,
}

impl Strategem {
    const fn builder() -> StrategemBuilder {
        StrategemBuilder::new()
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn difficulty(&self) -> &StrategemDifficulty {
        &self.difficulty
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

struct StrategemBuilder {
    idx: usize,
    code: StrategemCode,
}

impl StrategemBuilder {
    const fn new() -> Self {
        Self {
            idx: 0,
            code: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
        }
    }

    const fn up(self) -> Self {
        self.insert(StrategemKey::Up)
    }

    const fn down(self) -> Self {
        self.insert(StrategemKey::Down)
    }

    const fn left(self) -> Self {
        self.insert(StrategemKey::Left)
    }

    const fn right(self) -> Self {
        self.insert(StrategemKey::Right)
    }

    const fn build(self, name: &'static str) -> Strategem {
        Strategem {
            name,
            difficulty: match self.idx {
                0..=3 => StrategemDifficulty::Easy,
                4..=6 => StrategemDifficulty::Medium,
                _ => StrategemDifficulty::Hard,
            },
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

const ALL_STRATEGEMS: [Strategem; 18] = [
    orbital_gatling_barrage(),
    orbital_airburst_strike(),
    orbital_120mm_he_barrage(),
    orbital_380mm_he_barrage(),
    orbital_walking_barrage(),
    orbital_laser(),
    orbital_railcannon_strike(),
    orbital_precision_strike(),
    orbital_gas_strike(),
    orbital_ems_strike(),
    orbital_smoke_strike(),
    eagle_strafing_run(),
    eagle_air_strike(),
    eagle_cluster_bomb(),
    eagle_napalm_airstrike(),
    eagle_smoke_strike(),
    eagle_110mm_rocket_pods(),
    eagle_500kg_bomb(),
];

pub const fn orbital_gatling_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .left()
        .up()
        .up()
        .build("Orbital Gatling Barrage")
}

pub const fn orbital_airburst_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .right()
        .build("Orbital Airburst Strike")
}

pub const fn orbital_120mm_he_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .down()
        .left()
        .right()
        .down()
        .build("Orbital 120MM HE Barrage")
}

pub const fn orbital_380mm_he_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .up()
        .up()
        .left()
        .down()
        .down()
        .build("Orbital 380MM HE Barrage")
}

pub const fn orbital_walking_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .right()
        .down()
        .right()
        .down()
        .build("Orbital Walking Barrage")
}

pub const fn orbital_laser() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .up()
        .right()
        .down()
        .build("Orbital Laser")
}

pub const fn orbital_railcannon_strike() -> Strategem {
    Strategem::builder()
        .right()
        .up()
        .down()
        .down()
        .right()
        .build("Orbital Railcannon Strike")
}

pub const fn orbital_precision_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .up()
        .build("Orbital Precision Strike")
}

pub const fn orbital_gas_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .down()
        .right()
        .build("Orbital Gas Strike")
}

pub const fn orbital_ems_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .left()
        .down()
        .build("Orbital EMS Strike")
}

pub const fn orbital_smoke_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .down()
        .up()
        .build("Orbital Smoke Strike")
}

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
