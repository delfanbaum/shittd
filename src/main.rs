use clap::Parser;
use shitd::cli::Cli;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args)
}
