use std::{default, io::{stdout, Result}};
use std::any::Any;

use crossterm::{event::KeyCode, cursor::{MoveTo, Show, EnableBlinking}, ExecutableCommand};
use ratatui::{prelude::Rect, widgets::{Block, Paragraph, Borders}, style::Stylize, Frame};
use ratatui::style::Style;

use crate::ui::{traits::component::Component, app_state::AppState};

pub struct Input {
    is_taking_input: bool,
    value: Vec<char>,
    label: String,
    length: u16,
    is_focused: bool,
    formatter: Option<char>
}


impl Input {
    pub fn new(label:&str) -> Input {
        Input {
            is_taking_input: false,
            value: Vec::new(),
            label: label.to_string(),
            length: 0,
            is_focused: false,
            formatter: None
        }
    }

    pub fn draw_cursor(&self, text_rect: Rect) -> Result<()> {
        if !self.is_focused { return Ok(())}
        let width = text_rect.x + self.length;
        let _ = stdout()
        .execute(MoveTo(width, text_rect.y))?;
        Ok(())
    }

    fn make_block(&self) -> Block {
        let mut block = Block::default().borders(Borders::ALL).title(self.label.clone());
        if self.is_focused && self.is_taking_input {
            block = block.border_style(Style::new().yellow());
        }
        block
    }

    fn make_string_to_render(&self) -> String {
        let mut string_to_render: String;
        if let Some(format) = self.formatter {
            string_to_render = format.to_string().repeat(self.value.len());
        }else {
            string_to_render = self.value.clone().into_iter().collect::<String>();
        }
        string_to_render
    }

    pub fn enable_focus(&mut self) {
        self.is_focused = true;
    }

    pub fn disable_focus(&mut self) {
        self.is_focused = false;
    }

    pub fn set_formatter(&mut self, formatter: Option<char>) {
        self.formatter = formatter;
    }
}


impl Component for Input {
    fn draw(&self, frame: &mut Frame, rect: Option<Rect>) {
        let rect = rect.unwrap();

        let mut inside_block = rect.clone();
        inside_block.y += 1;
        inside_block.x += 1;
        inside_block.height = 1;

        let block = self.make_block();
        let string_to_render = self.make_string_to_render();

        frame.render_widget(block, rect);
        frame.render_widget(Paragraph::new(string_to_render).white().on_black(),inside_block);

        let _ = self.draw_cursor(inside_block);
    }

    fn handle_events(&mut self, app_state: &mut AppState) -> std::io::Result<()> {

        match app_state.key_pressed() {
            KeyCode::Enter => {
                self.is_taking_input = true;
                app_state.set_input_mode_to_input();
            },
            KeyCode::Esc => {
                self.is_taking_input = false;
                app_state.set_input_mode_to_normal();
            },
            _ => ()
        }

        if self.is_taking_input && self.is_focused {
            match app_state.key_pressed() {
                KeyCode::Char(c) => {
                    self.value.push(c);
                    self.length = self.value.len() as u16;
                },
                KeyCode::Backspace => {
                    self.value.pop();
                    self.length = self.value.len() as u16;
                    ()
                }
                _ => ()
            }
        }

        Ok(())
    }
}