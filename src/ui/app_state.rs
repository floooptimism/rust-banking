use crossterm::event::{self, KeyCode, KeyEventKind};
use std::io::Result;

#[derive(Copy, Clone)]
pub enum Event {
    None
}

#[derive(Copy, Clone)]
pub enum InputMode {
    // App is reading inputs into a widget
    Input,
    // App is normally handling inputs
    Normal
}

pub struct AppState {
    key_pressed: KeyCode,
    key_released: KeyCode,
    triggered_event: Event,
    input_mode: InputMode
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            key_released: KeyCode::Null,
            key_pressed: KeyCode::Null,
            triggered_event: Event::None,
            input_mode: InputMode::Normal
        }
    }


    pub fn get_events(&mut self) {
        self.key_pressed = KeyCode::Null;
        self.key_released = KeyCode::Null;
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
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
    pub fn triggered_event(&self) -> Event {
        self.triggered_event
    }
    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn set_input_mode_to_input(&mut self) {
        self.input_mode = InputMode::Input
    }

    pub fn set_input_mode_to_normal(&mut self) {
        self.input_mode = InputMode::Normal
    }
}
