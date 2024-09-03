use anyhow::Result;
use core::panic;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    pub db_path: Box<Path>,
    pub tasks: Vec<Task>,
    // config TK
}

impl Default for Db {
    fn default() -> Self {
        let home = match home::home_dir() {
            Some(path) if !path.as_os_str().is_empty() => path,
            _ => panic!("Unable to get your home dir!"),
        };
        let default_db_path = home.join(".shittd.json");
        Db {
            db_path: default_db_path.into_boxed_path(),
            tasks: Vec::new(),
        }
    }
}
impl Db {
    pub fn init(&mut self) -> Result<()> {
        // Opens or creates a file at the object's path
        if self.db_path.exists() {
            self.open()
        } else {
            let mut db_file = File::create(self.db_path.clone())?;
            db_file.write_all(b"{}")?; // empty json
            Ok(())
        }
    }

    pub fn open(&mut self) -> Result<()> {
        let read_file =
            fs::read_to_string(self.db_path.as_os_str()).expect("Unable to read in data file");
        self.tasks = serde_json::from_str(&read_file).expect("Invalid data file.");
        Ok(())
    }

    pub fn save(self) -> Result<()> {
        fs::write(
            self.db_path,
            serde_json::to_string_pretty(&self.tasks).unwrap(),
        )
        .expect("Unable to write data file.");
        Ok(())
    }

    pub fn get_next_id(&self) -> Option<u8> {
        let max_id = &self.tasks.iter().max_by_key(|task| task.id)?;
        Some(max_id.id + 1)
    }

    pub fn insert_task(&mut self, task_name: String) {
        let next_id = self.get_next_id().unwrap_or(1);

        self.tasks.push(Task {
            id: next_id,
            name: task_name,
            complete: false,
        });
    }

    pub fn finish_task(&mut self, tasks_to_finish: Vec<u8>) {
        self.tasks
            .iter_mut()
            .filter(|task| tasks_to_finish.contains(&task.id))
            .for_each(|t| t.finish())
    }

    pub fn remove_finished_tasks(&mut self) {
        self.tasks.retain(|t| !t.complete);
    }
}
