use serde::{Serialize, Deserialize};
use chrono::{Days, NaiveDate};


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u8,
    pub name: String,
    pub date: NaiveDate,
    pub complete: bool
}

impl Task {
    pub fn push(&mut self) {
        self.date = self.date.checked_add_days(Days::new(1))
    }
    pub fn finish(&mut self) {
        self.complete = true
    }
}
