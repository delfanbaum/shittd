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
        }
        Commands::List { timeframe: _ } => {
            println!("{}", list_std(&db.tasks));
        }
        _ => {
            println!("{:?}", args)
        }
    }

    db.save().expect("Unable to write db");
}
