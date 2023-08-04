use tui::{
    backend::Backend,
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Cell, Row, Table},
    layout::{Layout, Constraint, Rect},
    Frame,
};

use crate::app::App;

pub fn render_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    let tasklist = app.data.tasks_printeable();
    let header_cells = ["", "Time", "Cat", "Task"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue))
        .height(1)
        .bottom_margin(1);
    let rows = tasklist.iter().map(|item| {
        Row::new(vec![
            Cell::from(item.get_checkbox_string()),
            Cell::from(item.get_time_spent_string()),
            Cell::from(item.get_category_string()),
            Cell::from(item.get_description_string()),
        ]).style(if app.selected_task == Some(item.id) {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        })
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Min(10),
            Constraint::Percentage(100),
        ]);
    frame.render_stateful_widget(t, *rect, &mut app.tablestate);
}

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(frame.size());
    
    render_table(app, frame, &rects[0]);
}
