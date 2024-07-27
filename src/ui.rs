use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
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

    let path_items: Vec<ListItem> = app
        .path_var_dirs
        .iter()
        .map(|path| ListItem::new(format!("{:?}", path)))
        .collect();
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

    f.render_stateful_widget(env_list, sub_chunks[0], &mut app.env_list_state);
    f.render_stateful_widget(path_list, sub_chunks[1], &mut app.path_list_state);

    let edit_paragraph = if app.editing {
        Paragraph::new(app.env_var_value.clone())
            .block(Block::default().borders(Borders::ALL).title("Edit Value"))
    } else {
        Paragraph::new(app.selected_value())
            .block(Block::default().borders(Borders::ALL).title("Value"))
    };

    f.render_widget(edit_paragraph, chunks[1]);
}
