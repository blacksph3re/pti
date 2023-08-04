use chrono::{Duration, Utc, DateTime};
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use itertools::Itertools;


#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Pomodoro {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Task {
    id: u32,
    description: String,
    done: bool,
    past_pomodoros: Vec<Pomodoro>,
    active_pomodoro_jointime: Option<DateTime<Utc>>,
    parent: Option<u32>,
    category: u32,
    date_added: DateTime<Utc>,
}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub hotkey: Option<char>,
    pub visible: bool,
}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Database {
    tasks: Vec<Task>,
    categories: Vec<Category>,
    pomodoro_duration_minutes: u32,
    active_pomodoro_starttime: Option<DateTime<Utc>>,
    default_category_id: u32,
}



impl Pomodoro {
    pub fn time_spent (&self) -> Duration {
        self.end_time - self.start_time
    }
}


pub fn get_category_by_id(categories: &[Category], id: u32) -> Option<&Category> {
    categories.iter().find(|category| category.id == id)
}

impl Category {
    pub fn new(id: u32, name: String, hotkey: Option<char>) -> Category {
        Category {
            id,
            name,
            hotkey,
            visible: true,
        }
    }
}

impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}


impl Task {
    pub fn new(id: u32, description: String, category: u32) -> Task {
        Task {
            id,
            description,
            done: false,
            past_pomodoros: Vec::new(),
            active_pomodoro_jointime: None,
            category,
            parent: None,
            date_added: Utc::now(),
        }
    }

    pub fn time_spent(&self) -> Duration {
        let current_duration = match self.active_pomodoro_jointime {
            Some(jointime) => Utc::now() - jointime,
            None => Duration::zero(),
        };
        self.past_pomodoros.iter().fold(current_duration, |acc, pomodoro| acc + Pomodoro::time_spent(pomodoro))
    }

    pub fn pomodoro_active(&self) -> bool {
        self.active_pomodoro_jointime.is_some()
    }

    pub fn join_pomodoro(&mut self) {
        self.active_pomodoro_jointime = Some(Utc::now());
    }

    pub fn leave_pomodoro(&mut self) {
        if let Some(jointime) = self.active_pomodoro_jointime {
            self.past_pomodoros.push(Pomodoro {
                start_time: jointime,
                end_time: Utc::now(),
            });
            self.active_pomodoro_jointime = None;
        }
    }
}

pub struct PrinteableTask {
    pub id: u32,
    pub description: String,
    pub done: bool,
    pub time_spent: Duration,
    pub pomodoro_active: bool,
    pub indent: u32,
    pub category: Category,
    pub date_added: DateTime<Utc>,
}

impl PrinteableTask {
    pub fn new(task: &Task, categories: &[Category], indent: u32) -> PrinteableTask {
        PrinteableTask {
            id: task.id,
            description: task.description.clone(),
            done: task.done,
            time_spent: task.time_spent(),
            pomodoro_active: task.pomodoro_active(),
            indent,
            category: get_category_by_id(categories, task.category).unwrap().clone(),
            date_added: task.date_added,
        }
    }

    pub fn get_checkbox_string(&self) -> String {
        if self.done {
            "[x]".to_string()
        } else if self.pomodoro_active {
            "[*]".to_string()
        } else {
            "[ ]".to_string()
        }
    }

    pub fn get_time_spent_string(&self) -> String {
        format!("{:02}:{:02}", self.time_spent.num_hours(), self.time_spent.num_minutes() % 60)
    }

    pub fn get_category_string(&self) -> String {
        if self.category.name == "nocat" {
            "".to_string()
        } else {
            self.category.name.clone()
        }
    }

    pub fn get_description_string(&self) -> String {
        let mut description = String::new();
        for _ in 0..self.indent {
            description.push_str("  ");
        }
        description.push_str(&self.description);
        description
    }
}

pub fn get_printeable_tasklist(tasks: &[Task], categories: &[Category], parent: Option<u32>, indent: u32) -> Vec<PrinteableTask> {
    let mut current_level_tasks: Vec<&Task> = tasks.iter().filter(|task| {
        get_category_by_id(categories, task.category).unwrap().visible && task.parent == parent 
    }).collect();
    current_level_tasks.sort_by(|a, b| a.date_added.cmp(&b.date_added));
    current_level_tasks.iter().map(|task| {
        let mut children = get_printeable_tasklist(tasks, categories, Some(task.id), indent + 1);
        children.insert(0, PrinteableTask::new(task, categories, indent));
        children
    }).flatten().collect()
}

pub struct PrinteableCategory {
    pub id: u32,
    category: Category,
    default: bool,
}

impl PrinteableCategory {
    pub fn new(category: Category, default: bool) -> PrinteableCategory {
        PrinteableCategory {
            id: category.id,
            category,
            default,
        }
    }

    pub fn get_visible_string(&self) -> String {
        match self.category.visible {
            true => String::from("(x)"),
            false => String::from("( )"),
        }
    }

    pub fn get_hotkey_string(&self) -> String {
        match self.category.hotkey {
            Some(hotkey) => format!("({})", hotkey),
            None => String::new(),
        }
    }

    pub fn get_description_string(&self) -> String {
        let name = if self.category.name == "nocat" {
            "no category".to_string()
        } else {
            self.category.name.clone()
        };
        match self.default {
            true => format!("{} (default)", name),
            false => name,
        }
    }
}

impl Database {
    pub fn new() -> Database {
        Database {
            tasks: Vec::new(),
            categories: vec![Category::new(0, "nocat".to_string(), Some('u'))],
            pomodoro_duration_minutes: 25,
            active_pomodoro_starttime: None,
            default_category_id: 0,
        }
    }

    fn example_db() -> Database {
        let mut database = Database::new();
        database.categories.push(Category::new(1, "archive".to_string(), Some('a')));
        database.categories[0].visible = false;
        database.categories.push(Category::new(2, "todo".to_string(), Some('t')));
        database.default_category_id = 2;
        database.tasks.push(Task::new(0, "Task 1".to_string(), 0));
        database.tasks.push(Task::new(1, "Task 2".to_string(), 1));
        database.tasks.push(Task::new(2, "Task 3".to_string(), 2));
        database.tasks.push(Task::new(3, "Task 4".to_string(), 0));
        database
    }

    pub fn load_or_create() -> Database {
        let filename: String = match std::env::var("POMODORO_DATABASE") {
            Ok(filename) => filename,
            Err(_) => "database.json".to_string(),
        };
        match Database::from_json_file(&filename) {
            Some(database) => database,
            None => Database::example_db(),
        }
    }

    pub fn save(&self) {
        let filename: String = match std::env::var("POMODORO_DATABASE") {
            Ok(filename) => filename,
            Err(_) => "database.json".to_string(),
        };
        self.to_json_file(&filename);
    }

    fn from_json_file(path: &str) -> Option<Database> {
        let json = std::fs::read_to_string(path).ok()?;
        Some(serde_json::from_str(&json).expect("Could not parse database json"))
    }

    fn to_json_file(&self, path: &str) {
        let serialized = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, serialized).expect("Could not save database");
    }

    pub fn tasks_printeable(&self) -> Vec<PrinteableTask> {
        get_printeable_tasklist(&self.tasks, &self.categories, None, 0)
    }

    pub fn categories_printeable(&self) -> Vec<PrinteableCategory> {
        self.categories
            .clone()
            .into_iter()
            .sorted()
            .map(|category| {
                let is_default = self.default_category_id == category.id;
                PrinteableCategory::new(category, is_default)
            })
            .collect::<Vec<PrinteableCategory>>()
    }

    pub fn check_task(&mut self, task_id: u32) {
        let task = self.tasks.iter_mut().find(|task| task.id == task_id).expect("Task not found");
        task.done = !task.done;
    }

    pub fn set_category(&mut self, task_id: u32, category_id: u32) {
        let task = self.tasks.iter_mut().find(|task| task.id == task_id).expect("Task not found");
        task.category = category_id;
    }

    pub fn get_category_by_hotkey(&self, hotkey: char) -> Option<&Category> {
        self.categories.iter().find(|category| category.hotkey == Some(hotkey))
    }

    pub fn add_task_from_string(&mut self, description: String) {
        let highest_id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0);
        self.tasks.push(Task::new(highest_id+1, description, self.default_category_id));
    }

    pub fn make_default_category(&mut self, category_id: u32) {
        self.default_category_id = category_id;
    }

    pub fn toggle_category_visible(&mut self, category_id: u32) {
        let mut category = self.categories.iter_mut().find(|category| category.id == category_id).expect("Category not found");
        category.visible = !category.visible;
    }
}
