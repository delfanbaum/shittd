use chrono::{Days, Local};
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
        Commands::Add {
            tasks,
            project,
            due_date,
            today,
        } => {
            let mut task_date = None;
            if let Some(date) = due_date {
                task_date = Some(parse_date(date).expect("Unable to parse date"));
            } else if today {
                task_date = Some(Local::now().date_naive())
            };

            for task in tasks {
                db.insert_task(task, project.clone(), task_date);
            }
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::List { timeframe } => {
            // explicit call only needed in this case (for now)
            db.order_tasks();
            println!("{}", list_std(&db.tasks, timeframe));
        }
        Commands::Push { tasks, date } => {
            let new_date = match date {
                Some(date) => Some(parse_date(date).expect("Unable to parse date")),
                None => Some(Local::now().date_naive() + Days::new(1)),
            };
            db.update_tasks(tasks, None, new_date);
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::Update {
            tasks,
            project,
            date,
        } => {
            let update_date = match date {
                Some(date) => Some(parse_date(date).expect("Unable to parse date")),
                None => None,
            };
            db.update_tasks(tasks, project, update_date);
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::Finish { task_id } => {
            db.finish_tasks(task_id);
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::Soon => {
            db.order_tasks();
            let timeframe = Timeframe::Soon;
            println!("{}", list_std(&db.tasks, timeframe));
        }
        Commands::Renumber => {
            db.renumber_tasks();
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
        Commands::Clean => {
            db.remove_finished_tasks();
            println!("{}", list_std(&db.tasks, Timeframe::Today));
        }
    }

    db.save().expect("Unable to write db");
}
