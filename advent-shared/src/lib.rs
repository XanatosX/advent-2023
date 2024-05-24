pub mod command;
pub mod file_loader;
pub mod prelude;
pub mod error;
pub mod geometry;

use cli_clipboard::set_contents;
use colored::Colorize;
use dialoguer::Confirm;


pub fn confirm_copy_clipboard(value: &str)
{
    if value.len() == 0
    {
        return;
    }

    if Confirm::new().with_prompt("Do you want to copy the result to your clipboard?").interact().unwrap_or(false)
    {
        set_contents(value.to_string()).expect("Could not set clipboard content")
    }
}

pub fn print_result(value: &str)
{
    println!("{}", value.green())
}

pub fn print_error(error: &str) {
    println!("{}", error.red())
}