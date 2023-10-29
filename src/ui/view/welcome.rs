use ratatui::Frame;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use crate::ui::container_helper::{apply_margins, get_main_layout_rect, Margin};
use crate::ui::traits::component::Component;
use crossterm::event::{self, KeyCode, KeyEventKind};
use crate::{AppState, Event};
use crate::ui::events::{get_key_pressed, has_event, is_key_event, is_key_pressed};

pub struct Welcome {
    counter: u16
}

impl Welcome {
    pub fn new() -> Welcome {
        Welcome {
            counter: 0
        }
    }
}

impl Component for Welcome {
     fn draw(&self, frame: &mut Frame) {
         let rect = get_main_layout_rect(frame);
         let rect = apply_margins(rect, Margin {
             left: 4,
             right: 4,
             top: 2,
             bottom: 2
         });
         frame.render_widget(Paragraph::new(format!("Counter: {}", self.counter)).white().on_black(), rect);
     }

    fn handle_events(&mut self, app_state: &AppState) {
        match app_state.key_pressed() {
            KeyCode::Char('j') => self.counter += 1,
            _ => ()
        }
    }
}