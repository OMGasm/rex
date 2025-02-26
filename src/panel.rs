use crate::stuff::*;

pub struct Panel {
    position: Position,
    size: Size,
    cursor: Position,
}

impl Panel {
    pub const fn new(position: Position, size: Size) -> Self {
        Self {
            position,
            size,
            cursor: Position { x: 0, y: 0 },
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn bounds(&self) -> (Position, Position) {
        (self.position, self.position + self.size)
    }

    pub fn cursor(&self) -> &Position {
        &self.cursor
    }

    pub fn move_cursor(&mut self, dir: CursorMovement) -> CursorMove {
        let size = self.size;
        let ref mut cursor = self.cursor;

        match dir {
            CursorMovement::Left if cursor.x > 0 => {
                cursor.x -= 1;
                CursorMove::Moved
            }
            CursorMovement::Down if cursor.y < size.height - 1 => {
                cursor.y += 1;
                CursorMove::Moved
            }
            CursorMovement::Up if cursor.y > 0 => {
                cursor.y -= 1;
                CursorMove::Moved
            }
            CursorMovement::Right if cursor.x < size.width - 1 => {
                cursor.x += 1;
                CursorMove::Moved
            }
            CursorMovement::LeftMost => {
                cursor.x = 0;
                CursorMove::Moved
            }
            CursorMovement::Bottom => {
                cursor.y = size.height - 1;
                CursorMove::Moved
            }
            CursorMovement::Top => {
                cursor.y = 0;
                CursorMove::Moved
            }
            CursorMovement::RightMost => {
                cursor.x = size.width - 1;
                CursorMove::Moved
            }
            _ => CursorMove::Blocked,
        }
    }
}

pub enum CursorMove {
    Moved,
    Blocked,
}

pub enum CursorMovement {
    Left,
    Down,
    Up,
    Right,
    LeftMost,
    Bottom,
    Top,
    RightMost,
}
