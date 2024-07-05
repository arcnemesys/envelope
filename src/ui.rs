use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, List, ListItem},
    Frame,
};
use ratatui::prelude::*;

use std::env;
use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let top_level_sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.size());

    let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "Envelope", 
        Style::default().fg(Color::Green).bg(Color::Black),))
        .block(title_block).centered();

        let sub_sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(top_level_sections[1]);

    frame.render_widget(title, top_level_sections[0]);

    let mut env_var_list_items = Vec::<ListItem>::new();
    let mut env_val_list_items = Vec::<ListItem>::new();

    let mut env_vars = env::vars();


    
    for (key, value) in env_vars {
        env_var_list_items.push(
            ListItem::new(Line::from(Span::styled(
                format!("Environment Variable: {}", key),
                Style::default().fg(Color::Green)))));
                
        env_val_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("Variable Value: {}", value),
            Style::default().fg(Color::Green)))));
 
    }

    let env_var_list = List::new(env_var_list_items);
    let env_val_list = List::new(env_val_list_items);

    frame.render_widget(env_var_list, sub_sections[0]);
    frame.render_widget(env_val_list, sub_sections[1]);

    
    let controls_block = Paragraph::new(
        format!("Press `Esc`, `Ctrl-C` or `q` to stop running",))
        .block(Block::bordered()
        .title("Controls")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded))
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));

    frame.render_widget(controls_block, top_level_sections[2]);
        
}
