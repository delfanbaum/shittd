use crate::dates::Timeframe;
use crate::task::parse_date;
use chrono::NaiveDate;
use chrono::{Days, Local};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "shittd")]
#[command(about="A manager for your shit to do", long_about = None)]
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
        /// Optionally provide a project name for a task (for sorting)
        #[arg(short, long)]
        project: Option<String>,
        /// Optionally provide a calendar date for this task
        #[arg(short, long)]
        due_date: Option<String>,
    },
    /// Lists incomplete and completed tasks
    #[command(aliases = ["ls"])]
    List {
        #[arg(value_enum, default_value = "today")]
        timeframe: Timeframe,
    },
    #[command(arg_required_else_help = true, aliases=["complete"])]
    /// Finishes one or many tasks by ID
    Finish {
        #[arg(value_name = "TASK_ID")]
        task_id: Vec<u8>,
    },
    /// Pushes task(s) off to the following day, or optionally a specific calendar date
    #[command(arg_required_else_help = true)]
    Push {
        #[arg(value_name = "TASK_ID")]
        tasks: Vec<u8>,

        #[arg(short, long)]
        date: Option<String>,
    },
    /// Updates task(s) by project and/or date
    #[command(arg_required_else_help = true)]
    Update {
        #[arg(value_name = "TASK_ID")]
        tasks: Vec<u8>,

        #[arg(short, long)]
        project: Option<String>,

        #[arg(short, long)]
        date: Option<String>,
    },
    /// Lists the tasks due in the next three days
    Soon,
    /// Renumbers task IDs
    Renumber,
    /// Removes completed tasks from the list
    #[command(aliases = ["clear"])]
    Clean,
}

pub fn handle_date_input(date: Option<String>) -> Option<NaiveDate> {
    match date {
        Some(date) => Some(parse_date(date).expect("Unable to parse date")),
        None => Some(Local::now().date_naive() + Days::new(1)),
    }
}
