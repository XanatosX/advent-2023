mod commands;

use advent_shared::command::Command;
use clap::{command, Parser};
use commands::calculate::Calculate;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Args
{
    Calculate(Calculate)
}


fn main() {
    match Args::parse() 
    {
        Args::Calculate(calculate_command) => calculate_command.execute(),
        
    }
}
