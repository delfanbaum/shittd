use chrono::Local;
use clap::Parser;
use shittd::{
    cli::{Cli, Commands},
    dates::Timeframe,
    db::Db,
    display::list_std,
    task::parse_date,
};

fn main() {
    let mut db = Db {
        ..Default::default()
    };

    let args = Cli::parse();

    match db.init() {
        Ok(_) => (),
        Err(_) => db.update().expect("Unable to read or automatically correct the database file. Please manually fix or update ~/.shittd.json and try again"),
    }

    match args.command {
        Commands::Add { tasks, date } => {
            let mut task_date = Local::now().date_naive();
            if date.is_some() {
                task_date = parse_date(date.unwrap()).expect("Unable to parse date");
            }
            for task in tasks {
                db.insert_task(task, task_date);
            }
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::List { timeframe } => {
            // explicit call only needed in this case (for now)
            db.order_tasks();
            println!("{}", list_std(&db.tasks, timeframe));
        }
        Commands::Push { task_id } => {
            db.push_tasks(task_id);
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::Finish { task_id } => {
            db.finish_tasks(task_id);
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::Clean => {
            db.remove_finished_tasks();
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
    }

    db.save().expect("Unable to write db");
}
