use clap::{Parser, Subcommand, ValueEnum};
//use chrono::{NaiveDate, Local};

#[derive(Debug, Parser)]
#[command(name = "shittd")]
#[command(about="Shit to do", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(value_name = "TASK_NAME")]
        task_name: String,

        //// this feels like it needs some kind of parser
        //#[arg(short, long, default_value = Local::now())]
        //date: String,
    },
    List {
        #[arg(value_enum, short, long, default_value = "today")]
        timeframe: Timeframe,
    },
    #[command(arg_required_else_help = true)]
    Finish {
        #[arg(value_name = "TASK_ID")]
        task_id: Vec<u8>,
    },
    //#[command(arg_required_else_help = true)]
    //Push {
    //    #[arg(value_name = "TASK_ID")]
    //    task_id: Vec<u8>,
    //},
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Timeframe {
    Today,
    Tomorrow,
    Week,
    Any,
}
