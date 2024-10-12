use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    ExecutableCommand,
};

use crate::{error::Result, screenln};

pub mod screen;

pub struct HideCursor;

impl HideCursor {
    pub fn hide() -> Result<HideCursorGuard> {
        std::io::stdout().execute(crossterm::cursor::Hide)?;
        Ok(HideCursorGuard)
    }
}

pub struct HideCursorGuard;

impl Drop for HideCursorGuard {
    fn drop(&mut self) {
        std::io::stdout().execute(crossterm::cursor::Show).unwrap();
    }
}

pub fn confirm_action() -> Result<bool> {
    while let Event::Key(ev) = crossterm::event::read()? {
        match ev {
            KeyEvent {
                code: KeyCode::Char('y'),
                kind: KeyEventKind::Press,
                ..
            } => return Ok(true),
            KeyEvent {
                code: KeyCode::Char('n'),
                kind: KeyEventKind::Press,
                ..
            } => return Ok(false),
            _ => (),
        }
    }

    Ok(false)
}

pub fn select_from_list(prompt: Option<&str>, list: Vec<(&str, char)>) -> Result<usize> {
    let _sc = screen::cleaner();
    if let Some(msg) = prompt {
        screenln!("{msg}")?;
    }

    list.iter()
        .enumerate()
        .for_each(|(i, (msg, _))| screenln!("{}. {msg}", i + 1).unwrap());

    while let Event::Key(ev) = crossterm::event::read()? {
        if let KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        } = ev
        {
            if let Some(idx) = list.iter().position(|(_, ch)| code.eq(&KeyCode::Char(*ch))) {
                return Ok(idx);
            }
        }
    }

    Ok(0)
}
