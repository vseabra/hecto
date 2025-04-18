use crate::common::{self, Position};
use crate::cursor::Cursor;
use crossterm::event::{read, Event, KeyEvent, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, queue};
use std::fmt::Display;
use std::io::{stdout, Error, Write};

pub struct Editor {
    should_quit: bool,
    cursor: Cursor,
    stdout_handle: std::io::Stdout,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            cursor: Cursor::default(),
            stdout_handle: stdout(),
        }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();

        match self.repl() {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {err:#?}");
            }
        }

        Self::terminate().unwrap();
    }

    fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;

        Self::clear_screen()
    }

    fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All))
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    fn repl(&mut self) -> Result<(), Error> {
        self.draw_rows()?;
        self.stdout_handle.flush()?;

        loop {
            self.draw_rows()?;

            let event = read()?;
            self.handle_event(&event)?;
            self.stdout_handle.flush()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                Char(char) => self.print(self.cursor.position, char)?,

                crossterm::event::KeyCode::Up => self.cursor.move_up(&mut self.stdout_handle)?,
                crossterm::event::KeyCode::Left => self.cursor.move_left_with_wrap(&mut self.stdout_handle)?,
                crossterm::event::KeyCode::Down => self.cursor.move_down(&mut self.stdout_handle)?,
                crossterm::event::KeyCode::Right => self.cursor.move_right_with_wrap(&mut self.stdout_handle)?,

                _ => {}
            }
        }

        Ok(())
    }

    fn print<T: Display>(
        &mut self,
        position: Position,
        content: T,
    ) -> Result<(), Error> {
        self.cursor.move_to(&mut self.stdout_handle, position)?;

        let print_command = crossterm::style::Print(content);

        queue!(&mut self.stdout_handle, print_command)?;

        self.cursor.move_right_with_wrap(&mut self.stdout_handle)

    }

    fn draw_rows(&mut self) -> Result<(), Error> {
        let (_, lines) = crossterm::terminal::size()?;
        self.cursor.hide(&mut self.stdout_handle)?;
        let cursor_before = self.cursor.position;

        for line in 0..lines {
            self.cursor.move_to(&mut self.stdout_handle, Position{x: 0, y: line})?;
            self.print(Position{x: 0, y: line}, "~")?;
        }

        self.cursor.move_to(&mut self.stdout_handle, cursor_before)?;
        self.cursor.show(&mut self.stdout_handle)
    }
}
