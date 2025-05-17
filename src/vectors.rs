use crate::common::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    pub fn from_u16_tuple(position: (u16, u16)) -> Self {
        Vec2 {
            x: position.0,
            y: position.1,
        }
    }

    pub fn line_start_with_gutter(line: u16) -> Self {
        Vec2 { x: 0, y: line }
    }

    pub fn projected_by(&self, other: Vec2) -> Self {
        return Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }

    pub fn unprojected_from(&self, other: Vec2) -> Self {
        return Vec2 {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        };
    }

    pub fn is_between(&self, start: Vec2, end: Vec2) -> bool {
        let is_after_end = self.x >= start.x && self.y >= start.y;
        let is_before_end = self.x <= end.x && self.y <= end.y;

        is_after_end && is_before_end
    }

    pub fn shifted(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Vec2 {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Direction::Right => Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Vec2 {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
            Direction::None => *self,
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2 { x: 0, y: 0 }
    }
}
