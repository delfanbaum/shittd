use serde::{Deserialize, Serialize};
//use chrono::{Days, NaiveDate};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u8,
    pub name: String,
    //pub date: NaiveDate,
    pub complete: bool,
}

impl Task {
    // ignore dates for the time being; start simple!
    //pub fn push(&mut self) {
    //    self.date = self.date.checked_add_days(Days::new(1))
    //}
    pub fn finish(&mut self) {
        self.complete = true
    }
}
