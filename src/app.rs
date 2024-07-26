use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use std::env;
use std::error;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App<'a, 'b> {
    /// Houses environment variables for the current environment.
    pub env_vars: Vec<(String, String)>,
    /// Houses the directories stored in the path variable.
    pub path_var_dirs: Vec<String>,
    /// Specifies which environment variable is currently being edited.
    pub selected_env_var: usize,
    /// Specifies which path variable is currently being edited.
    pub selected_path_dir: usize,
    /// Specifies whether or not the app is in an `editing` state.
    pub editing: bool,
    /// Houses the edited environment variable value string.
    pub env_var_value: String,
    /// Houses the edited environment variable key string.
    pub env_var_key: String,
    /// Holds the state of the list of environment variables
    pub env_list_state: ratatui::widgets::ListState,
    /// Holds the state of the list of path variable components
    pub path_list_state: ratatui::widgets::ListState,
    /// Boolean to determine if app is running,
    pub running: bool,
    /// The current screen the user is on
    pub current_screen: CurrentScreen,
    /// Houses the state indicating what a user is currently editing.
    pub currently_editing: Option<CurrentlyEditing>,
    pub list_index: u32,
    pub active_list: List<'a>,
    pub inactive_list: List<'b>,
}

pub enum ActiveList {
    EnvList,
    PathList,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new() -> App<'a, 'b> {
        let mut env_list_state = ratatui::widgets::ListState::default();
        env_list_state.select(Some(0));
        let mut path_list_state = ratatui::widgets::ListState::default();
        path_list_state.select(Some(0));
        let env_vars = env::vars().collect();
        let path_var_dirs = Vec::new();
        App {
            env_vars,
            path_var_dirs,
            selected_env_var: 0,
            selected_path_dir: 0,
            editing: false,
            env_var_value: String::new(),
            env_var_key: String::new(),
            env_list_state,
            path_list_state,
            running: true,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            list_index: 0,
            active_list: List::new(vec![ListItem::new("")]),
            inactive_list: List::new(vec![ListItem::new("")]),
        }
    }

    pub fn selected_value(&self) -> &str {
        // Select the tuple in env_vars, at the index
        // stored in self.selected, which defaults to zero.
        &self.env_vars[self.selected_env_var].1
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::PathVar => {
                    self.currently_editing = Some(CurrentlyEditing::EnvVarValue)
                }
                CurrentlyEditing::EnvVarValue => {
                    self.currently_editing = Some(CurrentlyEditing::PathVar)
                }
                _ => {}
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::EnvVarValue);
        }
    }
}

pub enum CurrentScreen {
    Main,
    Editing,
    Path,
    Exiting,
}
pub enum CurrentlyEditing {
    EnvVarValue,
    EnvVarName,
    PathVar,
}
