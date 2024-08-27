use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use std::env;
use std::env::{split_paths, var_os};
use std::error;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    /// Houses environment variables for the current environment.
    pub env_vars: Vec<(String, String)>,
    /// Houses the directories stored in the path variable.
    pub path_var_dirs: Vec<PathBuf>,
    /// Specifies which environment variable is currently being edited.
    pub selected_env_var: usize,
    /// Specifies the environment variable name associated with the value.
    pub selected_env_key: String,
    /// Specifies which path variable is currently being edited.
    pub selected_path_dir: usize,
    /// Specifies whether or not the app is in an `editing` state.
    pub editing: bool,
    /// Houses the edited environment variable value string.
    pub env_var_value: String,
    /// Houses the path variable being edited.
    pub path_var_value: String,
    pub path_var_edit: String,
    /// Houses the edited environment variable key string.
    pub env_var_key: String,
    /// Holds the state of the list of environment variables
    pub env_list_state: ratatui::widgets::ListState,
    /// Holds the state of the list of path variable components
    pub path_list_state: ratatui::widgets::ListState,
    /// Boolean to determine if app is running,
    pub running: bool,
    /// Houses the state indicating what a user is currently editing.
    pub currently_editing: Option<CurrentlyEditing>,
    /// Currently activated list number.
    pub list_index: u32,
    /// Currently active list widget.
    pub activated_list: ActiveList,
    /// User shell
    pub shell: String,
    /// Environment variables from .bashrc
    shell_env_vars: HashMap<String, String>,
    /// Shell config path
    config_path: PathBuf
}

pub enum ActiveList {
    EnvList,
    PathList,
}

impl App {
    pub fn new() -> App {
        let mut env_list_state = ratatui::widgets::ListState::default();
        env_list_state.select(Some(0));
        let mut path_list_state = ratatui::widgets::ListState::default();
        path_list_state.select(Some(0));
        let env_vars = env::vars().collect();
        let mut path_var_dirs = Vec::new();
        let shell = get_shell_config().unwrap();
        let config_path = get_config_path;
        let key = "PATH";
        let path_var = var_os(key);
        

        match path_var {
            Some(paths) => {
                for path in split_paths(&paths) {
                    path_var_dirs.push(path);
                }
            }
            None => println!("{key} not set in current environment."),
        }
        let app = App {
            env_vars,
            path_var_dirs,
            selected_env_var: 0,
            selected_env_key: String::new(),
            selected_path_dir: 0,
            editing: false,
            env_var_value: String::new(),
            env_var_key: String::new(),
            env_list_state,
            path_list_state,
            running: true,
            currently_editing: None,
            list_index: 0,
            activated_list: ActiveList::EnvList,
            path_var_value: String::new(),
            path_var_edit: String::new(),
            shell,
            shell_env_vars: HashMap::new(),
            config_path,

        }
        app
    }

    pub fn selected_value(&self) -> &str {
        &self.env_vars[self.selected_env_var].1
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_active(&mut self) {
        let active_index = self.list_index;
        match self.activated_list {
            ActiveList::EnvList => {
                self.activated_list = ActiveList::PathList;
                self.list_index = (active_index + 1) % 2;
            }
            ActiveList::PathList => {
                self.activated_list = ActiveList::EnvList;
                self.list_index = (active_index + 1) % 2;
            }
        }
    }
}
 
pub enum CurrentlyEditing {
    EnvVarValue,
    EnvVarName,
    PathVar,
}

// Reference code that may be deleted soon.

fn get_shell_config() -> Result<String, Error> {
    let home = std::env::var("HOME").expect("Couldn't get user home directory");
    let mut home_path = std::path::PathBuf::from(home);
    let mut shell = String::new();
    for entry in home_path.read_dir().expect("read dir failed") {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        match file_name {
            ".bashrc" => {
                shell = String::from(file_name);
            }
            _ => {}
        }
    }

    Ok(shell)
}

fn get_config_path() -> PathBuf {
    let shell = get_shell_config();
    let home = std::env::var("HOME").expect("Couldn't get user home directory");
    let mut config_path = PathBuf::from(home).push(shell);

    config_path
    

}
pub fn get_shell_vars() -> HashMap<String, String> {
    let config_path = get_config_path();

    let config_file = File::read(true).open(config_path).unwrap()
}
