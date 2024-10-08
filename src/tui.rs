use std::io::Write;

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{self, ClearType},
    ExecutableCommand,
};

use crate::error::Result;

pub struct ScreenWriter {
    lines_count: u16,
}

impl ScreenWriter {
    pub fn new() -> Self {
        Self { lines_count: 0 }
    }

    pub fn clear() -> Result<()> {
        std::io::stdout().execute(terminal::Clear(ClearType::FromCursorDown))?;
        Ok(())
    }
}

impl std::io::Write for ScreenWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.lines_count += 1;
        std::io::stdout().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        let fmt = fmt.to_string().replace('\n', "\r\n");

        fmt.chars()
            .filter(|ch| ch.eq(&'\n'))
            .for_each(|_| self.lines_count += 1);

        std::io::stdout().write(fmt.as_bytes()).map(|_| ())
    }
}

impl Drop for ScreenWriter {
    fn drop(&mut self) {
        std::io::stdout()
            .execute(cursor::MoveUp(self.lines_count))
            .unwrap();
    }
}

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

pub fn select_from_list(
    screen: Option<ScreenWriter>,
    prompt: Option<&str>,
    list: Vec<(&str, char)>,
) -> Result<usize> {
    let mut screen = match screen {
        Some(screen) => screen,
        None => ScreenWriter::new(),
    };

    if let Some(msg) = prompt {
        writeln!(screen, "{msg}")?;
    }

    list.iter()
        .enumerate()
        .for_each(|(i, (msg, _))| writeln!(screen, "{}. {msg}", i + 1).unwrap());

    while let Event::Key(ev) = crossterm::event::read()? {
        if let KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        } = ev
        {
            if let Some(idx) = list.iter().position(|(_, ch)| code.eq(&KeyCode::Char(*ch))) {
                drop(screen);
                ScreenWriter::clear()?;
                return Ok(idx);
            }
        }
    }

    drop(screen);
    ScreenWriter::clear()?;

    Ok(0)
}
