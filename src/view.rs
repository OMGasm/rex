use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Stylize,
    terminal::{BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom},
};

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
        use std::io::Write;
        let bpr = self.bytes_per_group * self.groups_per_row;
        let divs = self.groups_per_row - 1;
        queue!(
            stdout,
            BeginSynchronizedUpdate,
            MoveTo(0, 0),
            Clear(ClearType::All)
        )?;
        let buf = self.file.buffer();
        let chunks = buf.chunks(bpr.into());
        write!(stdout, "{}", " ".repeat(8 + 2))?;
        for g in 0..self.groups_per_row {
            for x in 0..self.bytes_per_group {
                write!(stdout, " {:02X}", self.bytes_per_group * g + x)?;
            }
            write!(stdout, " ")?;
        }
        write!(stdout, "ASCII\r\n\n")?;
        for (l, c) in chunks.enumerate() {
            write!(stdout, "{:08X}: ", l + self.buffer_cursor_line as usize)?;
            for c in c.chunks(self.bytes_per_group.into()) {
                for c in c.iter().skip(1) {
                    write!(stdout, " {c:02X}")?;
                }
            }

            let str = String::from_utf8_lossy(c);
            let str = str.replace('\n', &" ".on_dark_grey().to_string());
            write!(stdout, "\x08|{}|\r\n", str)?;
        }
        let (cx, cy) = self.view_cursor;
        let x = match self.panel {
            Panel::Hex => cx * 3,
            Panel::Ascii => bpr * 3 + divs + 1 + cx,
        };
        write!(
            stdout,
            "{:08X}: ",
            self.buffer_cursor_line as u64 * bpr as u64
        )?;
        execute!(stdout, MoveTo(x.into(), cy.into()), EndSynchronizedUpdate)?;
        Ok(())
    }

    pub fn switch_panel(&mut self, move_type: PanelMovement) {
        self.panel.switch();
        match move_type {
            PanelMovement::LeftEdge => self.view_cursor.0 = 0,
            PanelMovement::RightEdge => {
                self.view_cursor.0 = self.bytes_per_group * self.groups_per_row - 1
            }
            PanelMovement::KeepCursor => {}
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
        if self.view_cursor.0 == self.bytes_per_group * self.groups_per_row - 1 {
            CursorMovement::StuckEdge
        } else {
            self.view_cursor.0 += 1;
            CursorMovement::Moved
        }
    }

    pub fn scroll_down(&mut self) -> io::Result<()> {
        let (_, ref mut y) = self.view_cursor;
        let pos = self.file.stream_position()?;
        if *y == self.rows - 1 {
            self.file.rewind()?;
            self.file.seek(SeekFrom::Start(
                pos + self.bytes_per_group as u64 * self.groups_per_row as u64,
            ))?;
            self.file.fill_buf()?;
            self.buffer_cursor_line += 1;
        } else {
            *y += 1;
        }
        Ok(())
    }

    pub fn scroll_up(&mut self) -> io::Result<()> {
        let (_, ref mut y) = self.view_cursor;
        if *y == 0 {
            let pos = self.file.stream_position()?;
            let bpr = self.bytes_per_group * self.groups_per_row;
            self.file
                .seek(SeekFrom::Start(pos.saturating_sub(bpr as u64)))?;
            self.file.fill_buf()?;
            self.buffer_cursor_line = self.buffer_cursor_line.saturating_sub(1);
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
