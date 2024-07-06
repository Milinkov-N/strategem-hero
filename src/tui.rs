use crossterm::{
    cursor,
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
        fmt.to_string()
            .chars()
            .filter(|ch| ch.eq(&'\n'))
            .for_each(|_| self.lines_count += 1);

        std::io::stdout().write_fmt(fmt)
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
