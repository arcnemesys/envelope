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
        .title("Environment Variables")
        .bg(Color::Rgb(51, 0, 25));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
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
        .highlight_style(
            ratatui::style::Style::default()
                .fg(Color::Rgb(185, 185, 220))
                .bg(Color::Rgb(28, 13, 41)),
        );

    let env_items: Vec<ListItem> = app
        .env_vars
        .iter()
        .map(|(key, value)| ListItem::new(format!("{}: {}\n", key, value)))
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
        .highlight_style(
            ratatui::style::Style::default()
                .fg(Color::Rgb(185, 185, 220))
                .bg(Color::Rgb(28, 13, 41)),
        );

    f.render_stateful_widget(env_list, sub_chunks[0], &mut app.env_list_state);
    f.render_stateful_widget(path_list, sub_chunks[1], &mut app.path_list_state);
    let overwrite_warning = "This environment variable value will be overwritten";

    let edit_paragraph = if app.editing {
        if app.overwrite {
            Paragraph::new(app.env_var_value.clone())
                .block(Block::default().borders(Borders::ALL).title(format!("Warning: {}. Edit Value", overwrite_warning)))
        } else {
        Paragraph::new(app.env_var_value.clone())
            .block(Block::default().borders(Borders::ALL).title("Edit Value"))}
    } else {
        Paragraph::new(app.selected_value())
            .block(Block::default().borders(Borders::ALL).title("Value"))
    };

    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    let editor_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(footer_chunks[0]);

    let edit_path = Paragraph::new(app.path_var_value.clone())
        .block(Block::default().borders(Borders::ALL).title("Edit Value"));
    let control_footer =
        Paragraph::new("switch: ↹ (tab), exit: q/esc, edit: e, save: enter, navigate: ⇵")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Title::from("Controls").alignment(Alignment::Center)),
            )
            .alignment(Alignment::Center);

    f.render_widget(edit_paragraph, editor_chunks[0]);
    f.render_widget(edit_path, editor_chunks[1]);
    f.render_widget(control_footer, footer_chunks[1]);
}
