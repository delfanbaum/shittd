use crate::task::Task;
use chrono::{Days, Local, Weekday};
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Timeframe {
    Today,
    Tomorrow,
    Week,
    Soon,
    All,
}

pub fn task_in_timeframe(task: &Task, timeframe: Timeframe) -> bool {
    let today = Local::now().date_naive();
    match timeframe {
        Timeframe::Today => task.due_date <= Some(today),
        Timeframe::Tomorrow => task.due_date <= Some(today + Days::new(1)),
        Timeframe::Week => task.due_date <= Some(today.week(Weekday::Mon).last_day()),
        Timeframe::Soon => task.due_date <= Some(today + Days::new(3)),
        Timeframe::All => true,
    }
}
