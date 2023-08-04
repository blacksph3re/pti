use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else {
                app.check_task();
            }
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
        // Other handlers you could add here.
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
