#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
    DiagUpLeft,
    DiagUpRight,
    DiagDownLeft,
    DiagDownRight,
}

impl Orientation {
    pub fn diagonal(orientation: Self) -> bool {
        match orientation {
            Self::DiagDownLeft | Self::DiagDownRight | Self::DiagUpLeft | Self::DiagUpRight => true,
            _ => false,
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::DiagDownLeft => Self::DiagUpRight,
            Self::DiagDownRight => Self::DiagUpLeft,
            Self::DiagUpLeft => Self::DiagDownRight,
            Self::DiagUpRight => Self::DiagDownLeft,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Direction {
    pub orientation: Orientation,
    pub row: usize,
    pub col: usize,
}

impl Direction {
    pub fn new(orientation: Orientation, row: usize, col: usize) -> Self {
        Self {
            orientation,
            row,
            col,
        }
    }

    pub fn advance(&self) -> Self {
        match self.orientation {
            Orientation::Up => {
                Self::new(self.orientation, self.row.wrapping_add_signed(-1), self.col)
            }
            Orientation::Down => {
                Self::new(self.orientation, self.row.wrapping_add_signed(1), self.col)
            }
            Orientation::Left => {
                Self::new(self.orientation, self.row, self.col.wrapping_add_signed(-1))
            }
            Orientation::Right => {
                Self::new(self.orientation, self.row, self.col.wrapping_add_signed(1))
            }
            Orientation::DiagDownLeft => Self::new(
                self.orientation,
                self.row.wrapping_add_signed(1),
                self.col.wrapping_add_signed(-1),
            ),
            Orientation::DiagDownRight => Self::new(
                self.orientation,
                self.row.wrapping_add_signed(1),
                self.col.wrapping_add_signed(1),
            ),
            Orientation::DiagUpLeft => Self::new(
                self.orientation,
                self.row.wrapping_add_signed(-1),
                self.col.wrapping_add_signed(-1),
            ),
            Orientation::DiagUpRight => Self::new(
                self.orientation,
                self.row.wrapping_add_signed(-1),
                self.col.wrapping_add_signed(1),
            ),
        }
    }
}
