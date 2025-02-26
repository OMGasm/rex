#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl From<Size> for Position {
    fn from(Size { width, height }: Size) -> Self {
        Position {
            x: width,
            y: height,
        }
    }
}

impl From<Position> for Size {
    fn from(Position { x, y }: Position) -> Self {
        Size {
            width: x,
            height: y,
        }
    }
}

impl From<Size> for (u16, u16) {
    fn from(Size { width, height }: Size) -> Self {
        (width, height)
    }
}

impl From<Position> for (u16, u16) {
    fn from(Position { x, y }: Position) -> Self {
        (x, y)
    }
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Self { width, height }
    }
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

impl<R: Into<Size>> std::ops::Add<R> for Size {
    type Output = Size;
    fn add(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Size {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl<R: Into<Position>> std::ops::Add<R> for Position {
    type Output = Position;
    fn add(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
