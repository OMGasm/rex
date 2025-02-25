use std::{fs::File, io::{self, BufRead, BufReader, Seek, SeekFrom}};

use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Stylize,
    terminal::{BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate},
};

use crate::{file::FileCursor, panel::ActivePanel};

pub struct Editor {
    stdout: io::Stdout,
    file: Option<FileCursor<BufReader<File>>>,
    hex_panel: Panel,
    ascii_panel: Panel,
    active: ActivePanel,
    options: Options,
    cursor: (u16, u16),
}

#[derive(Debug, Default)]
pub struct Options {
    bytes_per_group: u16,
    groups_per_row: u16,
}

impl Editor {
    pub fn new(mut stdout: io::Stdout, options: Options) -> Self {
        Self {
            stdout,
            file: todo!(),
            hex_panel: todo!(),
            ascii_panel: todo!(),
            options,
            cursor: (0, 0),
            active: ActivePanel::Ascii
        }
    }

    pub fn display(&self) -> Result<(), EditorError> {
        let stdout = self.stdout;
        let Options{bytes_per_group, groups_per_row} = self.options;
        use std::io::Write;
        let bpr = bytes_per_group * groups_per_row;
        let divs = groups_per_row - 1;
        queue!(
            stdout,
            BeginSynchronizedUpdate,
            MoveTo(0, 0),
            Clear(ClearType::All)
        )?;
        let file = self.file.expect("there should be a file here!");
        let buf = file.buffer();
        let chunks = buf.chunks(bpr.into());

        // header - {minor offset...}
        write!(stdout, "{}", " ".repeat(8 + 2))?;
        for g in 0..groups_per_row {
            for x in 0..bytes_per_group {
                write!(stdout, " {:02X}", bytes_per_group * g + x)?;
            }
            write!(stdout, " ")?;
        }
        // header - ASCII
        write!(stdout, " ASCII\r\n\n")?;

        let file_pos = self.file.expect("there should be a file here!").position()?;
        // {offset}: [hex byte...] |[ascii]|
        for (l, c) in chunks.enumerate() {
            write!(stdout, "{:08X}: ", l as u64 * 10 + file_pos)?;
            for c in c.chunks(bytes_per_group.into()) {
                for c in c {
                    write!(stdout, " {c:02X}")?;
                }
                write!(stdout, " ")?;
            }

            let str = String::from_utf8_lossy(c);
            let str = str.replace('\n', &" ".on_dark_grey().to_string());
            write!(stdout, "|{str}|\r\n")?;
        }
        let x = match self.active {
            ActivePanel::Hex => cx * 3 + 8 + 3 + cx / bytes_per_group,
            ActivePanel::Ascii => 8 + 3 + bpr * 3 + divs + 1 + cx,
        };
        execute!(
            stdout,
            MoveTo(x.into(), 2 + u16::from(cy)),
            EndSynchronizedUpdate
        )?;
        Ok(())
    }

    pub fn switch_panel(&mut self, move_type: &PanelMovement) {
        let (current, other) = match self.active {
            ActivePanel::Hex => (self.hex_panel, self.ascii_panel),
            ActivePanel::Ascii => (self.ascii_panel, self.hex_panel),
        };

        match move_type {
            PanelMovement::LeftEdge => ,
            PanelMovement::RightEdge => {
                self.view_cursor.0 = self.bytes_per_group * self.groups_per_row - 1;
            }
            PanelMovement::KeepCursor => {}
        }
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
        let pos = self.file.position();
        Ok(())
    }

    pub fn scroll_up(&mut self) -> io::Result<()> {
        let (_, ref mut y) = self.view_cursor;
        Ok(())
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
