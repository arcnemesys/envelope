use crate::app::{ActiveList, App, AppResult};
use globalenv::set_var;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{env::join_paths, path::PathBuf};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let config_path = app.config_path.clone();
    let mut shell_config = OpenOptions::new().append(true).open(&config_path).unwrap();
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('e') => {
            if !app.editing {
                app.editing = true;
                app.env_var_value = app.selected_value().to_string();
            }
        }
        KeyCode::Char(c) => {
            if app.editing {
                match app.activated_list {
                    ActiveList::EnvList => app.env_var_value.push(c),
                    ActiveList::PathList => {
                        app.path_var_value.push(c);
                        let path_edit = app.path_var_value.clone();
                        app.path_var_edit = path_edit;
                        app.path_var_edit.push(c);
                    }
                }
            }
        }
        KeyCode::Backspace => {
            if app.editing {
                match app.activated_list {
                    ActiveList::EnvList => {
                        app.env_var_value.pop();
                    }
                    ActiveList::PathList => {
                        app.path_var_edit.pop();
                    }
                }
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
                    app.path_var_value = app.path_var_dirs[app.selected_path_dir]
                        .clone()
                        .to_str()
                        .unwrap()
                        .to_owned();
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
                if !app.editing && app.selected_path_dir > 0 {
                    app.selected_path_dir -= 1;
                    app.path_var_value = app.path_var_dirs[app.selected_path_dir]
                        .clone()
                        .to_str()
                        .unwrap()
                        .to_owned();
                    app.path_list_state.select(Some(app.selected_path_dir))
                }
            }
        },
        KeyCode::Enter => match app.activated_list {
            ActiveList::EnvList => {
                // Now that we have env vars stored in app state,
                // we can check for values that would be overwritten
                // or duplicated, display a pop up to the user,
                // and make decisions based on the interaction
                app.env_vars[app.selected_env_var].1 = app.env_var_value.clone();
                let key = app.env_vars[app.selected_env_var].0.clone();
                let env_var_key = key[..].to_ascii_uppercase().trim_matches('\"').to_owned();
                if app.shell_env_vars.contains_key(&env_var_key) {
                    app.overwrite = true;
                }
                let env_var = format!(
                    "export {}=\"{}\"\n",
                    env_var_key,
                    &app.env_var_value.clone()[..]
                );
                write_to_config(&app, &env_var, &mut shell_config);
                app.editing = !app.editing;
            }
            ActiveList::PathList => {
                app.path_var_dirs[app.selected_path_dir] = PathBuf::from(app.path_var_edit.clone());
                let path_var = app.path_var_edit.clone();
                let export_var = format!("export PATH=$PATH:{}\n", path_var,);

                write_to_config(&app, &export_var, &mut shell_config);
                app.editing = !app.editing;
            }
        },
        _ => {}
    }
    Ok(())
}

pub fn write_to_config(app: &App, config_var: &str, config_file: &mut File) {
    if app.activated_list == ActiveList::PathList {
        config_file
            .write_all(b"\n")
            .expect("Unable to write new line to config file");
        config_file
            .write_all(config_var.as_bytes())
            .expect(&format!("Unable to write {:?} to config file", config_var));
    } else if app.activated_list == ActiveList::EnvList {
        config_file
            .write_all(b"\n")
            .expect("Unable to write new line to config file");
        config.file.write_all(config_var.as_bytes())
            ..expect(&format!("Unable to write {:?} to config file", config_var));
    }
}
