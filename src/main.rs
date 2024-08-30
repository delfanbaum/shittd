use clap::Parser;
use shittd::{cli::Cli, db::Db};

fn main() {
    let mut db = Db{..Default::default()};
    db.init().expect("Unable to open or create db");
    let args = Cli::parse();

    println!("{:?}", args);
}
