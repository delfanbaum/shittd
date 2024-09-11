use crate::{
    dates::{task_in_timeframe, Timeframe},
    task::Task,
};
use tabled::{builder::Builder, settings::Style as TabledStyle};
use term_size::dimensions;

pub fn list_std(tasks: &[Task], timeframe: Timeframe) -> String {
    let mut builder = Builder::new();

    // term info
    let (width, _) = match dimensions() {
        Some((w, _)) => (w, 0),
        None => (60, 0),
    };

    let _width = std::cmp::min(width.saturating_sub(60), 30);

    // header style TK
    builder.push_record([
        "ID".to_string(),
        "Task".to_string(),
        "Date".to_string(),
        "Status".to_string(),
    ]);

    for task in tasks.iter().filter(|t| task_in_timeframe(t, timeframe)) {
        let complete = match task.complete {
            true => " [x]".to_string(),
            false => " [ ]".to_string(),
        };

        builder.push_record([
            task.id.to_string(),
            task.name.to_string(),
            task.date.format("%Y-%m-%d").to_string(),
            complete,
        ])
    }

    println!(); // blank line for beauty reasons
    builder.build().with(TabledStyle::psql()).to_string()
}
