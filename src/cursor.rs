use crate::common::{self, Direction, Position};
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

pub fn move_right(stdout_handle: &mut Stdout) -> Result<(Position, Direction), Error> {
    let (columns, _) = crossterm::terminal::size()?;
    let (mut x, y) = crossterm::cursor::position()?;
    let mut scroll_direction = Direction::None;

    x += 1;

    let is_scroll = x + 1 > columns;
    if is_scroll {
        x = columns;
        scroll_direction = Direction::Right;
    }

    let new_position = Position { x, y };

    Ok((move_to(stdout_handle, new_position)?, scroll_direction))
}

pub fn move_left(stdout_handle: &mut Stdout) -> Result<(Position, Direction), Error> {
    let (mut x, y) = crossterm::cursor::position()?;
    let mut scroll_direction = Direction::None;

    x -= 1;

    let is_scroll = x - 1 == 0;
    if is_scroll {
        x = 2;
        scroll_direction = Direction::Left;
    }

    let new_position = Position { x, y };

    Ok((move_to(stdout_handle, new_position)?, scroll_direction))
}

pub fn move_down(stdout_handle: &mut Stdout) -> Result<(Position, Direction), Error> {
    let (_, rows) = crossterm::terminal::size()?;
    let (x, mut y) = crossterm::cursor::position()?;
    let mut scroll_direction = Direction::None;

    y += 1;

    let is_scroll = y + 1 >= rows;
    if is_scroll {
        y = rows;
        scroll_direction = Direction::Down;
    }

    let new_position = Position { x, y };

    Ok((move_to(stdout_handle, new_position)?, scroll_direction))
}

pub fn move_up(stdout_handle: &mut Stdout) -> Result<(Position, Direction), Error> {
    let (x, mut y) = crossterm::cursor::position()?;
    let mut scroll_direction = Direction::None;

    if y > 0 {
        y -= 1;
    }

    let is_scroll = y == 0;
    if is_scroll {
        scroll_direction = Direction::Up;
    }

    let new_position = Position { x, y };

    Ok((move_to(stdout_handle, new_position)?, scroll_direction))
}

pub fn hide(stdout_handle: &mut Stdout) -> Result<(), Error> {
    queue!(stdout_handle, crossterm::cursor::Hide)
}

pub fn show(stdout_handle: &mut Stdout) -> Result<(), Error> {
    queue!(stdout_handle, crossterm::cursor::Show)
}
