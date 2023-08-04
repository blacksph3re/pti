use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{CursorMove};

fn handle_todo_view_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.select_no_task();
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            app.check_task();
        }
        KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('h') | KeyCode::Char('H') => {
            app.select_first_category();
        }
        KeyCode::Up => {
            app.select_previous_task();
        }
        KeyCode::Down => {
            app.select_next_task();
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.toggle_pomodoro();
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.notification_manager.notify("Test notification", "This is a test notification");
        }
        // Check for category hotkeys
        KeyCode::Char(character) => {
            match app.data.get_category_by_hotkey(character) {
                Some(category) => {
                    // Without ctrl, assigns the current task to the category. With crtl, toggles the category visibility
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.toggle_category_visible(category.id);
                    } else {
                        app.set_category(category.id);
                    }
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

fn handle_category_view_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => {
            app.select_no_category();
            app.select_no_task();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.select_no_category();
        }
        KeyCode::Up => {
            app.select_previous_category();
        }
        KeyCode::Down => {
            app.select_next_category();
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            app.make_default_category(app.selected_category.expect("Category handler called without a selected category"));
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            app.toggle_category_visible(app.selected_category.expect("Category handler called without a selected category"));
        }
        // Check for category hotkeys
        KeyCode::Char(character) if key_event.modifiers == KeyModifiers::CONTROL => {
            match app.data.get_category_by_hotkey(character) {
                Some(category) => {
                    app.toggle_category_visible(category.id);
                }
                None => {}
            }
        }
        _ => {}
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
            if app.selected_category == None {
                if app.selected_task == None {
                    handle_text_view_events(key_event, app)
                } else {
                    handle_todo_view_events(key_event, app)
                }
            } else {
                handle_category_view_events(key_event, app)
            }
        }
    }
}
