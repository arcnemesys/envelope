use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::borrow::BorrowMut;
use std::env::{split_paths, var_os};
use std::ops::DerefMut;

use crate::app::App;

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

    let key = "PATH";
    let mut path_items: Vec<ListItem> = Vec::new();
    let mut path_vars = Vec::new();
    let path_var = var_os(key);

    match path_var {
        Some(paths) => {
            for path in split_paths(&paths) {
                path_items.push(ListItem::new(format!("{:?}", path.clone())));
                path_vars.push(path);
            }
        }
        None => println!("{key} not set in current environment."),
    }
    app.path_var_dirs = path_vars;

    let mut _path_list = List::new(path_items);

    let path_list = _path_list
        .clone()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Path").alignment(Alignment::Center)),
        )
        .highlight_symbol(">>")
        .highlight_style(ratatui::style::Style::default().fg(Color::LightGreen));

    let env_items: Vec<ListItem> = app
        .env_vars
        .iter()
        .map(|(key, value)| ListItem::new(format!("{}: {}", key, value)))
        .collect();

    let _env_list = List::new(env_items);

    let env_list = _env_list
        .clone()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Environment Variables").alignment(Alignment::Center)),
        )
        .highlight_symbol(">>")
        .highlight_style(ratatui::style::Style::default().fg(Color::Yellow));

    let mut lists = vec![env_list.clone(), path_list.clone()];
    app.active_list = env_list.clone();
    app.inactive_list = path_list.clone();
    active(&mut lists[0].clone());
    inactive(&mut lists[1].clone());
    f.render_stateful_widget(env_list.clone(), chunks[0], &mut app.env_list_state);
    f.render_stateful_widget(&path_list.clone(), sub_chunks[1], &mut app.path_list_state);

    let edit_paragraph = if app.editing {
        Paragraph::new(app.env_var_value.clone())
            .block(Block::default().borders(Borders::ALL).title("Edit Value"))
    } else {
        Paragraph::new(app.selected_value())
            .block(Block::default().borders(Borders::ALL).title("Value"))
    };

    f.render_widget(edit_paragraph, chunks[1]);
}

pub fn active(list: &mut List) {
    let active_block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().blue())
        .title(Title::from("Active").alignment(Alignment::Right));

    let mut active_list = list
        .clone()
        .block(active_block)
        .highlight_symbol(">>")
        .highlight_style(Style::default().fg(Color::Magenta));

    let list = &mut active_list;
}

pub fn inactive(list: &mut List) {
    let inactive_block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Color::DarkGray))
        .title(Title::from("Inactive").alignment(Alignment::Right));
    let mut inactive_list = list.clone().block(inactive_block);

    let list = &mut inactive_list;
}
