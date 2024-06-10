use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::error::Result;

#[derive(Debug)]
pub enum Key {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Escape,
}

pub fn read() -> Result<Option<Key>> {
    match crossterm::event::read()? {
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            kind: KeyEventKind::Press,
            ..
        }) => Ok(Some(Key::ArrowUp)),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            kind: KeyEventKind::Press,
            ..
        }) => Ok(Some(Key::ArrowDown)),
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            kind: KeyEventKind::Press,
            ..
        }) => Ok(Some(Key::ArrowLeft)),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            kind: KeyEventKind::Press,
            ..
        }) => Ok(Some(Key::ArrowRight)),
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            kind: KeyEventKind::Press,
            ..
        }) => Ok(Some(Key::Escape)),
        _ => Ok(None),
    }
}
