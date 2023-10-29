use std::time::Duration;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};


pub enum Event {
    KeyCode(KeyCode),
    Exit,
    None
}

pub fn has_event() -> bool {
    match event::poll(std::time::Duration::from_millis(0)){
        Ok(val) => val,
        _ => false
    }
}

pub fn is_key_event() -> bool {
    match event::read() {
        Ok(event) => {
            match event {
                event::Event::Key(key) => true,
                _ => false
            }
        },
        _ => false
    }
}

pub fn is_key_pressed(key_input: char) -> bool {
    if event::poll(std::time::Duration::from_millis(0)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(key_input) {
                return true;
            }
        }
    }
    return false;
}

pub fn get_key_pressed(duration: Option<u64>) -> KeyCode {
    if event::poll(std::time::Duration::from_millis(duration.unwrap_or(0))).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                return key.code;
            }
        }
    }
    KeyCode::Null
}

