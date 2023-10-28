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
use crate::ui::container_helper::get_main_layout_rect;

mod banking;
mod ui;


enum Event {
    Exit,
    None
}

fn main() -> Result<()>{
    startup()?;

    render_loop()?;

    cleanup()?;
    Ok(())
}

fn render_loop() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        draw(&mut terminal)?;
        let returned_event = handle_events()?;

        match returned_event {
            Event::Exit => break, // * Exit render loop quits the program
            Event::None => ()
        }
    }

    Ok(())
}

fn draw(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    terminal.draw( |frame| {
        let main_layout = get_main_layout_rect(frame);

        frame.render_widget(Paragraph::new("Hello world").white().on_black(), main_layout);

    })?;
    Ok(())
}

fn handle_events() -> Result<Event> {
    let mut return_event = Event::None;
    if event::poll(std::time::Duration::from_millis(20))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return_event = Event::Exit;
            }
        }
    }
    Ok(return_event)
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