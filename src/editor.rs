use crossterm::event::{read, Event, KeyEvent, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Error};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
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
        loop {
            let event = read()?;

            self.handle_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        if self.should_quit {
            Self::clear_screen()?;
            println!("Goodbye!");
        }
        Ok(())
    }
}
