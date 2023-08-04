use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{CursorMove};

fn handle_todo_view_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc => {
            app.select_no_task();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.check_task();
        }
        // Uncheck category
        KeyCode::Char('u') | KeyCode::Char('U') => {
            app.set_category(None);
        }
        // Counter handlers
        KeyCode::Up => {
            app.select_previous_task();
        }
        KeyCode::Down => {
            app.select_next_task();
        }
        // Check for category hotkeys
        KeyCode::Char(character) => {
            match app.data.get_category_by_hotkey(character) {
                Some(category) => {
                    app.set_category(Some(category.id));
                }
                None => {}
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_text_view_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // On arrow keys, select a task
        KeyCode::Up => {
            app.select_previous_task();
        }
        KeyCode::Down => {
            app.select_next_task();
        }
        // On enter, add a task
        KeyCode::Enter => {
            app.add_task(app.textarea.lines()[0].to_string());
            app.textarea.move_cursor(CursorMove::Jump(0, 0));
            app.textarea.delete_line_by_end();
        }
        _ => {
            app.textarea.input(key_event);
        }
    }
    Ok(())
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
            Ok(())
        }
        _ => {
            if app.selected_task == None {
                handle_text_view_events(key_event, app)
            } else {
                handle_todo_view_events(key_event, app)
            }
        }
    }
}
