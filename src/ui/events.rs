use std::time::Duration;
use crossterm::event;
use crossterm::event::{KeyCode};


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

