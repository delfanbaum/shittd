use chrono::{DateTime, Days, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u8,
    pub name: String,
    /// Eventually this will be like a foreign key
    pub project: Option<String>,
    /// Date at which at ask is actually due
    pub due_date: Option<NaiveDate>,
    /// Date after which task is no longer valid and can be removed
    pub invalid_date: Option<NaiveDate>,
    pub complete: bool,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: 0,
            name: "Some Task".to_string(),
            project: None,
            due_date: None,
            invalid_date: None,
            complete: false,
        }
    }
}

impl Task {
    // pushing sets the date to "tomorrow"
    pub fn push(&mut self) {
        self.due_date = Some(Local::now().date_naive() + Days::new(1))
    }

    pub fn update(&mut self, project: Option<String>, due_date: Option<NaiveDate>) {
        if self.project.is_none() {
            self.project = project;
        }
        if self.due_date.is_none() {
            self.due_date = due_date;
        }
    }

    pub fn finish(&mut self) {
        self.complete = true
    }

    pub fn renumber(&mut self, new_id: u8) {
        self.id = new_id
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

pub fn task_in_project(task: &Task, project: &str) -> bool {
    match &task.project {
        Some(task_project) => task_project == project,
        None => false,
    }
}
