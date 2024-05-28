use core::panic;
use advent_shared::{command::Command, confirm_copy_clipboard, file_loader::FileLoader, print_error, print_result};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "edit", version = "1.0", author = "Xanatos", about = "Command to calculate the coordinates")]
pub struct Calculate
{
    #[arg(short, long)]
    input_path: String,

   #[arg(long, short, action, default_value_t=false)]
   replace_letters: bool,
}

impl Command for Calculate {
    fn execute(&self) {
        let lines = self.load_coordinate_files();
        let final_coordinate = self.sum_all_coordinates(lines);

        let final_coordinate_string = final_coordinate.to_string();
        if final_coordinate > 0
        {
            print_result(final_coordinate_string.as_str());
        } else {
            print_error("The value is equal to 0, seems like something went wrong!");
            return;
        }
        
        confirm_copy_clipboard(final_coordinate_string.as_str());
    }
}

impl Calculate
{
    fn load_coordinate_files(&self) -> Vec<String>
    {
        let mut loader = FileLoader::new(&self.input_path);
        let mut lines = Vec::new();
        for line in loader.get_file_lines().expect("Data was not read")
        {
            let mut changed_line = line.to_string();
            if self.replace_letters
            {
                changed_line = changed_line.replace("one", "o1e")
                                           .replace("two", "t2o")
                                           .replace("three", "t3e")
                                           .replace("four", "f4r")
                                           .replace("five", "f5e")
                                           .replace("six", "s6x")
                                           .replace("seven", "s7n")
                                           .replace("eight", "e8t")
                                           .replace("nine", "n9e");
            }
            lines.push(changed_line);
        }

        lines
    }

    fn calculate_coordinate_per_line(&self, line: &str) -> u32
    {
        let mut first = String::new();
        let mut first_found = false;
        let mut last = String::new();
        for char in line.chars()
        {
            if char.is_numeric()
            {
                if !first_found
                {
                    first = char.clone().to_string();
                    first_found = true;
                }
                last = char.clone().to_string();
            }
        }

        let combined = format!("{}{}", first, last);
        match combined.parse::<u32>()
        {
            Ok(number) => number,
            Err(_) => panic!(),
        }
    }

    fn sum_all_coordinates(&self, coordinates: Vec<String>) -> u32
    {
        if coordinates.is_empty()
        {
            return 0;
        }
        let mut value_lines = Vec::new();
        for line in coordinates.iter()
        {
            let value = self.calculate_coordinate_per_line(line);
            value_lines.push(value);
        }

        value_lines.iter().sum()
    }
}