use chrono::{Duration, Utc, DateTime};
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;


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
    category: Option<u32>,
    date_added: DateTime<Utc>,
}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Category {
    id: u32,
    name: String,
    order: u32,
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
    pub fn new(id: u32, name: String) -> Category {
        Category {
            id,
            name,
            order: id,
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
        self.order.cmp(&other.order)
    }
}


impl Task {
    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            done: false,
            past_pomodoros: Vec::new(),
            active_pomodoro_jointime: None,
            category: None,
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

impl Database {
    pub fn new() -> Database {
        Database {
            tasks: Vec::new(),
            categories: Vec::new(),
            pomodoro_duration_minutes: 25,
            active_pomodoro_starttime: None,
        }
    }

    pub fn load_or_create() -> Database {
        let filename: String = match std::env::var("POMODORO_DATABASE") {
            Ok(filename) => filename,
            Err(_) => "database.json".to_string(),
        };
        match Database::from_json_file(&filename) {
            Some(database) => database,
            None => Database::new(),
        }
    }

    pub fn from_json_file(path: &str) -> Option<Database> {
        let json = std::fs::read_to_string(path).ok()?;
        Database::from_json(&json)
    }

    fn from_json(json: &str) -> Option<Database> {
        serde_json::from_str(json).ok()
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_json_file(&self, path: &str) {
        std::fs::write(path, self.to_json()).unwrap();
    }
}