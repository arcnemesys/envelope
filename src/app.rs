use std::env;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Houses environment variables for the current environment.
    pub env_vars: Vec<(String, String)>,
    /// Specifies which environment variable is currently being edited.
    pub selected_env_var: usize,
    /// Specifies whether or not the app is in an `editing` state.
    pub editing: bool,
    /// Houses the edited environment variable value string.
    pub edit_value: String,
    /// Holds the state of the list of environment variables
    pub env_list_state: ratatui::widgets::ListState,
    /// Holds the state of the list of path variable components
    pub path_list_state: ratatui::widgets::ListState,
    /// Boolean to determine if app is running,
    pub running: bool,
}

impl App {
    pub fn new() -> App {
        let mut env_list_state = ratatui::widgets::ListState::default();
        env_list_state.select(Some(0));
        let mut path_list_state = ratatui::widgets::ListState::default();
        path_list_state.select(Some(0));
        let env_vars = env::vars().collect();
        App {
            env_vars,
            selected_env_var: 0,
            editing: false,
            edit_value: String::new(),
            env_list_state,
            path_list_state,
            running: true,
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
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
pub enum CurrentlyEditing {
    Key,
    Value,
}
