use ratatui::{
    backend::Backend,
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Cell, Row, Table, Wrap, Paragraph, Gauge},
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
        ])
    });


    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Todos"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Percentage(100),
        ]);
    let selected_task_index = match app.selected_task {
        Some(selected) => tasklist.iter().position(|task| task.id == selected),
        None => None,
    };
    frame.render_stateful_widget(t, *rect, &mut app.tablestate
        .clone()
        .with_selected(selected_task_index)
        .with_offset(0)
    );
}

fn render_category_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    let categorylist = app.data.categories_printeable();
    let header_cells = ["Show", "Hotkey", "Name"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue))
        .height(1)
        .bottom_margin(1);
    let rows = categorylist.iter().map(|item| {
        Row::new(vec![
            Cell::from(item.get_visible_string()),
            Cell::from(item.get_hotkey_string()),
            Cell::from(item.get_description_string()),
        ])
    }).collect::<Vec<Row>>();
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Categories"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Length(4),
            Constraint::Length(6),
            Constraint::Percentage(100),
        ]);
    let selected_category_index = match app.selected_category {
        Some(selected) => categorylist.iter().position(|category| category.id == selected),
        None => None,
    };
    frame.render_stateful_widget(t, *rect, &mut app.tablestate
        .clone()
        .with_selected(selected_category_index)
        .with_offset(0));
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
    let p = Paragraph::new("Ctrl+c to quit, Up/Down to select a task, PgUp/Down to move a task, Esc to switch to view mode, c to switch to category view, x to check task, ctrl+<hotkey> to toggle category visibility, d to set default category, n to test notifications")
        .block(Block::default().title("Help").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: false });
    frame.render_widget(p, *rect);
}

fn render_progress_gauge<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    let gauge = match app.data.get_remaining_pomodoro_time() {
        Some((duration_left, fraction_completed)) => {
            Gauge::default()
                .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
                .label(format!("{:02}:{:02} left", duration_left.num_minutes(), duration_left.num_seconds() % 60))
                .ratio(fraction_completed)
        },
        None => {
            Gauge::default()
                .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
                .label("No pomodoro active, start one with 'p'")
                .percent(0)
        }
    };
    
    frame.render_widget(gauge, *rect);
}


fn render_todo_view<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(5),
            Constraint::Length(3),
            ].as_ref())
        .margin(1)
        .split(frame.size());
    
    render_progress_gauge(app, frame, &rects[0]);
    render_todo_table(app, frame, &rects[1]);
    render_input(app, frame, &rects[2])
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
