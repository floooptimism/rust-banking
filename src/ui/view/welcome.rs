use std::any::Any;
use std::borrow::BorrowMut;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::{self, Result, stdout};
use std::ops::Deref;

use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::ui::container_helper::{apply_margins, get_main_layout_rect, Margin};
use crate::ui::traits::component::{Component};
use crate::ui::widgets::input::{Input};
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::text::Line;
use crate::{AppState};
use crate::ui::app_state::Event::ButtonClicked;
use crate::ui::traits::focusable::Focusable;
use crate::ui::widgets::button::Button;

#[derive(Copy, Clone)]
enum InputFocus {
    Username,
    Password,
    LoginButton,
    None
}

pub struct Welcome {
    counter: i64,
    text: Vec<char>,
    username_input: Input,
    password_input: Input,
    login_button: Button,
    input_focus_index: u8,
    focuses: [InputFocus; 3],


    //* States
    has_entered_credentials: bool
}

impl Welcome {
    pub fn new() -> Welcome {
        let input = Input::new("Username");
        let mut pass_input = Input::new("Password");
        pass_input.set_formatter(Some('*'));

        let button = Button::new("login_button", "Login");

        let mut instance = Welcome {
            counter: 0,
            text: Vec::new(),
            username_input: input,
            password_input: pass_input,
            input_focus_index: 0,
            focuses: [InputFocus::Username, InputFocus::Password, InputFocus::LoginButton],
            login_button: button,
            has_entered_credentials: false
        };

        instance.apply_focus();
        instance
    }

    pub fn handle_input_events(&mut self, app_state: &mut AppState) {
        match app_state.key_pressed() {
            KeyCode::Up | KeyCode::Down => {
                self.username_input.disable_focus();
                self.password_input.disable_focus();
                self.login_button.disable_focus();
            }
            _ => ()
        }

        match app_state.key_pressed() {
            KeyCode::Up => {
                self.input_focus_index = max((self.input_focus_index as i8) - 1, 0) as u8;
            }
            KeyCode::Down => {
                self.input_focus_index = min(self.input_focus_index + 1, (self.focuses.len() - 1) as u8);
            }
            _ => ()
        }

        self.apply_focus();


        let _ = self.username_input.handle_events(app_state);
        let _ = self.password_input.handle_events(app_state);
        let _ = self.login_button.handle_events(app_state);

    }

    fn apply_focus(&mut self) {
        match self.focuses.get(self.input_focus_index as usize).unwrap() {
            InputFocus::Username => self.username_input.enable_focus(),
            InputFocus::Password => self.password_input.enable_focus(),
            InputFocus::LoginButton => self.login_button.enable_focus(),
            _ => ()
        }
    }


    fn draw_banner(&self, frame: &mut Frame, mut rect: &mut Rect) {
        rect.height = 6;

        let lines = vec![
            Line::from(" ____   _____  _____ "),
            Line::from("|  _ \\ |  __ \\|_   _|"),
            Line::from("| |_) || |__) | | |  "),
            Line::from("|  _ < |  ___/  | |  "),
            Line::from("| |_) || |     _| |_ "),
            Line::from("|____/ |_|    |_____|")
        ];

        frame.render_widget(Paragraph::new(lines).white(), rect.clone());
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

         frame.render_widget(Block::default().on_black(), rect.clone());

         let mut banner_rect = rect.clone();

         self.draw_banner(frame, &mut banner_rect);

         let mut username_rect = banner_rect.clone();
         username_rect.y += banner_rect.height + 2;
         username_rect.height = 3;

         let mut password_rect = username_rect.clone();
         password_rect.y += username_rect.height;

         let mut login_button = password_rect.clone();
         login_button.y += password_rect.height;
         login_button.width = ("login".len() + 2) as u16;

         self.username_input.draw(frame, Some(username_rect));
         self.password_input.draw(frame, Some(password_rect));
         self.login_button.draw(frame, Some(login_button));


     }

    fn handle_events(&mut self, app_state: &mut AppState) -> Result<()> {

        if app_state.has_event(ButtonClicked("login_button".to_string())) {
            self.has_entered_credentials = true;
        }

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