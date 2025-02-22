use std::{fs::File, io::{self, BufRead, BufReader}};
use crossterm::{cursor::MoveTo, execute, queue, style::Stylize, terminal::{Clear, ClearType}};


#[derive(Debug)]
pub struct FileView {
    file: BufReader<File>,
    bytes_per_group: u8,
    groups_per_row: u8,
    rows: u8,
    view_cursor: (u8, u8),
    buffer_cursor_line: u8,
    panel: Panel,
}

impl FileView {
    pub fn new(file: BufReader<File>) -> FileView {
        FileView {
            file,
            bytes_per_group: 8,
            groups_per_row: 2,
            view_cursor: (0, 0),
            buffer_cursor_line: 0,
            panel: Panel::Ascii,
            rows: 10,
        }
    }

    pub fn display(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
        let buf = self.file.buffer();
        let chunks = buf.chunks(16);
        for c in chunks {
            let str = String::from_utf8_lossy(c);
            let str = str.replace('\n', &" ".on_dark_grey().to_string());
            use std::io::Write;
            write!(stdout, "{:02X}", c[0])?;
            for c in c.iter().skip(1) {
                write!(stdout, " {c:02X}")?;
            }

            write!(stdout, " |{}|\r\n", str)?;
        }
        let x = match self.panel {
            Panel::Hex => self.view_cursor.0 * 3,
            Panel::Ascii => 16 * 3 + 1 + self.view_cursor.0,
        };
        execute!(stdout, MoveTo(x.into(), self.view_cursor.1.into()))?;
        Ok(())
    }

    pub fn switch_panel(&mut self, move_type: PanelMovement) {
        self.panel.switch();
        match move_type {
            PanelMovement::LeftEdge => self.view_cursor.0 = 0,
            PanelMovement::RightEdge => self.view_cursor.0 = self.bytes_per_group * self.groups_per_row - 1,
            PanelMovement::KeepCursor => {},
        }
    }

    pub fn cursor(&mut self) -> &mut (u8, u8) {
        &mut self.view_cursor
    }

    pub fn cursor_left(&mut self) -> CursorMovement {
        if self.view_cursor.0 == 0 {
            CursorMovement::StuckEdge
        } else {
            self.view_cursor.0 -= 1;
            CursorMovement::Moved
        }
    }

    pub fn cursor_right(&mut self) -> CursorMovement {
        if self.view_cursor.0 == 15 {
            CursorMovement::StuckEdge
        } else {
            self.view_cursor.0 += 1;
            CursorMovement::Moved
        }
    }

    pub fn scroll_down(&mut self) -> io::Result<()> {
        let (_, ref mut y) = self.view_cursor;
        if *y == self.rows - 1 {
            self.file.seek_relative(16)?;
            self.file.fill_buf()?;
        } else {
            *y += 1;
        }
        Ok(())
    }

    pub fn scroll_up(&mut self) -> io::Result<()> {
        let (_, ref mut y) = self.view_cursor;
        if *y == 0 {
            self.file.seek_relative(-16)?;
            self.file.fill_buf()?;
        } else {
            *y -= 1;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Panel {
    Hex,
    Ascii,
}

impl Panel {
    pub fn switch(&mut self) {
        *self = match *self {
            Self::Hex => Self::Ascii,
            Self::Ascii => Self::Hex,
        };
    }

    pub fn current(&self) -> Self {
        match *self {
            Panel::Hex => Self::Hex,
            Panel::Ascii => Self::Ascii,
        }
    }
}

pub enum CursorMovement {
    StuckEdge,
    Moved,
}

pub enum PanelMovement {
    LeftEdge,
    RightEdge,
    KeepCursor,
}
