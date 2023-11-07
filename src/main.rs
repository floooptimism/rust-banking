use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, event::KeyCode, cursor::Show,
};
use ratatui::
    prelude::{CrosstermBackend, Terminal};
    
use std::io::{stdout, Result, Stdout};
use crossterm::cursor::Hide;
use crate::ui::app_state::{AppState, CursorMode, InputMode};
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
    
    
    'main_loop: loop {
        app_state.handle_events();
        main_view.handle_events(&mut app_state)?;
        draw(&mut terminal, &main_view)?;
        
        app_state.handle_cursor_modes();

        // * root event handler
        // * If we're not accepting inputs
        if let InputMode::Normal = app_state.input_mode() {
            match app_state.key_pressed() {
                KeyCode::Char('q') => break 'main_loop, //* Exit program
                _ => ()
            }
        }


    }

    Ok(())
}

fn draw(terminal: &mut Terminal<CrosstermBackend<Stdout>>, root_view: &impl Component) -> Result<()> {
    terminal.draw( |frame| {
        root_view.draw(frame, None);
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