use crate::{
    dates::{task_in_timeframe, Timeframe},
    task::Task,
};
use chrono::Local;
use tabled::{builder::Builder, settings::Style as TabledStyle};
use term_size::dimensions;

pub fn list_std(tasks: &[Task], timeframe: Timeframe) -> String {
    let mut builder = Builder::new();

    let (proj_len, date_len, done_len) = (" Project ".len(), " %Y-%m-%d ".len(), " Done ".len());
    let max_display_width = 100;

    // header style TK
    builder.push_record([
        "ID".to_string(),
        "Task".to_string(),
        "Project".to_string(),
        "Date".to_string(),
        "Done".to_string(),
    ]);

    // term info
    let (width, _) = match dimensions() {
        Some((w, _)) => {
            if w > max_display_width {
                (max_display_width, 0)
            } else {
                (w, 0)
            }
        }
        None => (80, 0),
    };

    let text_width = {
        width - 3 // ID plus
        - 12 // the separators
        - proj_len
        - date_len
        - done_len
    };

    let mut group_date = Local::now().date_naive();

    for task in tasks.iter().filter(|t| task_in_timeframe(t, timeframe)) {
        let mut wrapped_text = String::new();
        let wrapped_lines = textwrap::wrap(task.name.as_str(), text_width);
        for line in wrapped_lines {
            let whitespace = " ".repeat(text_width - line.len());
            wrapped_text.push_str(&format!("{line}{whitespace}\n"));
        }
        // remove trailing newline
        wrapped_text
            .pop()
            .expect("Error popping training newline char");
        wrapped_text = wrapped_text.to_string();

        let complete = match task.complete {
            true => " [x]".to_string(),
            false => " [ ]".to_string(),
        };

        // separate days
        let display_date = match task.due_date {
            Some(date) => {
                // if needed, reset group date and add seperator
                if date > group_date {
                    group_date = date;
                    builder.push_record(["", "", "", "", ""]);
                }
                // bind date to variable
                date.format("%Y-%m-%d").to_string()
            }
            None => "".into(),
        };

        let display_project: String = match &task.project {
            Some(project) => project.chars().take("project".len()).collect(),
            None => "".into(),
        };

        builder.push_record([
            task.id.to_string(),
            wrapped_text,
            display_project,
            display_date,
            complete,
        ])
    }

    println!(); // blank line for beauty reasons
    builder.build().with(TabledStyle::modern()).to_string()
}
