use clap::{Parser, Subcommand, ValueEnum};
//use chrono::{NaiveDate, Local};

#[derive(Debug, Parser)]
#[command(name = "shittd")]
#[command(about="A manager for your shit to do list", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    /// Adds one or more tasks to the list
    Add {
        #[arg(value_name = "TASK_NAME")]
        tasks: Vec<String>,
        //// this feels like it needs some kind of parser
        //#[arg(short, long, default_value = Local::now())]
        //date: String,
    },
    /// Lists incomplete and completed tasks
    #[command(aliases = ["ls"])]
    List {
        #[arg(value_enum, short, long, default_value = "today")]
        timeframe: Timeframe,
    },
    #[command(arg_required_else_help = true)]
    /// Finishes one or many tasks by ID
    Finish {
        #[arg(value_name = "TASK_ID")]
        task_id: Vec<u8>,
    },
    //#[command(arg_required_else_help = true)]
    //Push {
    //    #[arg(value_name = "TASK_ID")]
    //    task_id: Vec<u8>,
    //},
    /// Removes completed tasks from the list
    Clean,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Timeframe {
    Today,
    Tomorrow,
    Week,
    Any,
}
