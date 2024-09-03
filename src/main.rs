use clap::Parser;
use shittd::{
    cli::{Cli, Commands},
    db::Db,
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
        },
        _ => {
            println!("{:?}", args)
        }
    }

    db.save().expect("Unable to write db");
}
