use crossterm::event::KeyCode;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn from_u16_tuple(position: (u16, u16)) -> Self {
        Position {
            x: position.0,
            y: position.1,
        }
    }

    pub fn line_start_with_gutter(line: u16) -> Self {
        Position { x: 2, y: line }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 2, y: 0 }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl From<&KeyCode> for Direction {
    fn from(key_code: &KeyCode) -> Self {
        match key_code {
            KeyCode::Up => Direction::Up,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            KeyCode::Down => Direction::Down,
            _ => Direction::None,
        }
    }
}

pub enum EditorCommand {
    Move(Direction),
    Resize((u16, u16)),
    Quit,
}
