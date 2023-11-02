use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::{self, Result, stdout};

use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::ui::container_helper::{apply_margins, get_main_layout_rect, Margin};
use crate::ui::traits::component::{Component};
use crate::ui::widgets::input::{Input};
use crossterm::event::{self, KeyCode, KeyEventKind};
use crate::{AppState};

enum InputFocus {
    Username,
    Password,
    None
}


pub struct Welcome {
    counter: i64,
    text: Vec<char>,
    username_input: Input,
    password_input: Input,
    input_focus: InputFocus
}

impl Welcome {
    pub fn new() -> Welcome {
        let input = Input::new("Username");
        let mut pass_input = Input::new("Password");
        pass_input.set_formatter(Some('*'));


        let mut instance = Welcome {
            counter: 0,
            text: Vec::new(),
            username_input: input,
            password_input: pass_input,
            input_focus: InputFocus::Username
        };

        instance.apply_focus();
        instance
    }

    pub fn handle_input_events(&mut self, app_state: &mut AppState) {
        match app_state.key_pressed() {
            KeyCode::Down | KeyCode::Up => {
                self.switch_focus();
                self.apply_focus();
            },
            _ => ()
        }

        let _ = self.username_input.handle_events(app_state);
        let _ = self.password_input.handle_events(app_state);

    }

    fn switch_focus(&mut self) {
        match self.input_focus {
            InputFocus::Username => self.input_focus = InputFocus::Password,
            InputFocus::Password => self.input_focus = InputFocus::Username,
            _ => ()
        }
    }

    fn apply_focus(&mut self) {
        match self.input_focus {
            InputFocus::Username => {
                self.username_input.enable_focus();
                self.password_input.disable_focus();
            },
            InputFocus::Password => {
                self.password_input.enable_focus();
                self.username_input.disable_focus();
            },
            InputFocus::None => {
                self.password_input.disable_focus();
                self.username_input.disable_focus();
            }
        }
    }
}


impl Component for Welcome {
     fn draw(&self, frame: &mut Frame, rect: Option<Rect>) {
         let rect = get_main_layout_rect(frame);
         let rect = apply_margins(rect, Margin {
             left: 4,
             right: 4,
             top: 2,
             bottom: 2
         });
         let mut username_rect = rect.clone();
         username_rect.y += 1;
         username_rect.height = 3;

         let mut password_rect = username_rect.clone();
         password_rect.y += username_rect.height;


         frame.render_widget(Paragraph::new(format!("Counter: {}", self.counter)).white().on_black(), rect);

         self.username_input.draw(frame, Some(username_rect));
         self.password_input.draw(frame, Some(password_rect));
       
     }

    fn handle_events(&mut self, app_state: &mut AppState) -> Result<()> {

        match app_state.key_pressed() {
            KeyCode::Char('j') => self.counter += 1 ,
            KeyCode::Char('k') => self.counter -= 1,
            _ => ()
        }

        match app_state.key_pressed() {
            KeyCode::Char(char) => self.text.push(char),
            _ => ()
        }

        self.handle_input_events(app_state);

        Ok(())
    }

}