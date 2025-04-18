use editor::Editor;

mod editor;
mod cursor;
mod common;
mod view;

fn main() {
    Editor::default().run()
}
