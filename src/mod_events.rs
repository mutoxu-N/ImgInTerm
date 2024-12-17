use std::io::Error;

use crossterm::event::{read, Event, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_events() -> Result<bool, Error> {
    match read() {
        Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
            handle_key_events(key_event)
        }
        Err(e) => Err(e),
        _ => Ok(true),
    }
}

fn handle_key_events(key_event: KeyEvent) -> Result<bool, Error> {
    println!("key_event: {:?}, {:?}", key_event.modifiers, key_event.code);

    match key_event.modifiers {
        // ctrl pressed
        KeyModifiers::CONTROL => match key_event.code {
            crossterm::event::KeyCode::Char('c') => Ok(false),
            _ => Ok(true),
        },
        KeyModifiers::NONE => match key_event.code {
            crossterm::event::KeyCode::Char('q') => Ok(false),
            _ => Ok(true),
        },
        _ => Ok(true),
    }
}
