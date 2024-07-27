use envelope::app::{App, AppResult};
use envelope::event::{Event, EventHandler};
use envelope::handler::handle_key_events;
use envelope::tui::Tui;
use ratatui::style::Style;
use ratatui::widgets::ListItem;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> AppResult<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(256);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}

// fn run_app<B: ratatui::backend::Backend>(
//     terminal: &mut Terminal<B>,
//     mut app: App,
// ) -> io::Result<()> {
//     loop {
//         if let Event::Key(key) = event::read()? {
//             // We have to alter the keycodes to differentiate between which list is being edited
//             // in order to stop them from scrolling in sync
//             match key.code {

//                 KeyCode::Up => {
//                     if !app.editing && app.selected_env_var > 0 {
//                         app.selected_env_var -= 1;
//                         app.env_list_state.select(Some(app.selected_env_var));
//                     }
//                 }
//
//                 KeyCode::Enter => {
//                     if app.editing {
//                         app.env_vars[app.selected_env_var].1 = app.edit_value.clone();
//                         app.editing = false;
//                     }
//                 }
//
//                 _ => {}
//             }
//         }
//     }
// }
