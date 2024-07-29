use crate::app::{ActiveList, App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::env::{remove_var, set_var};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Esc => {
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
            app.toggle_active();
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
                    app.path_var_value = app.path_var_dirs[app.selected_path_dir].clone();
                    app.path_list_state.select(Some(app.selected_path_dir))
                }
            }
        },
        KeyCode::Up => match app.activated_list {
            ActiveList::EnvList => {
                if !app.editing && app.selected_env_var > 0 {
                    app.selected_env_var -= 1;
                    app.env_list_state.select(Some(app.selected_env_var))
                }
            }
            ActiveList::PathList => {
                if app.selected_path_dir > 0 {
                    app.selected_path_dir += 1;
                    app.path_var_value = app.path_var_dirs[app.selected_path_dir].clone();
                    app.path_list_state.select(Some(app.selected_path_dir))
                }
            }
        },
        KeyCode::Enter => match app.activated_list {
            ActiveList::EnvList => {
                app.env_vars[app.selected_env_var].1 = app.env_var_value.clone();
                let key = app.env_vars[app.selected_env_var].0.clone();
                set_var(key, app.env_var_value.clone());
                app.editing = false;
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
