use crossterm::event::KeyCode;

#[derive(PartialEq, Clone, Copy)]
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
