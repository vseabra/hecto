use std::io::stdout;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use editor::Editor;

mod buffer;
mod common;
mod cursor;
mod editor;
mod view;
mod vectors;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut content = String::new();

    if let Some(filename) = args.get(1) {
        let file_content = std::fs::read_to_string(filename);

        match file_content {
            Ok(lines) => content = lines,
            Err(error) => panic!("{}", error),
        };
    }

    let current_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        current_hook(panic_info);
    }));

    Editor::new(content).run()
}
