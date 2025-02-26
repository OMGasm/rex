use crate::{
    file::{CursorDirection, CursorError, FileCursor},
    input::TermInput,
    panel::Panel,
    stuff::{Position, Size},
};
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Stylize,
    terminal::{
        disable_raw_mode, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate,
        LeaveAlternateScreen,
    },
};
use std::io::{self};

pub struct Editor {
    stdout: io::Stdout,
    file: Option<FileCursor>,
    hex_panel: Panel,
    ascii_panel: Panel,
    active: ActivePanel,
    options: Options,
    cursor: Position,
}

#[derive(Debug, Default)]
pub struct Options {
    pub bytes_per_group: u16,
    pub groups_per_row: u16,
}

impl Editor {
    pub fn new(stdout: io::Stdout, options: Options) -> Self {
        let Options {
            bytes_per_group,
            groups_per_row,
        } = options;
        let bpr = bytes_per_group * groups_per_row;
        let hex_panel = Panel::new((0, 1).into(), (bpr * 3 - 1, 10).into());
        let hex_bounds = hex_panel.bounds();
        let ascii_pos = Position::from((hex_bounds.1.x + 1, hex_bounds.0.y));
        let ascii_panel = Panel::new(ascii_pos, Size::from((bpr, 10)));
        Self {
            stdout,
            file: None,
            hex_panel,
            ascii_panel,
            options,
            cursor: ascii_pos,
            active: ActivePanel::Ascii,
        }
    }

    pub fn open_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), EditorError> {
        let file = std::fs::File::open(path)?;
        let Options {
            bytes_per_group,
            groups_per_row,
        } = self.options;

        let bpr = bytes_per_group as usize * groups_per_row as usize;
        let file = FileCursor::new(file, bpr * 10, bpr as u64);
        self.file = Some(file);
        self.display()
    }

    pub fn display(&mut self) -> Result<(), EditorError> {
        let mut stdout = &self.stdout;
        let Options {
            bytes_per_group,
            groups_per_row,
        } = self.options;
        use std::io::Write;
        let bpr = bytes_per_group * groups_per_row;
        let divs = groups_per_row - 1;
        queue!(
            stdout,
            BeginSynchronizedUpdate,
            MoveTo(0, 0),
            Clear(ClearType::All)
        )?;
        let file = self.file.as_mut().expect("there should be a file here!");
        let file_pos = file.position()?;
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
        let cur = self.active_panel().cursor();
        execute!(stdout, MoveTo(cur.x, 2 + cur.y), EndSynchronizedUpdate)?;
        Ok(())
    }

    fn active_panel(&self) -> &Panel {
        match self.active {
            ActivePanel::Hex => &self.hex_panel,
            ActivePanel::Ascii => &self.ascii_panel,
        }
    }

    pub fn switch_panel(&mut self, move_type: &PanelMovement) {
        let other = match self.active {
            ActivePanel::Hex => &self.ascii_panel,
            ActivePanel::Ascii => &self.hex_panel,
        };
        let ob = other.bounds();

        match move_type {
            PanelMovement::LeftEdge => {
                self.cursor.x = ob.0.x;
            }
            PanelMovement::RightEdge => {
                self.cursor.x = ob.1.x;
            }
            PanelMovement::KeepCursor => {
                self.cursor.x = other.cursor().x;
            }
        }
    }

    pub fn scroll(&mut self, direction: CursorDirection) -> Result<(), EditorError> {
        let file = self.file.as_mut().unwrap();
        file.scroll(direction);
        Ok(())
    }

    pub fn event_loop(&mut self) -> Result<(), EditorError> {
        loop {
            let event = match TermInput::poll_event() {
                Ok(event) => event,
                Err(e) => return self.quit(Some(e)),
            };

            match event {
                Action::Left => self.active_panel(),
                Action::Down => todo!(),
                Action::Up => todo!(),
                Action::Right => todo!(),
                Action::SwitchPanel => todo!(),
                Action::OnFocus => todo!(),
                Action::OnBlur => todo!(),
                Action::Mouse(position) => todo!(),
                Action::Paste(_) => todo!(),
                Action::Resize(size) => todo!(),
                Action::UnboundKey => todo!(),
                Action::Quit => todo!(),
            };
            self.display()?
        }
    }

    fn quit<E: std::error::Error>(&mut self, err: Option<E>) -> Result<(), EditorError> {
        execute!(self.stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        if let Some(err) = err {
            eprintln!("{err}");
        }
        std::process::exit(0);
    }
}

#[derive(Debug)]
pub enum EditorError {}

impl From<CursorError> for EditorError {
    fn from(value: CursorError) -> Self {
        todo!()
    }
}

impl From<io::Error> for EditorError {
    fn from(value: io::Error) -> Self {
        todo!()
    }
}

pub enum PanelMovement {
    LeftEdge,
    RightEdge,
    KeepCursor,
}

pub enum Action {
    Left,
    Down,
    Up,
    Right,
    SwitchPanel,
    OnFocus,
    OnBlur,
    Mouse(Position),
    Paste(String),
    Resize(Size),
    UnboundKey,
    Quit,
}

#[derive(Debug)]
pub enum ActivePanel {
    Hex,
    Ascii,
}
