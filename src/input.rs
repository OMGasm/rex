use std::io;

use crate::{
    editor::Action,
    stuff::{Position, Size},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct TermInput;

impl TermInput {
    pub fn poll_event() -> io::Result<Action> {
        let event = event::read()?;
        Ok(match event {
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => Action::Left,
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => Action::Down,
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => Action::Up,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => Action::Right,

            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => Action::Quit,
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => Action::Quit,
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => Action::Quit,
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                ..
            }) => Action::SwitchPanel,
            Event::FocusGained => Action::OnFocus,
            Event::FocusLost => Action::OnBlur,
            Event::Mouse(e) => Action::Mouse(Position::from((e.column, e.row))),
            Event::Paste(s) => Action::Paste(s),
            Event::Resize(w, h) => Action::Resize(Size::from((w, h))),
            Event::Key(_) => Action::UnboundKey,
        })
    }
}
