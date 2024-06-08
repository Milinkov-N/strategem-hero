use super::{Strategem, StrategemClass, StrategemCode, StrategemDifficulty, StrategemKey};

pub struct StrategemBuilder {
    idx: usize,
    code: StrategemCode,
    class: StrategemClass,
}

impl StrategemBuilder {
    pub const fn new(class: StrategemClass) -> Self {
        Self {
            idx: 0,
            code: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
            class,
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
            difficulty: match self.idx {
                0..=3 => StrategemDifficulty::Easy,
                4..=6 => StrategemDifficulty::Medium,
                _ => StrategemDifficulty::Hard,
            },
            class: self.class,
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
