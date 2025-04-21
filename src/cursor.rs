use crate::common::{self, Position};
use crossterm::{execute, queue};
use std::io::{Error, Stdout};

pub fn move_to(stdout_handle: &mut Stdout, target: common::Position) -> Result<Position, Error> {
    let (columns, rows) = crossterm::terminal::size()?;

    let clamped_x = target.x.clamp(0, columns - 1);
    let clamped_y = target.y.clamp(0, rows - 1);

    let move_command = crossterm::cursor::MoveTo(clamped_x, clamped_y);
    let updated_position = Position {
        x: clamped_x,
        y: clamped_y,
    };

    execute!(stdout_handle, move_command)?;

    Ok(updated_position)
}

pub fn move_right_with_wrap(stdout_handle: &mut Stdout) -> Result<Position, Error> {
    let (columns, _) = crossterm::terminal::size()?;
    let (mut x, mut y) = crossterm::cursor::position()?;

    x += 1;

    let should_wrap = x + 1 > columns;
    if should_wrap {
        x = 2;
        y += 1
    }

    let new_position = Position { x, y };

    move_to(stdout_handle, new_position)
}

pub fn move_left_with_wrap(stdout_handle: &mut Stdout) -> Result<Position, Error> {
    let (columns, _) = crossterm::terminal::size()?;

    let (mut x, mut y) = crossterm::cursor::position()?;

    x -= 1;

    let should_wrap = x - 1 == 0;
    if should_wrap {
        x = if y == 0 { 2 } else { columns };
        y = if y > 0 { y - 1 } else { 0 };
    }

    let new_position = Position { x, y };

    move_to(stdout_handle, new_position)
}

pub fn move_down(stdout_handle: &mut Stdout) -> Result<Position, Error> {
    let (x, y) = crossterm::cursor::position()?;
    let new_position = Position { x, y: y + 1 };

    move_to(stdout_handle, new_position)
}

pub fn move_up(stdout_handle: &mut Stdout) -> Result<Position, Error> {
    let (x, mut y) = crossterm::cursor::position()?;
    y = if y == 0 { 0 } else { y - 1 };

    let new_position = Position { x, y };

    move_to(stdout_handle, new_position)
}

pub fn hide(stdout_handle: &mut Stdout) -> Result<(), Error> {
    queue!(stdout_handle, crossterm::cursor::Hide)
}

pub fn show(stdout_handle: &mut Stdout) -> Result<(), Error> {
    queue!(stdout_handle, crossterm::cursor::Show)
}
