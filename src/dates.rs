use crate::task::Task;
use chrono::{Days, Local, Weekday};
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Timeframe {
    Today,
    Tomorrow,
    Week,
    All,
}

pub fn task_in_timeframe(task: &Task, timeframe: Timeframe) -> bool {
    let today = Local::now().date_naive();
    match timeframe {
        Timeframe::Today => task.date <= today,
        Timeframe::Tomorrow => task.date <= today + Days::new(1),
        Timeframe::Week => task.date <= today.week(Weekday::Mon).last_day(),
        Timeframe::All => true,
    }
}
