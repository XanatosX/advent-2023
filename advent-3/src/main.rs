use advent_shared::command::Command;
use clap::{command, Parser};
use commands::calculate_command::Calculator;

mod commands;
mod model;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Args
{
    Calculate(Calculator)
}

fn main() {
    match Args::parse()
    {
        Args::Calculate(calculate_command) => calculate_command.execute()
    }
}

