use clap::Parser;
use shittd::{
    cli::{Cli, Commands},
    db::Db,
    display::list_std,
};

fn main() {
    let mut db = Db {
        ..Default::default()
    };
    db.init().expect("Unable to open or create db");

    let args = Cli::parse();

    match args.command {
        Commands::Add { task_name } => {
            db.insert_task(task_name);
            println!("{}", list_std(&db.tasks));
        }
        Commands::List { timeframe: _ } => {
            // explicit call only needed in this case (for now) 
            db.order_tasks();
            println!("{}", list_std(&db.tasks));
        }
        Commands::Finish { task_id } => {
            db.finish_task(task_id);
            println!("{}", list_std(&db.tasks));
        }
        Commands::Clean => {
            db.remove_finished_tasks();
            println!("{}", list_std(&db.tasks));
        }
    }

    db.save().expect("Unable to write db");
}
