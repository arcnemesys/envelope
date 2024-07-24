use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use envelope::app::App;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::env;
use std::env::{remove_var, set_var, split_paths, var_os};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            // We have to alter the keycodes to differentiate between which list is being edited
            // in order to stop them from scrolling in sync
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    if !app.editing && app.selected_env_var < app.env_vars.len() - 1 {
                        app.selected_env_var += 1;
                        app.env_list_state.select(Some(app.selected_env_var));
                    }
                }
                KeyCode::Up => {
                    if !app.editing && app.selected_env_var > 0 {
                        app.selected_env_var -= 1;
                        app.env_list_state.select(Some(app.selected_env_var));
                    }
                }
                KeyCode::Char('e') => {
                    if !app.editing {
                        app.editing = true;
                        app.edit_value = app.selected_value().to_string();
                    }
                }
                KeyCode::Esc => {
                    if app.editing {
                        app.editing = false;
                    }
                }
                KeyCode::Enter => {
                    if app.editing {
                        app.env_vars[app.selected_env_var].1 = app.edit_value.clone();
                        app.editing = false;
                    }
                }
                KeyCode::Char(c) => {
                    if app.editing {
                        app.edit_value.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if app.editing {
                        app.edit_value.pop();
                    }
                }
                _ => {}
            }
        }
    }
}
