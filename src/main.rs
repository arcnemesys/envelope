use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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

struct App {
    env_vars: Vec<(String, String)>,
    selected_env_var: usize,
    editing: bool,
    edit_value: String,
    env_list_state: ratatui::widgets::ListState,
    path_list_state: ratatui::widgets::ListState,
}

impl App {
    fn new() -> App {
        let mut env_list_state = ratatui::widgets::ListState::default();
        env_list_state.select(Some(0));
        let mut path_list_state = ratatui::widgets::ListState::default();

        let env_vars = env::vars().collect();
        App {
            env_vars,
            selected_env_var: 0,
            editing: false,
            edit_value: String::new(),
            env_list_state,
            path_list_state,
        }
    }

    fn selected_value(&self) -> &str {
        // Select the tuple in env_vars, at the index
        // stored in self.selected, which defaults to zero.
        &self.env_vars[self.selected_env_var].1
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .borders(Borders::ALL)
                .title("Environment Variables");
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(size);

            let sub_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            let empty_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let key = "PATH";
            let mut path_items: Vec<ListItem> = Vec::new();

            let path_var = var_os(key);

            match path_var {
                Some(paths) => {
                    for path in split_paths(&paths) {
                        path_items.push(ListItem::new(format!("{:?}", path)));
                    }
                }
                None => println!("{key} not set in current environment."),
            }

            let path_list = List::new(path_items)
                .block(Block::default().borders(Borders::ALL).title("Path"))
                .highlight_symbol(">>")
                .highlight_style(
                    ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                );

            f.render_stateful_widget(path_list, sub_chunks[1], &mut app.path_list_state);

            let items: Vec<ListItem> = app
                .env_vars
                .iter()
                .map(|(key, value)| ListItem::new(format!("{}: {}", key, value)))
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Variables"))
                .highlight_symbol(">>")
                .highlight_style(
                    ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                );

            f.render_stateful_widget(list, chunks[0], &mut app.env_list_state);

            let edit_paragraph = if app.editing {
                Paragraph::new(app.edit_value.clone())
                    .block(Block::default().borders(Borders::ALL).title("Edit Value"))
            } else {
                Paragraph::new(app.selected_value())
                    .block(Block::default().borders(Borders::ALL).title("Value"))
            };

            f.render_widget(edit_paragraph, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
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
