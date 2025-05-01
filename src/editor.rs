use crate::buffer::Buffer;
use crate::common::{Direction, EditorCommand, Position};
use crate::cursor;
use crate::view::View;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::{
    Event::{Key, Resize},
    KeyCode::Char,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
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

    /// refresh_screen redraws the screen if should_redraw is true and
    /// correctly resets the cursor_position
    fn refresh_screen(&mut self) -> Result<(), Error> {
        if self.should_redraw {
            self.view.render()?;
            self.should_redraw = false;
            cursor::move_to(&mut self.stdout_handle, self.cursor_position)?;
        };
        Ok(())
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)
    }

    fn repl(&mut self) -> Result<(), Error> {
        self.view.render()?;
        cursor::move_to(&mut self.stdout_handle, self.cursor_position)?;

        loop {
            let terminal_event = &read()?;

            let command = match terminal_event {
                Key(key_event) => self.handle_input(key_event),
                Resize(new_x, new_y) => Some(EditorCommand::Resize((*new_x, *new_y))),
                _ => None,
            };

            if let Some(command) = command {
                self.execute_command(command)?;
            }

            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    /// execute_command executes a command and returns a Result.
    /// it also adds the reverse command to the undo stack
    ///
    /// * `command`: the command to execute
    fn execute_command(&mut self, command: EditorCommand) -> Result<(), Error> {
        match command {
            EditorCommand::Move(direction) => self.move_cursor(direction)?,
            EditorCommand::Quit => self.should_quit = true,
            EditorCommand::Resize(_) => self.should_redraw = true, // TODO handle cursor position so that it stays in bounds
        }
        Ok(())
    }

    fn handle_input(&mut self, key_event: &KeyEvent) -> Option<EditorCommand> {
        let KeyEvent {
            code, modifiers, ..
        } = key_event;

        match code {
            Char('q') => {
                if *modifiers == KeyModifiers::CONTROL {
                    return Some(EditorCommand::Quit);
                }
                return None;
            }
            arrow_key @ KeyCode::Up
            | arrow_key @ KeyCode::Down
            | arrow_key @ KeyCode::Left
            | arrow_key @ KeyCode::Right => {
                return Some(EditorCommand::Move(arrow_key.into()));
            }
            _ => None,
        }
    }

    fn move_cursor(&mut self, direction: Direction) -> Result<(), Error> {
        let (new_position, scroll_direction) = match direction {
            Direction::Up => cursor::move_up(&mut self.stdout_handle)?,
            Direction::Down => cursor::move_down(&mut self.stdout_handle)?,
            Direction::Left => cursor::move_left(&mut self.stdout_handle)?,
            Direction::Right => cursor::move_right(&mut self.stdout_handle)?,
            Direction::None => (self.cursor_position, Direction::None),
        };

        self.cursor_position = new_position;

        if scroll_direction != Direction::None {
            self.view.scroll(scroll_direction);
            self.should_redraw = true;
        }

        Ok(())
    }
}
