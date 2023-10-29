use crossterm::event::{self, KeyCode, KeyEventKind};
use std::io::Result;
pub struct AppState {
    key_pressed: KeyCode,
    key_released: KeyCode
}

impl AppState {

    pub fn new() -> AppState {
        AppState {
            key_released: KeyCode::Null,
            key_pressed: KeyCode::Null
        }
    }


    pub fn get_events(&mut self) {
        self.key_pressed = KeyCode::Null;
        self.key_released = KeyCode::Null;
        if event::poll(std::time::Duration::from_millis(0)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.kind {
                    KeyEventKind::Press => self.key_pressed = key.code,
                    KeyEventKind::Release => self.key_released = key.code,
                    _ => ()
                }
            }
        }
    }
    pub fn key_pressed(&self) -> KeyCode {
        self.key_pressed
    }
    pub fn key_released(&self) -> KeyCode {
        self.key_released
    }
}
