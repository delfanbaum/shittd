use chrono::{Days, Local};
use clap::Parser;
use shittd::{
    cli::{Cli, Commands},
    dates::Timeframe,
    db::{Db, TaskFilter},
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
            println!("{}", list_std(&db.todays_tasks()));
        }
        Commands::List { timeframe, project } => {
            // explicit call only needed in this case (for now)
            db.order_tasks();

            let mut filters = vec![TaskFilter::Time(timeframe)];
            if let Some(project) = project {
                filters.push(TaskFilter::Project(project))
            };

            println!("{}", list_std(&db.filter_tasks(filters)));
        }
        Commands::Push { tasks, date } => {
            let new_date = match date {
                Some(date) => Some(parse_date(date).expect("Unable to parse date")),
                None => Some(Local::now().date_naive() + Days::new(1)),
            };
            db.update_tasks(tasks, None, new_date);
            println!("{}", list_std(&db.todays_tasks()));
        }
        Commands::Update {
            tasks,
            project,
            date,
        } => {
            let update_date = date.map(|date| parse_date(date).expect("Unable to parse date"));

            db.update_tasks(tasks, project, update_date);
            println!("{}", list_std(&db.todays_tasks()));
        }
        Commands::Finish { task_id } => {
            db.finish_tasks(task_id);
            println!("{}", list_std(&db.todays_tasks()));
        }
        Commands::Soon => {
            db.order_tasks();
            let timeframe = Timeframe::Soon;
            println!(
                "{}",
                list_std(&db.filter_tasks(vec![TaskFilter::Time(timeframe)]))
            );
        }
        Commands::Renumber => {
            db.renumber_tasks();
            println!("{}", list_std(&db.todays_tasks()));
        }
        Commands::Clean => {
            db.remove_finished_tasks();
            println!("{}", list_std(&db.todays_tasks()));
        }
    }

    db.save().expect("Unable to write db");
}
