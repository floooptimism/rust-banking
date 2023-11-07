use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyCode;
use crossterm::ExecutableCommand;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::ui::app_state::{AppState, Event};
use crate::ui::traits::component::Component;
use crate::ui::traits::focusable::Focusable;

pub struct Button {
    id: String,
    text: String,
    is_focused: bool
}

impl Button {
    pub fn new(id: &str, text: &str) -> Button {
        Button {
            id: id.to_string(),
            text: text.to_string(),
            is_focused: false
        }
    }
}

impl Focusable for Button {
    fn enable_focus(&mut self) {
        self.is_focused = true
    }

    fn disable_focus(&mut self) {
        self.is_focused = false;
    }
}

impl Component for Button {
    fn draw(&self, frame: &mut Frame, rect: Option<Rect>) {
        let rect = rect.unwrap();
        // let total_length = self.text.len() + 2;
        // if rect.width < total_length as u16 {
        //     panic!("Width in rect given is too short for button text: {}", self.text);
        // }

        let mut block = Block::default().borders(Borders::ALL);

        if self.is_focused {
            block = block.border_style(Style::new().yellow());
        }

        let text = Paragraph::new(self.text.clone()).block(block).alignment(Alignment::Center).wrap(Wrap {trim:true});

        frame.render_widget(text, rect);
    }

    fn handle_events(&mut self, app_state: &mut AppState) -> std::io::Result<()> {
        if self.is_focused {
            app_state.set_cursor_mode_to_hidden()
        }
        //* Press enter key
        if let KeyCode::Enter = app_state.key_pressed() {
            if self.is_focused {
                app_state.push_event(
                    Event::ButtonClicked(self.id.clone())
                )
            }
        }

        Ok(())
    }
}