use crate::buffer::Buffer;
use crate::common::EditorCommand;
use crate::view::View;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::{
    Event::{Key, Resize},
    KeyCode::Char,
};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::{stdout, Error, Write};

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new(initial_state: String) -> Self {
        let initial_buffer = Buffer::from_string(initial_state);
        Editor {
            should_quit: false,
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
        let mut stdout_handle = stdout();
        queue!(stdout_handle, EnterAlternateScreen)?;

        enable_raw_mode()?;

        Self::queue_clear_screen()?;
        stdout_handle.flush()
    }

    fn queue_clear_screen() -> Result<(), Error> {
        queue!(&mut stdout(), Clear(ClearType::All))
    }

    fn terminate() -> Result<(), Error> {
        let mut stdout_handle = stdout();
        disable_raw_mode()?;
        queue!(stdout_handle, LeaveAlternateScreen)?;
        queue!(stdout_handle, Print("\n"))?;
        stdout_handle.flush()
    }

    fn repl(&mut self) -> Result<(), Error> {
        self.view.render()?;

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

            self.view.render()?;

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
            EditorCommand::Move(direction) => self.view.move_cursor(direction)?,
            EditorCommand::Quit => self.should_quit = true,
            EditorCommand::Resize(new_dimensions) => self.view.resize(new_dimensions)?
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
}
