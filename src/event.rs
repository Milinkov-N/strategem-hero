use std::fmt::Display;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::error::Result;

pub struct Controls {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

impl Controls {
    pub fn wasd() -> Self {
        Self {
            up: KeyCode::Char('w'),
            left: KeyCode::Char('a'),
            down: KeyCode::Char('s'),
            right: KeyCode::Char('d'),
        }
    }

    pub fn arrows() -> Self {
        Self {
            up: KeyCode::Up,
            left: KeyCode::Left,
            down: KeyCode::Down,
            right: KeyCode::Right,
        }
    }
}

impl Display for Controls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}  - {}, {}  - {}, {}  - {}, {}  - {}",
            crate::strategem::StrategemKey::Left,
            format_key_code(&self.left),
            crate::strategem::StrategemKey::Up,
            format_key_code(&self.up),
            crate::strategem::StrategemKey::Right,
            format_key_code(&self.right),
            crate::strategem::StrategemKey::Down,
            format_key_code(&self.down),
        )
    }
}

#[derive(Debug)]
pub enum Key {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Escape,
}

pub fn read(controls: &Controls) -> Result<Option<Key>> {
    match crossterm::event::read()? {
        Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) if code.eq(&controls.up) => Ok(Some(Key::ArrowUp)),
        Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) if code.eq(&controls.down) => Ok(Some(Key::ArrowDown)),
        Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) if code.eq(&controls.left) => Ok(Some(Key::ArrowLeft)),
        Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) if code.eq(&controls.right) => Ok(Some(Key::ArrowRight)),
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            kind: KeyEventKind::Press,
            ..
        }) => Ok(Some(Key::Escape)),
        _ => Ok(None),
    }
}

fn format_key_code(code: &KeyCode) -> String {
    match code {
        KeyCode::Char(ch) => format!("'{ch}'"),
        KeyCode::Up => format!("ArrowUp"),
        KeyCode::Left => format!("ArrowLeft"),
        KeyCode::Down => format!("ArrowDown"),
        KeyCode::Right => format!("ArrowRight"),
        other => format!("{other:?}"),
    }
}
