use std::any::Any;
use std::io::Result;

use ratatui::{Frame, prelude::Rect};
use crate::{AppState};

pub trait Component {
    fn draw(&self, frame: &mut Frame, rect: Option<Rect>);

    fn handle_events(&mut self, app_state: &mut AppState) -> Result<()>;
}