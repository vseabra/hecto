use crossterm::queue;
use crossterm::terminal::{Clear, ClearType};

use crate::buffer::Buffer;
use crate::common::Direction;
use crate::cursor;
use crate::vectors::Vec2;
use std::io::stdout;
use std::io::Error;
use std::io::Write;

pub struct View {
    stdout_handle: std::io::Stdout,
    buffer: Buffer,
    scroll_offset: Vec2,
    cursor_position: Vec2,
    needs_redraw: bool,
    statusline: String,
}

impl View {
    pub fn new(buffer: Buffer) -> Self {
        cursor::move_to(&mut stdout(), Vec2::line_start_with_gutter(0))
            .expect("failed to move cursor on window init");

        View {
            stdout_handle: stdout(),
            buffer,
            scroll_offset: Vec2::default(),
            cursor_position: Vec2::line_start_with_gutter(0),
            needs_redraw: true,
            statusline: String::new(),
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        self.needs_redraw = false;

        cursor::hide(&mut self.stdout_handle)?;
        self.draw_rows()?;
        self.draw_statusline()?;
        cursor::show(&mut self.stdout_handle)?;

        self.stdout_handle.flush()
    }

    fn draw_statusline(&mut self) -> Result<(), Error> {
        let last_line = self.get_bounds()?.y - 1;

        #[cfg(debug_assertions)]
        {
            self.needs_redraw = true;
            self.statusline = format!(
                "pos: ({}, {}), bounds: ({}, {}), offset: ({}, {}), cut: {:?}",
                self.cursor_position.x,
                self.cursor_position.y,
                self.get_bounds()?.x,
                self.get_bounds()?.y,
                self.scroll_offset.x,
                self.scroll_offset.y,
                self.character_at(self.cursor_position.projected_by(self.scroll_offset))
                    .unwrap_or_default()
            );
        }

        self.print(Vec2 { x: 0, y: last_line }, &self.statusline)?;
        self.restore_cursor()?;

        Ok(())
    }

    fn draw_rows(&mut self) -> Result<(), Error> {
        let Vec2 { y: rows, .. } = self.get_bounds()?;

        let lines_to_draw = &self.buffer.lines;

        for line_idx in 0..rows - 1 {
            self.clear_line(line_idx)?;

            let vertical_offset = (self.scroll_offset.y + line_idx) as usize;
            let horizontal_offset = self.scroll_offset.x as usize;

            if let Some(line_to_draw) = lines_to_draw.get(vertical_offset) {
                let line_to_draw: String = line_to_draw.chars().skip(horizontal_offset).collect();
                self.print(Vec2::line_start_with_gutter(line_idx), &line_to_draw)?;
            };
        }

        self.restore_cursor()?;
        Ok(())
    }

    // print prints a string at the given position. It does not restore the cursor position.
    fn print(&self, position: Vec2, content: &str) -> Result<Vec2, Error> {
        let mut stdout_handle = stdout();

        cursor::move_to(&mut stdout_handle, position)?;

        let print_command = crossterm::style::Print(content);

        queue!(&mut stdout_handle, print_command)?;

        Ok(position)
    }

    // clear_line clears a line. It does not restore the cursor position.
    fn clear_line(&self, line: u16) -> Result<(), Error> {
        let mut stdout_handle = stdout();

        cursor::move_to(&mut stdout_handle, Vec2::from_u16_tuple((0, line))).unwrap_or_default();
        let clear_cmd = Clear(ClearType::CurrentLine);

        queue!(stdout(), clear_cmd)?;

        Ok(())
    }

    pub fn move_cursor(&mut self, direction: Direction) -> Result<(), Error> {
        let absolute_pos = self.cursor_position.projected_by(self.scroll_offset);

        let wish_absolute_pos = self.next_valid_position_in_direction(absolute_pos, direction)?;
        let wish_terminal_pos = wish_absolute_pos.unprojected_from(self.scroll_offset);

        while !self.is_position_visible(wish_absolute_pos)? {
            let is_after_viewport = wish_absolute_pos.x > self.get_viewport()?.x;
            let is_before_viewport = wish_absolute_pos.x < self.scroll_offset.x;
            let is_above_viewport = wish_absolute_pos.y < self.get_viewport()?.y;
            let is_below_viewport = wish_absolute_pos.y > self.get_viewport()?.y;

            let direction = if is_after_viewport {
                Direction::Right
            } else if is_before_viewport {
                Direction::Left
            } else if is_above_viewport {
                Direction::Up
            } else if is_below_viewport {
                Direction::Down
            } else {
                Direction::None
            };

            self.scroll_offset = self.scroll_offset.shifted(direction);
            self.needs_redraw = true;
        }

        let clamped_wish_terminal_pos = self.clamp_to_bounds(wish_terminal_pos)?;
        self.cursor_position = cursor::move_to(&mut self.stdout_handle, clamped_wish_terminal_pos)?;

        Ok(())
    }

    pub fn resize(&mut self, new_dimensions: Vec2) -> Result<(), Error> {
        // TODO: snap cursor to fit within new dimensions
        self.needs_redraw = true;
        Ok(())
    }

    fn get_bounds(&self) -> Result<Vec2, Error> {
        let size = crossterm::terminal::size()?;
        Ok(Vec2::from_u16_tuple(size))
    }

    fn get_viewport(&self) -> Result<Vec2, Error> {
        let mut viewport = self.get_bounds()?.projected_by(self.scroll_offset);
        viewport.x -= 1;
        viewport.y -= 2;
        Ok(viewport)
    }

    fn is_position_visible(&self, absolute_position: Vec2) -> Result<bool, Error> {
        let viewport_end = self.get_viewport()?;

        Ok(absolute_position.is_between(self.scroll_offset, viewport_end))
    }

    fn clamp_to_bounds(&self, position: Vec2) -> Result<Vec2, Error> {
        let bounds = self.get_bounds()?;
        let clamped_x = position.x.clamp(0, bounds.x - 1);
        let clamped_y = position.y.clamp(0, bounds.y - 2);

        Ok(Vec2 {
            x: clamped_x,
            y: clamped_y,
        })
    }

    fn restore_cursor(&mut self) -> Result<(), Error> {
        cursor::move_to(&mut self.stdout_handle, self.cursor_position)?;
        Ok(())
    }

    fn end_of_line(&self, line: u16) -> Result<Vec2, Error> {
        let line_opt = self.buffer.lines.get(line as usize);

        match line_opt {
            Some(line_str) => Ok(Vec2 {
                x: line_str.len() as u16,
                y: line,
            }
            .shifted(Direction::Left)),
            None => Ok(Vec2 { x: 0, y: line }),
        }
    }

    fn start_of_line(&self, line: u16) -> Result<Vec2, Error> {
        Ok(Vec2 { x: 0, y: line })
    }

    fn character_at(&self, absolute_position: Vec2) -> Option<char> {
        self.buffer
            .lines
            .get(absolute_position.y as usize)?
            .chars()
            .nth(absolute_position.x as usize)
    }

    fn next_valid_position_in_direction(
        &self,
        position: Vec2,
        wish_direction: Direction,
    ) -> Result<Vec2, Error> {
        let wish_position = position.shifted(wish_direction);

        let is_end_of_line = self.character_at(wish_position).is_none();
        let is_start_of_line = position.x == 0 && wish_position.y != 0;
        let wish_line_smaller = self.end_of_line(wish_position.y)?.x < wish_position.x;

        match wish_direction {
            Direction::Up if wish_line_smaller => self.end_of_line(wish_position.y),
            Direction::Right if is_end_of_line => self.start_of_line(wish_position.y + 1),
            Direction::Down if wish_line_smaller => self.end_of_line(wish_position.y),
            Direction::Left if is_start_of_line => {
                self.end_of_line(wish_position.y.saturating_sub(1))
            }
            Direction::None => panic!("next valid position called with Direction::None"),
            _ => Ok(wish_position),
        }
    }
}
