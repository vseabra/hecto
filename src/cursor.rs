use crate::vectors::Vec2;
use crossterm::{execute, queue};
use std::io::{Error, Stdout};

pub fn move_to(stdout_handle: &mut Stdout, target: Vec2) -> Result<Vec2, Error> {
    let move_command = crossterm::cursor::MoveTo(target.x, target.y);

    execute!(stdout_handle, move_command)?;

    Ok(target)
}

pub fn hide(stdout_handle: &mut Stdout) -> Result<(), Error> {
    queue!(stdout_handle, crossterm::cursor::Hide)
}

pub fn show(stdout_handle: &mut Stdout) -> Result<(), Error> {
    queue!(stdout_handle, crossterm::cursor::Show)
}
