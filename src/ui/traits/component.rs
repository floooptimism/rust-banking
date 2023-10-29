use ratatui::Frame;
use crate::{AppState, Event};

pub trait Component {
    fn draw(&self, frame: &mut Frame);

    fn handle_events(&mut self, app_state: &AppState);
}