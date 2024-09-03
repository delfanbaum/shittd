//use ansi_term::Style;
//use term_size::dimensions;
use tabled::{builder::Builder, settings::Style as TabledStyle};
use term_size::dimensions;
use crate::task::Task;

pub fn list_std(tasks: &Vec<Task>) -> String {
    let mut builder = Builder::new();

    // term info
    let (width, _) = match dimensions(){
        Some((w, _)) => (w, 0),
        None => (60,0)
    };

    let _width = std::cmp::min(width.saturating_sub(60), 30);

    // header style TK
    builder.push_record([
        "ID".to_string(),
        "Task".to_string(),
        "Status".to_string(),
    ]);

    for task in tasks.iter() {
        let complete = match task.complete {
            true => " [x]".to_string(),
            false => " [ ]".to_string(),
        };


        builder.push_record([
            task.id.to_string(),
            task.name.to_string(),
            complete
        ])
        
    }

    builder.build().with(TabledStyle::psql()).to_string()
}
