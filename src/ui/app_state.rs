use crossterm::event::{self, KeyCode, KeyEventKind};
use std::io::{Result, stdout};
use crossterm::cursor::{Hide, Show};
use crossterm::ExecutableCommand;
use crate::banking::auth::Credential;
use crate::banking::types::ID;

#[derive(PartialEq, Clone)]
pub enum Event {
    ButtonClicked(String),
    None
}

#[derive(Copy, Clone, PartialEq)]
pub enum InputMode {
    // App is reading inputs into a widget
    Input,
    // App is normally handling inputs
    Normal
}

#[derive(Copy, Clone)]
pub enum CursorMode {
    Show,
    Hidden
}

pub struct AppState {
    key_pressed: KeyCode,
    key_released: KeyCode,
    triggered_events: Vec<Event>,
    events_pool: Vec<Event>,
    input_mode: InputMode,
    cursor_mode: CursorMode,
    credential: Option<Credential>,

}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            key_released: KeyCode::Null,
            key_pressed: KeyCode::Null,
            triggered_events: Vec::new(),
            input_mode: InputMode::Normal,
            cursor_mode: CursorMode::Hidden,
            events_pool: Vec::new(),
            credential: None
        }
    }


    pub fn get_key_events(&mut self) {
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

    pub fn handle_events(&mut self) {
        // Fetch events
        self.get_key_events();

        // move triggered events to events_pool for consumers to read
        self.events_pool = self.triggered_events.clone();
        self.triggered_events.clear();

        // Handle input modes
        match self.key_pressed() {
            KeyCode::Enter => self.set_input_mode_to_input(),
            KeyCode::Esc => self.set_input_mode_to_normal(),
            _ => ()
        }

    }

    pub fn handle_cursor_modes(&self) {
        // cursor modes
        match self.cursor_mode() {
            CursorMode::Show => {
                if self.input_mode == InputMode::Input {
                    let _ = stdout().execute(Show);
                }
            }
            CursorMode::Hidden => {
                let _ = stdout().execute(Hide);
            }
        }
    }

    pub fn key_pressed(&self) -> KeyCode {
        self.key_pressed
    }
    pub fn key_released(&self) -> KeyCode {
        self.key_released
    }

    pub fn get_events(&self) -> &Vec<Event>{
        &self.events_pool
    }

    pub fn has_event(&self, event: Event) -> bool {
        for event_in_pool in self.events_pool.iter(){
            if event == *event_in_pool {
                return true;
            }
        }
        return false;
    }
    pub fn get_events_mut(&mut self) -> &mut Vec<Event>  {
        self.events_pool.as_mut()
    }
    pub fn push_event(&mut self, event: Event) {
        self.triggered_events.push(event);
    }
    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn cursor_mode(&self) -> CursorMode { self.cursor_mode }

    pub fn set_cursor_mode_to_show(&mut self) {
        self.cursor_mode = CursorMode::Show
    }

    pub fn set_cursor_mode_to_hidden(&mut self) {
        self.cursor_mode = CursorMode::Hidden
    }

    pub fn set_input_mode_to_input(&mut self) {
        self.input_mode = InputMode::Input
    }

    pub fn set_input_mode_to_normal(&mut self) {
        self.input_mode = InputMode::Normal
    }

    pub fn set_credentials(&mut self, id: ID, username: String, password: String) {
        self.credential = Some(Credential::new(id, username, password));
    }
}
