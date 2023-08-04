use std::error;
use crate::model::Database;
use tui::widgets::TableState;
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App<'a> {
    pub tablestate: TableState,
    pub textarea: TextArea<'a>,
    /// Is the application running?
    pub running: bool,
    
    pub data: Database,

    pub data_changed: bool,

    pub selected_task: Option<u32>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            tablestate: TableState::default(),
            textarea: TextArea::default(),
            running: true,
            data: Database::load_or_create(),
            data_changed: false,
            selected_task: None
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if self.data_changed {
            self.data.save();
            self.data_changed = false;
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_previous_task(&mut self) {
        let tasklist = self.data.tasks_printeable();
        match self.selected_task {
            Some(selected) => {
                let tasklist_index = tasklist.iter().position(|task| task.id == selected);
                self.selected_task = match tasklist_index {
                    Some(0) => Some(tasklist[tasklist.len() - 1].id),
                    Some(index) => Some(tasklist[index - 1].id),
                    None => None,
                }
            }
            None => {
                if tasklist.len() > 0 {
                    self.selected_task = Some(tasklist[tasklist.len() - 1].id);
                }
            }
        }
    }

    pub fn select_next_task(&mut self) {
        let tasklist = self.data.tasks_printeable();
        match self.selected_task {
            Some(selected) => {
                let tasklist_index = tasklist.iter().position(|task| task.id == selected);
                self.selected_task = match tasklist_index {
                    Some(index) if index == tasklist.len() - 1 => Some(tasklist[0].id),
                    Some(index) => Some(tasklist[index + 1].id),
                    None => None,
                }
            }
            None => {
                if tasklist.len() > 0 {
                    self.selected_task = Some(tasklist[0].id);
                }
            }
        }
    }

    pub fn check_task(&mut self) {
        match self.selected_task {
            Some(selected) => {
                self.data.check_task(selected);
                self.data_changed = true;
            }
            None => {}
        }
    }

    pub fn set_category(&mut self, category: Option<u32>) {
        match self.selected_task {
            Some(selected) => {
                self.data.set_category(selected, category);
                self.data_changed = true;
            }
            None => {}
        }
    }

}
