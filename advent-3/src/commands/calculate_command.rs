use advent_shared::{command::Command, confirm_copy_clipboard, file_loader::FileLoader, geometry::point::Point, print_result};
use clap::Parser;

use crate::model::schematic_number::SchematicNumber;


#[derive(Parser, Debug)]
#[clap(name = "calculator", version = "1.0", author = "Xanatos", about = "Command to solve the schematics")]
pub struct Calculator{
    #[arg(long, short)]
    input: String
}

impl Command for Calculator
{
    fn execute(&self) {
        let lines = self.get_lines_of_input();
        let schematic_numbers = self.get_schematic_numbers(&lines);
        let schematic_number_count = schematic_numbers.len();
        let symbol_points = self.get_symbol_points(&lines);

        let mut valid_schematics: Vec<SchematicNumber> = Vec::new();
        for symbol_point in symbol_points
        {
            let valid_numbers: Vec<&SchematicNumber> = schematic_numbers.iter().filter(|number| number.point_does_touch(&symbol_point)).collect();
            for valid_number in valid_numbers
            {
                if valid_schematics.iter().any(|schematic| schematic.get_start_position() == valid_number.get_start_position())
                {
                    continue;
                }
                valid_schematics.push(valid_number.clone())
            }
        }
        let valid_number_count = valid_schematics.len();

        //let numbers: Vec<&i32> = valid_schematics.iter().map(|item| item.get_number()).collect();
        //println!("{:?}", numbers);
        println!("{}/{}", valid_number_count, schematic_number_count);
        //let first_row: Vec<&SchematicNumber> = valid_schematics.iter().filter(|item| *item.get_start_position().get_y() == 139).collect();
        //println!("{}: {:?}", first_row.len(), first_row);
        
        let result: i32 = valid_schematics.iter().map(|schematic| schematic.get_number().clone()).sum();

        print_result(result.to_string().as_str());
        confirm_copy_clipboard(result.to_string().as_str());
    }
}

impl Calculator
{
    fn get_lines_of_input(&self) -> Vec<String>
    {
        let mut loader = FileLoader::new(&self.input);
        loader.get_file_lines().expect("Something went wrong loading the data")
    }

    fn get_symbol_points(&self, input_lines: &Vec<String>) -> Vec<Point>
    {
        //let allowed_chars = vec!['*', '#', '+', '-', '$', '&', '/', '%', '@', '='];
        let mut return_data = Vec::new();
        let mut current_y_position: i32 = 0;
        for line in input_lines
        {
            let mut current_x_position: i32 = 0;
            for char in line.chars()
            {
                if !self.is_numeric_or_period(char)
                {
                    return_data.push(Point::new(current_x_position, current_y_position))
                }
                current_x_position = current_x_position + 1;
            }
            current_y_position = current_y_position + 1;
        }
        //println!("{:?}", return_data);
        return_data
    }
    
    fn is_numeric_or_period(&self, c: char) -> bool
    {
        char::is_numeric(c) || c == '.'
    }

    fn get_schematic_numbers(&self, input_lines: &Vec<String>) -> Vec<SchematicNumber>
    {
        let mut return_data = Vec::new();
        let mut current_y_position: i32 = 0;
        for line in input_lines
        {
            let mut current_x_position: i32 = 0;
            let mut current_number = String::new();
            let mut current_start_position: i32 = -1;
            for char in line.chars()
            {
                if char.is_numeric() {
                    
                    if current_start_position == -1
                    {
                        current_start_position = current_x_position.clone();
                    }
                    current_number.push(char);
                } else {
                    if current_start_position > -1
                    {
                        let number = current_number.parse::<i32>().unwrap();
                        return_data.push(SchematicNumber::new(number, current_start_position, current_y_position))
                    }
                    current_number = String::new();
                    current_start_position = -1;
                }
                current_x_position = current_x_position + 1;
            }
            //Code to add the last number of the line as well
            if current_start_position > -1
            {
                let number = current_number.parse::<i32>().unwrap();
                return_data.push(SchematicNumber::new(number, current_start_position, current_y_position))
            }

            current_y_position = current_y_position + 1;
        }

        //println!("{:?}", return_data);
        return_data
    }
}