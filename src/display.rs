use crate::{
    dates::{task_in_timeframe, Timeframe},
    task::Task,
};
use chrono::Local;
use tabled::{builder::Builder, settings::Style as TabledStyle};
use term_size::dimensions;

pub fn list_std(tasks: &[Task], timeframe: Timeframe) -> String {
    let mut builder = Builder::new();

    // header style TK
    builder.push_record([
        "ID".to_string(),
        "Task".to_string(),
        "Date".to_string(),
        "Status".to_string(),
    ]);

    // term info
    let (width, _) = match dimensions() {
        Some((w, _)) => (w, 0),
        None => (80, 0),
    };

    let text_width = {
        width - 3 // ID plus
        - 12 // the separators
        - "%Y-%m-%d".len()
        - "Status".len()
    };

    let mut group_date = Local::now().date_naive();

    for task in tasks.iter().filter(|t| task_in_timeframe(t, timeframe)) {
        let mut wrapped_text = String::new();
        let wrapped_lines = textwrap::wrap(task.name.as_str(), text_width);
        for line in wrapped_lines {
            wrapped_text.push_str(&format!("{line}\n"));
        }
        wrapped_text = wrapped_text.trim().to_string();

        let complete = match task.complete {
            true => " [x]".to_string(),
            false => " [ ]".to_string(),
        };

        // separate days
        if task.date > group_date {
            // reset
            group_date = task.date;
            // add seperator
            builder.push_record(["", "", "", ""])
        }

        builder.push_record([
            task.id.to_string(),
            wrapped_text,
            task.date.format("%Y-%m-%d").to_string(),
            complete,
        ])
    }

    println!(); // blank line for beauty reasons
    builder.build().with(TabledStyle::modern()).to_string()
}
