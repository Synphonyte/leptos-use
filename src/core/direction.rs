/// Direction enum
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Copy, Clone, Default, Debug)]
/// Directions flags
pub struct Directions {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl Directions {
    /// Returns the value of the provided direction
    pub fn get_direction(&self, direction: Direction) -> bool {
        match direction {
            Direction::Top => self.top,
            Direction::Bottom => self.bottom,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }

    /// Sets the value of the provided direction
    pub fn set_direction(mut self, direction: Direction, value: bool) -> Self {
        match direction {
            Direction::Top => self.top = value,
            Direction::Bottom => self.bottom = value,
            Direction::Left => self.left = value,
            Direction::Right => self.right = value,
        }

        self
    }
}
