use ratatui::prelude::*;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::env::{split_paths, var_os};

use crate::app::App;
use std::env;

/// Renders the user interface widgets.
pub fn render(app: &mut App, f: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

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
        .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

    f.render_stateful_widget(path_list, sub_chunks[1], &mut app.path_list_state);

    let items: Vec<ListItem> = app
        .env_vars
        .iter()
        .map(|(key, value)| ListItem::new(format!("{}: {}", key, value)))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Variables"))
        .highlight_symbol(">>")
        .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

    f.render_stateful_widget(list, chunks[0], &mut app.env_list_state);

    let edit_paragraph = if app.editing {
        Paragraph::new(app.edit_value.clone())
            .block(Block::default().borders(Borders::ALL).title("Edit Value"))
    } else {
        Paragraph::new(app.selected_value())
            .block(Block::default().borders(Borders::ALL).title("Value"))
    };

    f.render_widget(edit_paragraph, chunks[1]);
}
