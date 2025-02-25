
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
