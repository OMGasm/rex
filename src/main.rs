use clap::Parser;
use crossterm::{
    cursor::MoveTo, event::{self, Event}, execute, queue, style::Stylize, terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    }
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser, Debug)]
struct CliArgs {
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let args = CliArgs::parse();
    eprintln!("Hello, world!\n{:?}", args);
    let file = File::open(args.file).expect("File not found");
    let rows = 10;
    let mut file = BufReader::with_capacity(rows * 16, file);
    file.fill_buf()?;
    let mut view = FileView::new(file);

    let mut stdout = io::stdout();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    view.display(&mut stdout)?;
    let res = loopy(&mut stdout, &mut view);

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    if let Err(e) = res {
        eprintln!("{e}");
    };
    Ok(())
}

fn loopy(stdout: &mut std::io::Stdout, view: &mut FileView) -> io::Result<()> {
    loop {
        let (ref mut cx, ref mut cy) = view.cursor;
        let event = event::read()?;
        if event == Event::Key(event::KeyCode::Left.into()) {
            if *cx == 0 {
                *cx = 15;
                view.panel.switch();
            } else {
                *cx -= 1;
            }
        }
        if event == Event::Key(event::KeyCode::Down.into()) {
            if *cy == view.rows - 1 {
                view.file.seek_relative(16)?;
                view.file.fill_buf()?;
            } else {
                *cy += 1;
            }
        }
        if event == Event::Key(event::KeyCode::Up.into()) {
            if *cy == 0 {
                view.file.seek_relative(-16)?;
                view.file.fill_buf()?;
            } else {
                *cy -= 1;
            }
        }
        if event == Event::Key(event::KeyCode::Right.into()) {
            if *cx == 15 {
                *cx = 0;
                view.panel.switch();
            } else {
                *cx += 1;
            }
        }

        if event
            == Event::Key(event::KeyEvent::new(
                event::KeyCode::Char('c'),
                event::KeyModifiers::CONTROL,
            ))
        {
            execute!(stdout, LeaveAlternateScreen)?;
            disable_raw_mode()?;
            eprintln!("Exited via break.");
            std::process::exit(0);
        }
        if event == Event::Key(event::KeyCode::Char('q').into()) {
            break;
        }
        if event == Event::Key(event::KeyCode::Esc.into()) {
            break;
        }

        view.display(stdout)?;
    }
    Ok(())
}

#[derive(Debug)]
struct FileView {
    file: BufReader<File>,
    bytes_per_group: u8,
    groups_per_row: u8,
    rows: u8,
    cursor: (u8, u8),
    panel: Panel,
}

#[derive(Debug)]
enum Panel {
    Hex,
    Ascii,
}

impl Panel {
    fn switch(&mut self) {
        *self = match *self {
            Self::Hex => Self::Ascii,
            Self::Ascii => Self::Hex,
        };
    }

    fn current(&self) -> Self {
        match *self {
            Panel::Hex => Self::Hex,
            Panel::Ascii => Self::Ascii,
        }
    }
}

impl FileView {
    pub fn new(file: BufReader<File>) -> FileView {
        FileView {
            file,
            bytes_per_group: 8,
            groups_per_row: 2,
            cursor: (0, 0),
            panel: Panel::Ascii,
            rows: 10,
        }
    }

    pub fn display(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
        println!("{}", String::from_utf8_lossy(self.file.buffer()).replace('\n', &" ".on_dark_grey().to_string()));
        let x = match self.panel {
            Panel::Hex => self.cursor.0 * 3,
            Panel::Ascii => 16 * 3 + 1 + self.cursor.0,
        };
        execute!(stdout, MoveTo(x.into(), self.cursor.1.into()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use super::*;
        use clap::CommandFactory;

        CliArgs::command().debug_assert();
    }
}
