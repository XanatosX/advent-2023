use advent_shared::command::Command;
use advent_shared::file_loader::FileLoader;
use advent_shared::{confirm_copy_clipboard, print_result};
use clap::Parser;

use crate::model::scratchcard::Scretchcard;

#[derive(Parser, Debug)]
#[clap(name = "calculate", version = "1.0", author = "Xanatos", about = "Command to calculate game result")]
pub struct Calculate 
{
    #[arg(short, long)]
    input: String,
}

impl Command for Calculate {
    fn execute(&self) {
        let scretchcards = self.get_cards();
        let mut result = 0;

        for card in scretchcards {
            let matchings = card.get_winning_number_count();
            let value;
            if matchings <= 0 {
                continue;
            }
            if matchings == 1 {
                value = 1;
            } else {
                let base: i32 = 2;
                let matching_changed = matchings - 1;
                value = base.pow(matching_changed.try_into().unwrap());
            }
            result += value;
        }
        
        print_result(result.to_string().as_str());
        confirm_copy_clipboard(result.to_string().as_str());
    }
}

impl Calculate {
    fn get_cards(&self) -> Vec<Scretchcard> {
        let mut file_loader = FileLoader::new(&self.input);
        let lines = file_loader.get_file_lines().expect("Something went wrong loading the data");

        let mut return_data = Vec::new();
        for line in lines {
            let splitted = line.split_once('|').expect("Split did not work");
            let winning_numbers_data = splitted.0.split_once(':').expect("Split did not work for wining numbers").1.trim();


            let card_numbers = splitted.1.trim();


            return_data.push(Scretchcard::new(self.get_numbers_from_string(winning_numbers_data), self.get_numbers_from_string(card_numbers)))
        }
        return_data
    }

    fn get_numbers_from_string(&self, data: &str) -> Vec<i32> {
        let mut current_number = String::new();
        let mut return_numbers = Vec::new();
        for character in data.trim().chars() {
            if character.is_numeric() {
                current_number += character.to_string().as_str();
                continue;
            }
            if current_number.is_empty() {
                continue;
            }
            let number = current_number.parse::<i32>().unwrap();
            return_numbers.push(number);
            current_number = String::new();
        }
        if current_number.len() > 0 {
            let number = current_number.parse::<i32>().unwrap();
            return_numbers.push(number);
        }
        
        println!("{:?}", return_numbers);
        return_numbers
    }
}