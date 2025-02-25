use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

use crate::editor::{CursorMovement, Editor, PanelMovement};

pub struct TermInput;

impl TermInput {
    pub fn event_loop(stdout: &mut std::io::Stdout, view: &mut Editor) -> io::Result<()> {
        loop {
            let event = event::read()?;
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => {
                    if let CursorMovement::StuckEdge = view.cursor_left() {
                        view.switch_panel(&PanelMovement::RightEdge);
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
                    if let CursorMovement::StuckEdge = view.cursor_right() {
                        view.switch_panel(&PanelMovement::LeftEdge);
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
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
                }) => view.switch_panel(&PanelMovement::KeepCursor),
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
}
