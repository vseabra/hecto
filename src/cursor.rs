use crate::common::{self, Position};
use crossterm::queue;
use std::io::{Error, Stdout};

pub struct Cursor {
    pub position: common::Position,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            position: common::Position { x: 2, y: 0 },
        }
    }
}

impl Cursor {
    pub fn move_to(
        &mut self,
        stdout_handle: &mut Stdout,
        target: common::Position,
    ) -> Result<(), Error> {
        // handle clamps
        let (columns, rows) = crossterm::terminal::size()?;

        let clamped_x = target.x.clamp(0, columns - 1);
        let clamped_y = target.y.clamp(0, rows - 1);

        let move_command = crossterm::cursor::MoveTo(clamped_x, clamped_y);
        queue!(stdout_handle, move_command)?;

        self.position = Position {
            x: clamped_x,
            y: clamped_y,
        };

        Ok(())
    }

    pub fn move_right_with_wrap(&mut self, stdout_handle: &mut Stdout) -> Result<(), Error> {
        let (columns, _) = crossterm::terminal::size()?;

        let mut new_x = self.position.x + 1;
        let mut new_y = self.position.y;

        let should_wrap = new_x + 1 > columns;

        if should_wrap {
            new_x = 2;
            new_y += 1;
        }

        let target = common::Position { x: new_x, y: new_y };

        self.move_to(stdout_handle, target)
    }

    pub fn move_left_with_wrap(&mut self, stdout_handle: &mut Stdout) -> Result<(), Error> {
        let (columns, _) = crossterm::terminal::size()?;

        let mut new_x = self.position.x - 1;
        let mut new_y = self.position.y;

        let should_wrap = new_x - 1 == 0;

        if should_wrap {
            new_x = columns;
            // new_y -= 1;
            new_y = if new_y == 0 { 0 } else { new_y - 1 };
        }

        let target = common::Position { x: new_x, y: new_y };

        self.move_to(stdout_handle, target)
    }

    pub fn move_down(&mut self, stdout_handle: &mut Stdout) -> Result<(), Error> {
        let new_x = self.position.x;
        let new_y = self.position.y + 1;

        let target = common::Position { x: new_x, y: new_y };

        self.move_to(stdout_handle, target)
    }

    pub fn move_up(&mut self, stdout_handle: &mut Stdout) -> Result<(), Error> {
        let new_x = self.position.x;
        let new_y = if self.position.y == 0 {
            0
        } else {
            self.position.y - 1
        };

        let target = common::Position { x: new_x, y: new_y };

        self.move_to(stdout_handle, target)
    }

    pub fn hide(&mut self, stdout_handle: &mut Stdout) -> Result<(), Error> {
        queue!(stdout_handle, crossterm::cursor::Hide)
    }

    pub fn show(&mut self, stdout_handle: &mut Stdout) -> Result<(), Error> {
        queue!(stdout_handle, crossterm::cursor::Show)
    }
}
