use std::path::PathBuf;
use std::env;
use dirs::home_dir;

pub const TASK_FILE: &str = "database.json";
pub const ALARM_FILE: &str = "alarm.mp3";

pub fn get_full_path(file: &str) -> PathBuf {
    let mut dir = match env::var("PTI_STORAGE_DIR") {
        Ok(val) => PathBuf::from(val),
        Err(_) => {
            let mut default_dir = home_dir().unwrap();
            default_dir.push(".pti");
            default_dir
        }
    };
    dir.push(file);
    dir
}