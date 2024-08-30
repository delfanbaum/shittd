use anyhow::Result;
use core::panic;
use serde_json;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::task::Task;

#[derive(Debug)]
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
        let read_file = fs::read_to_string("~/.shittd.json").expect("Unable to read in data file");
        self.tasks = serde_json::from_str(&read_file).expect("Invalid data file.");
        Ok(())
    }
}
