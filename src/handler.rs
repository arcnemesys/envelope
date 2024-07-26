use crate::app::{App, AppResult, CurrentScreen};
use crate::ui::{active, inactive};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Main;
            app.currently_editing = None;
            app.editing = false;
        }
        KeyCode::Char('e') => {
            if !app.editing {
                app.editing = true;
                app.env_var_value = app.selected_value().to_string();
            }
        }
        KeyCode::Char(c) => {
            if app.editing {
                app.env_var_value.push(c)
            }
        }
        KeyCode::Backspace => {
            if app.editing {
                app.env_var_value.pop();
            }
        }
        KeyCode::Tab => {
            inactive(app.active_list.clone());
            active(app.inactive_list.clone());
        }
        KeyCode::Down => {}
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
