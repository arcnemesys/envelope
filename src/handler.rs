use crate::app::{ActiveList, App, AppResult, CurrentScreen};
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
            inactive(&mut app.active_list.clone());
            let active_index = app.list_index;
            app.list_index = (active_index + 1) % 2;
            app.toggle_active();
            active(&mut app.inactive_list.clone());
        }
        KeyCode::Down => match app.activated_list {
            ActiveList::EnvList => {
                if !app.editing && app.selected_env_var < app.env_vars.len() - 1 {
                    app.selected_env_var += 1;
                    app.env_list_state.select(Some(app.selected_env_var))
                }
            }
            ActiveList::PathList => {
                if app.selected_path_dir < app.path_var_dirs.len() - 1 {
                    app.selected_path_dir += 1;
                    app.path_list_state.select(Some(app.selected_path_dir))
                }
            }
        },
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
