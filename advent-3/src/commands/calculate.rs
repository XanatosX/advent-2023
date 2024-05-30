use advent_shared::{command::Command, confirm_copy_clipboard, file_loader::FileLoader, geometry::point::Point, print_error, print_result};
use clap::Parser;

use crate::model::schematic_number::SchematicNumber;


#[derive(Parser, Debug)]
#[clap(name = "calculate", version = "1.0", author = "Xanatos", about = "Command to solve the schematics")]
pub struct Calculate{
    #[arg(long, short)]
    input: String,

    
    #[arg(long, short, action, default_value_t=false)]
    gear_ratio: bool
}

impl Command for Calculate
{
    fn execute(&self) {
        let lines = self.get_lines_of_input();
        let mut allow_point_list: Vec<char> = Vec::new();
        if self.gear_ratio {
            allow_point_list.push('*')
        }
        let symbol_points = self.get_symbol_points(&lines, allow_point_list);
        let schematic_numbers = self.get_schematic_numbers(&lines);        
        
        let result: i32;
        if !self.gear_ratio {
             result = self.solve_default_riddle(symbol_points, schematic_numbers);
        } else {
            result = self.solve_gear_ratio_riddle(symbol_points, schematic_numbers);
        }

        if result <= 0
        {
            print_error("Could not caluclate a result!");
            return;
        }

        print_result(result.to_string().as_str());
        confirm_copy_clipboard(result.to_string().as_str());
    }
}

impl Calculate
{
    fn get_lines_of_input(&self) -> Vec<String>
    {
        let mut loader = FileLoader::new(&self.input);
        loader.get_file_lines().expect("Something went wrong loading the data")
    }

    fn get_symbol_points(&self, input_lines: &Vec<String>, allow_list: Vec<char>) -> Vec<Point>
    {
        let mut return_data = Vec::new();
        let mut current_y_position: i32 = 0;
        for line in input_lines
        {
            let mut current_x_position: i32 = 0;
            for char in line.chars()
            {
                if !self.is_numeric_or_period(char)
                {
                    if allow_list.is_empty() || allow_list.contains(&char) {
                        return_data.push(Point::new(current_x_position, current_y_position))
                    }
                }
                current_x_position = current_x_position + 1;
            }
            current_y_position = current_y_position + 1;
        }
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

        return_data
    }

    fn solve_default_riddle(&self, symbol_points: Vec<Point>, schematic_numbers: Vec<SchematicNumber>) -> i32
    {
        let schematic_number_count = schematic_numbers.len();
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

        println!("{}/{}", valid_number_count, schematic_number_count);

        valid_schematics.iter().map(|schematic| schematic.get_number().clone()).sum()
    }

    fn solve_gear_ratio_riddle(&self, symbol_points: Vec<Point>, schematic_numbers: Vec<SchematicNumber>) -> i32 {
        let mut current_result = 0;
        for symbol_point in symbol_points.iter() {
            let valid_numbers: Vec<i32> = schematic_numbers.iter().filter(|number| number.point_does_touch(&symbol_point)).map(|schematic| *schematic.get_number()).collect();
            if valid_numbers.len() == 2 {
                let first_number = *valid_numbers.get(0).expect("Should not be triggered");
                let second_number = *valid_numbers.get(1).expect("Should not be triggered");
                
                current_result += first_number * second_number;
            }
        }

        current_result
    }
}