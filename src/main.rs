use envelope::app::{App, AppResult, get_shell_vars};
use envelope::event::{Event, EventHandler};
use envelope::handler::handle_key_events;
use envelope::tui::Tui;
use ratatui::style::Style;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> AppResult<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    tui.exit()?;
    get_shell_vars();
    Ok(())
}
