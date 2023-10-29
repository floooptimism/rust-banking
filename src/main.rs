use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result, Stdout};
use std::ops::BitAnd;
use crate::ui::app_state::AppState;
use crate::ui::events::{Event, get_key_pressed, has_event, is_key_event, is_key_pressed};
use crate::ui::traits::component::Component;
use crate::ui::view::welcome::Welcome;

mod banking;
mod ui;


fn main() -> Result<()>{
    startup()?;
    render_loop()?;

    cleanup()?;
    Ok(())
}

fn render_loop() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut main_view = Welcome::new();
    let mut app_state = AppState::new();

    loop {
        app_state.get_events();
        main_view.handle_events(&app_state);
        draw(&mut terminal, &main_view)?;

        // root event handler
        match app_state.key_pressed() {
            KeyCode::Char('q') => break, //* Exit program
            _ => ()
        }
    }

    Ok(())
}

fn draw(terminal: &mut Terminal<CrosstermBackend<Stdout>>, root_view: &impl Component) -> Result<()> {
    terminal.draw( |frame| {
        root_view.draw(frame);
    })?;
    Ok(())
}


fn startup() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(())
}

fn cleanup() -> Result<()>{
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}