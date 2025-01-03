use std::cmp::{max, min};
use std::io::{stdout, Error, Write};

use crossterm::cursor::MoveToColumn;

use crossterm::event::KeyCode::{Backspace, Char, Delete, End, Enter, Esc, Home, Left, Right};
use crossterm::event::{read, Event, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue};

use crate::mod_display::DisplayInfo;

struct Config {
    magnify_step: f64,
}

static CONFIG: Config = Config { magnify_step: 0.1 };

pub fn handle_events(info: &mut DisplayInfo) -> Result<bool, Error> {
    match read() {
        Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
            handle_key_events(key_event, info)
        }
        Err(e) => Err(e),
        _ => Ok(true),
    }
}

fn handle_key_events(key_event: KeyEvent, info: &mut DisplayInfo) -> Result<bool, Error> {
    match key_event.modifiers {
        // simple key
        KeyModifiers::NONE => match key_event.code {
            Char('q') => Ok(false),
            Char('o') => {
                let file_path = input_box("input file path: ")?;
                execute!(
                    stdout(),
                    Clear(ClearType::CurrentLine),
                    Print(format!("file_path: {}", file_path)),
                )
                .unwrap();
                info.image_file_path = file_path.clone();
                Ok(true)
            }
            Char('-') => {
                if info.magnify >= 1.0 + CONFIG.magnify_step {
                    info.magnify -= CONFIG.magnify_step;
                }
                Ok(true)
            }
            _ => Ok(true),
        },
        // ctrl pressed
        KeyModifiers::CONTROL => match key_event.code {
            Char('c') => Ok(false),
            _ => Ok(true),
        },
        KeyModifiers::SHIFT => match key_event.code {
            Char('+') => {
                info.magnify += CONFIG.magnify_step;
                Ok(true)
            }
            _ => Ok(true),
        },
        _ => Ok(true),
    }
}

fn input_box(input_msg: &str) -> Result<String, Error> {
    // show input message
    execute!(stdout(), Clear(ClearType::CurrentLine), Print(input_msg),).unwrap();
    stdout().flush().unwrap();

    // get input
    let mut input = String::new();
    let mut pos = 0;
    loop {
        match read() {
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    Char(c) => {
                        if c == 'c' && key_event.modifiers == KeyModifiers::CONTROL {
                            return Err(Error::new(
                                std::io::ErrorKind::Interrupted,
                                "Input was canceled by user",
                            ));
                        }

                        input.insert(pos, c);
                        pos += 1;
                    }
                    Backspace => {
                        if pos > 0 {
                            input.remove(pos - 1);
                            pos -= 1;
                        }
                    }
                    Delete => {
                        if pos < input.len() {
                            input.remove(pos);
                        }
                    }
                    Enter => {
                        println!();
                        return Ok(input);
                    }

                    Left => pos = max(0, pos - 1),
                    Right => pos = min(input.len(), pos + 1),
                    Home => pos = 0,
                    End => pos = input.len(),

                    Esc => {
                        return Err(Error::new(
                            std::io::ErrorKind::Interrupted,
                            "Input was canceled by user",
                        ));
                    }
                    _ => {}
                }
                queue!(stdout(), MoveToColumn(0)).unwrap();
                queue!(stdout(), Print(input_msg), Print(&input)).unwrap();
                queue!(stdout(), Clear(ClearType::UntilNewLine)).unwrap();
                queue!(stdout(), MoveToColumn(input_msg.len() as u16 + pos as u16)).unwrap();
                stdout().flush().unwrap();
            }
            Err(e) => {
                return Err(e);
            }
            _ => {}
        }
    }
}
