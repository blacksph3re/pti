use ratatui::{
    backend::Backend,
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Cell, Row, Table, Wrap, Paragraph},
    layout::{Layout, Constraint, Rect, Direction},
    Frame,
};

use crate::app::App;

fn render_todo_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
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
        .block(Block::default().borders(Borders::ALL).title("Todos"))
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

fn render_category_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    let categorylist = app.data.categories_printeable();
    let header_cells = ["Hotkey", "Name"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue))
        .height(1)
        .bottom_margin(1);
    let mut rows = categorylist.iter().map(|item| {
        Row::new(vec![
            Cell::from(item.get_hotkey_string()),
            Cell::from(item.get_description_string()),
        ]).style(if app.selected_category == Some(item.id) {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        })
    }).collect::<Vec<Row>>();
    rows.insert(0, Row::new(vec![
        Cell::from("(u)"),
        Cell::from("No Category"),
    ]));
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Categories"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Length(6),
            Constraint::Percentage(100),
        ]);
    frame.render_stateful_widget(t, *rect, &mut app.tablestate);
}

fn render_input<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    app.textarea.set_style(Style::default());
    app.textarea.set_cursor_line_style(Style::default());
    app.textarea.set_cursor_style(match app.selected_task {
        Some(_) => Style::default(),
        None => Style::default().add_modifier(Modifier::REVERSED),
    });
    app.textarea.set_block(Block::default().borders(Borders::ALL).title("New Task"));
    frame.render_widget(app.textarea.widget(), *rect);
}

fn render_help_text<B: Backend>(frame: &mut Frame<'_, B>, rect: &Rect) {
    let p = Paragraph::new("Ctrl+c to quit, Up/Down to select a task, Esc to switch to view mode, c to switch to category view, x to check task")
        .block(Block::default().title("Help").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: false });
    frame.render_widget(p, *rect);
}


fn render_todo_view<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(3),
            ].as_ref())
        .margin(1)
        .split(frame.size());
    
    render_todo_table(app, frame, &rects[0]);
    render_input(app, frame, &rects[1])
}

fn render_category_view<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(6),
            ].as_ref())
        .margin(1)
        .split(frame.size());
    
    render_category_table(app, frame, &rects[0]);
    render_help_text(frame, &rects[1]);
}

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    match app.selected_category {
        Some(_) => render_category_view(app, frame),
        None => render_todo_view(app, frame),
    }
}
