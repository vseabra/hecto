use crate::buffer::Buffer;
use crate::common::{Direction, Position};
use crate::cursor;
use crate::view::View;
use crossterm::event::{read, Event, KeyEvent, KeyModifiers};
use crossterm::event::{
    Event::{Key, Resize},
    KeyCode::Char,
};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{stdout, Error};

pub struct Editor {
    should_quit: bool,
    stdout_handle: std::io::Stdout,
    cursor_position: Position,
    should_redraw: bool,
    view: View,
}

impl Editor {
    pub fn new(initial_state: String) -> Self {
        let initial_buffer = Buffer::from_string(initial_state);
        Editor {
            should_quit: false,
            should_redraw: true,
            stdout_handle: stdout(),
            cursor_position: Position { x: 2, y: 0 },
            view: View::new(initial_buffer),
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
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        Self::clear_screen()
    }

    fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All))
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)
    }

    fn repl(&mut self) -> Result<(), Error> {
        self.view.render()?;
        cursor::move_to(&mut self.stdout_handle, self.cursor_position)?;

        loop {
            if self.should_redraw {
                self.view.render()?;
                self.should_redraw = false;
            };
            // after drawing the view, we need to move the cursor to the current position
            cursor::move_to(&mut self.stdout_handle, self.cursor_position)?;

            let event = read()?;
            self.handle_event(&event)?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), Error> {
        let mut scroll_direction = Direction::None;

        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                // Char(char) => self.print(self.cursor.position, char)?,
                crossterm::event::KeyCode::Up => {
                    (self.cursor_position, scroll_direction) = cursor::move_up(&mut self.stdout_handle)?;
                }
                crossterm::event::KeyCode::Left => {
                    (self.cursor_position, scroll_direction) = cursor::move_left(&mut self.stdout_handle)?;
                }
                crossterm::event::KeyCode::Down => {
                    (self.cursor_position, scroll_direction) = cursor::move_down(&mut self.stdout_handle)?;
                }
                crossterm::event::KeyCode::Right => {
                    (self.cursor_position, scroll_direction) = cursor::move_right(&mut self.stdout_handle)?;
                }

                _ => {}
            }

            // self.should_redraw = true;
        }

        if scroll_direction != Direction::None {
            self.should_redraw = true;
            self.view.scroll(scroll_direction);
        }

        if let Resize(_, _) = event {
            self.should_redraw = true;
        }

        Ok(())
    }
}
