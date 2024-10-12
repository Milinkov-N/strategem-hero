use std::{
    cell::RefCell,
    io::Write,
    sync::{Mutex, OnceLock},
};

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use crate::error::Result;

#[macro_export]
macro_rules! screenln {
        ($($arg:tt)*) => {{
            $crate::tui::screen::print(format_args!($($arg)*))
        }};
    }

static SCREEN: OnceLock<Mutex<RefCell<Screen>>> = OnceLock::new();

#[derive(Default)]
struct Screen {
    lines_count: u16,
}

impl Screen {
    fn move_back(&mut self) -> Result<()> {
        std::io::stdout().execute(cursor::MoveUp(self.lines_count))?;
        self.lines_count = 0;
        Ok(())
    }

    fn move_back_by(&mut self, value: u16) -> Result<()> {
        std::io::stdout().execute(cursor::MoveUp(value))?;
        self.lines_count -= value;
        Ok(())
    }

    fn full_clear(&mut self) -> Result<()> {
        std::io::stdout()
            .execute(cursor::MoveUp(self.lines_count))?
            .execute(terminal::Clear(ClearType::FromCursorDown))?;
        self.lines_count = 0;
        Ok(())
    }
}

impl std::io::Write for Screen {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.lines_count += 1;
        std::io::stdout().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        let mut fmt = fmt.to_string().replace('\n', "\r\n");

        if !fmt.ends_with("\r\n") {
            fmt.push_str("\r\n");
        }

        fmt.chars()
            .filter(|ch| ch.eq(&'\n'))
            .for_each(|_| self.lines_count += 1);

        std::io::stdout().write(fmt.as_bytes()).map(|_| ())
    }
}

pub struct ScreenScoped(u16);

impl Drop for ScreenScoped {
    fn drop(&mut self) {
        let mut mtx = SCREEN.get().unwrap().lock().unwrap();
        let val = mtx.get_mut().lines_count - self.0;
        mtx.get_mut().move_back_by(val).unwrap();
    }
}

pub struct ScreenCleaner;

impl Drop for ScreenCleaner {
    fn drop(&mut self) {
        let mut mtx = SCREEN.get().unwrap().lock().unwrap();
        mtx.get_mut().full_clear().unwrap();
    }
}

pub fn scope() -> ScreenScoped {
    let mut mtx = SCREEN.get_or_init(Mutex::default).lock().unwrap();
    ScreenScoped(mtx.get_mut().lines_count)
}

pub fn print(fmt: std::fmt::Arguments<'_>) -> Result<()> {
    let mut mtx = SCREEN.get_or_init(Mutex::default).lock().unwrap();
    mtx.get_mut().write_fmt(fmt)?;
    Ok(())
}

pub fn move_back() -> Result<()> {
    let mut mtx = SCREEN.get_or_init(Mutex::default).lock().unwrap();
    mtx.get_mut().move_back()
}

pub fn clear() -> Result<()> {
    std::io::stdout().execute(terminal::Clear(ClearType::FromCursorDown))?;
    Ok(())
}

pub fn cleaner() -> ScreenCleaner {
    ScreenCleaner
}
