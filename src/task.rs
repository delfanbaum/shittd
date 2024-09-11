use chrono::{DateTime, Days, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u8,
    pub name: String,
    pub date: NaiveDate,
    pub complete: bool,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: 0,
            name: "Some Task".to_string(),
            date: Local::now().date_naive(),
            complete: false,
        }
    }
}

impl Task {
    // ignore dates for the time being; start simple!
    pub fn push(&mut self) {
        self.date = self.date.checked_add_days(Days::new(1)).unwrap()
    }
    pub fn finish(&mut self) {
        self.complete = true
    }
}

pub fn parse_date(value: String) -> Result<NaiveDate, String> {
    if let Ok(date) = value.parse::<DateTime<Local>>() {
        Ok(date.date_naive())
    } else {
        let date = value
            .parse::<NaiveDate>()
            .map_err(|e| format!("Need a valid RFC3339-formatted date or datetime: {e}"))?;
        Ok(date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .date_naive())
    }
}
