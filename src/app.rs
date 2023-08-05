use std::error;
use std::fs::{File, OpenOptions};
use fs2::FileExt;
use crate::model::Database;
use crate::notification::NotificationManager;
use crate::constants::{TASK_FILE, get_full_path};
use ratatui::widgets::TableState;
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App<'a> {
    pub database_file: File,

    pub tablestate: TableState,
    pub textarea: TextArea<'a>,
    /// Is the application running?
    pub running: bool,
    
    pub data: Database,

    pub data_changed: bool,

    pub selected_task: Option<u32>,
    pub selected_category: Option<u32>,

    pub notification_manager: NotificationManager,
}

fn open_file() -> File {
    let path = get_full_path(TASK_FILE);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.as_path())
        .expect(format!("Failed to open task file {}.", path.display()).as_str());
    
    file.lock_exclusive()
        .expect("Failed to lock task file.");

    file
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let db_file = open_file();
        let db = Database::load_or_create();
        Self {
            database_file: db_file,
            tablestate: TableState::default(),
            textarea: TextArea::default(),
            running: true,
            data: db,
            data_changed: false,
            selected_task: None,
            selected_category: None,
            notification_manager: NotificationManager::new(),
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
        // If some data has changed, save it.
        if self.data_changed {
            self.data.save();
            self.data_changed = false;
        }

        // Check if a pomodoro is over
        match self.data.check_active_pomodoro_over() {
            Some(task_descriptions) => {
                self.data.save();
                let body = format!("{} Tasks are over:\n{}", task_descriptions.len(), task_descriptions.join("\n"));
                self.notification_manager.notify("Pomodoro over!", &body);
            }
            None => {}
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
                    Some(0) => Some(tasklist[0].id),
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
                    Some(index) if index == tasklist.len() - 1 => Some(tasklist[index].id),
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

    pub fn select_previous_category(&mut self) {
        let categorylist = self.data.categories_printeable();
        match self.selected_category {
            Some(selected) => {
                let categorylist_index = categorylist.iter().position(|category| category.id == selected);
                self.selected_category = match categorylist_index {
                    Some(0) => Some(categorylist[0].id),
                    Some(index) => Some(categorylist[index - 1].id),
                    None => None,
                }
            }
            None => {
                if categorylist.len() > 0 {
                    self.selected_category = Some(categorylist[categorylist.len() - 1].id);
                }
            }
        }
    }

    pub fn select_next_category(&mut self) {
        let categorylist = self.data.categories_printeable();
        match self.selected_category {
            Some(selected) => {
                let categorylist_index = categorylist.iter().position(|category| category.id == selected);
                self.selected_category = match categorylist_index {
                    Some(index) if index == categorylist.len() - 1 => Some(categorylist[index].id),
                    Some(index) => Some(categorylist[index + 1].id),
                    None => None,
                }
            }
            None => {
                if categorylist.len() > 0 {
                    self.selected_category = Some(categorylist[0].id);
                }
            }
        }
    }

    pub fn select_first_category(&mut self) {
        let categorylist = self.data.categories_printeable();
        if categorylist.len() > 0 {
            self.selected_category = Some(categorylist[0].id);
        }
    }

    pub fn select_no_category(&mut self) {
        self.selected_category = None;
    }

    pub fn make_default_category(&mut self, category: u32) {
        self.data.make_default_category(category);
        self.data_changed = true;
    }

    pub fn toggle_category_visible(&mut self, category: u32) {
        self.data.toggle_category_visible(category);
        // Our selected task might have gone invisible, so we need to check that.
        let new_tasklist = self.data.tasks_printeable();
        if new_tasklist.iter().find(|task| Some(task.id) == self.selected_task).is_none() {
            self.selected_task = None;
        }
        self.data_changed = true;
    }

    pub fn select_no_task(&mut self) {
        self.selected_task = None;
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

    pub fn add_task(&mut self, task: String) {
        self.data.add_task_from_string(task);
        self.data_changed = true;
    }

    pub fn set_category(&mut self, category: u32) {
        match self.selected_task {
            Some(selected_task_id) => {
                let mut previous_index = self.data.tasks_printeable().iter().position(|task| task.id == selected_task_id).expect("Task not found");
                self.data.set_category(selected_task_id, category);
                // Find the closest task to the previous one in case this one got invisible
                let new_tasklist = self.data.tasks_printeable();
                if new_tasklist.len() == 0 {
                    self.selected_task = None;
                } else {
                    if previous_index >= new_tasklist.len() {
                        previous_index = new_tasklist.len() - 1;
                    }
                    self.selected_task = Some(new_tasklist[previous_index].id);    
                }

                self.data_changed = true;
            }
            None => {}
        }
    }

    pub fn toggle_pomodoro(&mut self) {
        match self.selected_task {
            Some(selected) => {
                self.data.toggle_pomodoro(selected);
                self.data_changed = true;
            }
            None => {}
        }
    }
}
