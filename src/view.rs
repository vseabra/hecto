use crossterm::queue;
use crossterm::terminal::{Clear, ClearType};

use crate::buffer::Buffer;
use crate::common::Direction;
use crate::{common::Position, cursor};
use std::io::stdout;
use std::io::Error;
use std::io::Write;

pub struct View {
    stdout_handle: std::io::Stdout,
    buffer: Buffer,
    scroll_offset: Position,
}

impl View {
    pub fn new(buffer: Buffer) -> Self {
        View {
            stdout_handle: stdout(),
            buffer,
            scroll_offset: Position { x: 0, y: 0 },
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        cursor::hide(&mut self.stdout_handle)?;

        self.draw_rows()?;

        cursor::show(&mut self.stdout_handle)?;
        self.stdout_handle.flush()
    }

    pub fn draw_rows(&mut self) -> Result<(), Error> {
        let (_, rows) = crossterm::terminal::size()?;

        let cursor_before_tuple = crossterm::cursor::position()?;
        let position_before = Position::from_u16_tuple(cursor_before_tuple);
        let lines_to_draw = &self.buffer.lines;

        for line_idx in 0..rows {
            self.clear_line(line_idx)?;
            // draw_gutter
            self.print(Position::from_u16_tuple((0, line_idx)), "~")?;

            let vertical_offset = (self.scroll_offset.y + line_idx) as usize;
            let horizontal_offset = self.scroll_offset.x as usize;

            if let Some(line_to_draw) = lines_to_draw.get(vertical_offset) {
                let line_to_draw: String = line_to_draw.chars().skip(horizontal_offset).collect();
                self.print(Position::line_start_with_gutter(line_idx), &line_to_draw)?;
            };
        }

        cursor::move_to(&mut self.stdout_handle, position_before)?;
        Ok(())
    }

    fn print(&self, position: Position, content: &str) -> Result<Position, Error> {
        let mut stdout_handle = stdout();

        cursor::move_to(&mut stdout_handle, position)?;

        let print_command = crossterm::style::Print(content);

        queue!(&mut stdout_handle, print_command)?;

        Ok(position)
    }

    // draw_line returns the position at the start of the next line
    // fn draw_line(&self, line: &str, starting_pos: Position) -> Result<Position, Error> {
    //     let (columns, _) = crossterm::terminal::size().unwrap_or_default();
    //     let will_wrap = line.len() > columns as usize;

    //     if !will_wrap {
    //         let end_pos = self.print(starting_pos, line)?;
    //         return Ok(Position {
    //             x: 2,
    //             y: end_pos.y + 1,
    //         });
    //     }

    //     let chunk_size = columns as usize - 2; // account for the gutterline
    //     let mut new_pos = starting_pos;
    //     let substrings_to_draw = line
    //         .chars()
    //         .collect::<Vec<char>>()
    //         .chunks(chunk_size)
    //         .map(|chunk| chunk.iter().collect())
    //         .collect::<Vec<String>>();

    //     for (i, substring) in substrings_to_draw.iter().enumerate() {
    //         let current_line = starting_pos.y + i as u16; // TODO this will fail on lines bigger
    //                                                       // than u16
    //         new_pos = Position::from_u16_tuple((2, current_line));

    //         self.print(new_pos, substring)?;
    //     }

    //     Ok(Position {
    //         x: 2,
    //         y: new_pos.y + 1,
    //     })
    // }

    fn clear_line(&self, line: u16) -> Result<(), Error> {
        let mut stdout_handle = stdout();

        cursor::move_to(&mut stdout_handle, Position::from_u16_tuple((0, line)))
            .unwrap_or_default();
        let clear_cmd = Clear(ClearType::CurrentLine);

        queue!(stdout(), clear_cmd)?;

        Ok(())
    }

    pub fn scroll(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.scroll_offset.y > 0 {
                    self.scroll_offset.y -= 1;
                }
            }
            Direction::Down => {
                self.scroll_offset.y += 1;
            }
            Direction::Left => {
                if self.scroll_offset.x > 0 {
                    self.scroll_offset.x -= 1;
                }
            }
            Direction::Right => {
                self.scroll_offset.x += 1;
            }
            Direction::None => {}
        }
    }
}
