pub struct Panel {
    position: (u16, u16),
    size: (u16, u16),
    cursor: (u16, u16),
}

impl Panel {
    pub fn new(position: (u16, u16), size: (u16, u16)) -> Self {
        Self {
            position,
            size,
            cursor: (0, 0),
        }
    }
}

#[derive(Debug)]
pub enum ActivePanel {
    Hex,
    Ascii,
}

impl ActivePanel {
    pub fn switch(&mut self) {
        *self = match *self {
            Self::Hex => Self::Ascii,
            Self::Ascii => Self::Hex,
        };
    }
}
