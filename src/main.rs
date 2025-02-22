mod view;
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};
use view::{FileView, PanelMovement};

#[derive(Parser, Debug)]
struct CliArgs {
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let args = CliArgs::parse();
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
        let event = event::read()?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if let view::CursorMovement::StuckEdge = view.cursor_left() {
                    view.switch_panel(PanelMovement::RightEdge);
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                view.scroll_down()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                view.scroll_up()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if let view::CursorMovement::StuckEdge = view.cursor_right() {
                    view.switch_panel(PanelMovement::LeftEdge);
                }
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                execute!(stdout, LeaveAlternateScreen)?;
                disable_raw_mode()?;
                eprintln!("Exited via break.");
                std::process::exit(0);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                ..
            }) => view.switch_panel(PanelMovement::KeepCursor),
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
            Event::Key(_) => {}
        }

        view.display(stdout)?;
    }
    Ok(())
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
