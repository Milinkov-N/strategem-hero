use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    ExecutableCommand,
};

use crate::{error::Result, screenln};

pub mod menu;
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

pub fn confirm_quit(action: Option<&str>) -> Result<()> {
    screenln!("Press 'q' to {}...", action.unwrap_or("quit"))?;
    while let Event::Key(ev) = crossterm::event::read()? {
        if let KeyEvent {
            code: KeyCode::Char('q'),
            kind: KeyEventKind::Press,
            ..
        } = ev
        {
            return Ok(());
        }
    }

    Ok(())
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
