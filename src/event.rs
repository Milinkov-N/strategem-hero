use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::error::Result;

pub struct Controls {
    up: crossterm::event::KeyCode,
    down: crossterm::event::KeyCode,
    left: crossterm::event::KeyCode,
    right: crossterm::event::KeyCode,
}

impl Controls {
    pub fn wasd() -> Self {
        Self {
            up: crossterm::event::KeyCode::Char('w'),
            left: crossterm::event::KeyCode::Char('a'),
            down: crossterm::event::KeyCode::Char('s'),
            right: crossterm::event::KeyCode::Char('d'),
        }
    }

    pub fn arrows() -> Self {
        Self {
            up: crossterm::event::KeyCode::Up,
            left: crossterm::event::KeyCode::Left,
            down: crossterm::event::KeyCode::Down,
            right: crossterm::event::KeyCode::Right,
        }
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
