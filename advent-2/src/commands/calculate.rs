use advent_shared::file_loader::FileLoader;
use advent_shared::{confirm_copy_clipboard, print_result};
use clap::Parser;

use advent_shared::command::Command;
use crate::model::game::Game;
use crate::model::game_round::GameRound;

#[derive(Parser, Debug)]
#[clap(name = "calculate", version = "1.0", author = "Xanatos", about = "Command to calculate game result")]
pub struct Calculate 
{
    #[arg(short, long)]
    input: String,

    #[arg(long, default_value_t = 12)]
    max_red: u32,

    #[arg(long, default_value_t = 13)]
    max_green: u32,

    #[arg(long, default_value_t = 14)]
    max_blue: u32,

    #[arg(long, short, action, default_value_t=false)]
    get_min_cubes: bool
}

impl Command for Calculate {
    fn execute(&self) {
        println!("Using a maximum of {} red cubes", self.max_red);
        println!("Using a maximum of {} green cubes", self.max_green);
        println!("Using a maximum of {} blue cubes", self.max_blue);

        let lines = self.load_lines_of_file();
        let mut games = Vec::new();
        for line in lines
        {
            games.push(self.build_game_for_line(&line));
        }
        let mut result: u32 = 0;
        if self.get_min_cubes
        {
            for game in games
            {
                let max_required_red = game.get_highest_red_value();
                let max_required_green = game.get_highest_green_value();
                let max_required_blue = game.get_highest_blue_value();

                let combined = max_required_red * max_required_green * max_required_blue;
                result = result + combined;
            }
        } else {
            let valid_games = self.get_valid_games(games);
            result = valid_games.iter().map(|game| game.get_id()).sum();
        }
        
        print_result(result.to_string().as_str());
        confirm_copy_clipboard(result.to_string().as_str());
    }
}

impl  Calculate {
    fn load_lines_of_file(&self) -> Vec<String>
    {
        let mut loader = FileLoader::new(&self.input);
        loader.get_file_lines().expect("Data was not read")
    }

    fn build_game_for_line(&self, line: &str) -> Game
    {
        let splitted = line.split_once(':').unwrap();
        let games = splitted.1;
        let id_char = splitted.0.replace("Game ", "");

        let id = id_char.trim().parse::<u32>().expect("Could not parse the data to number!");
        
        let mut game = Game::new(id);

        for round in games.split(";")
        {
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;
            for data in round.split(",")
            {
                let split = data.trim().split_once(' ').unwrap();
                let color = &split.1.to_lowercase();
                let value = split.0.parse::<u32>().expect("Could not parse");

                match color.as_str() {
                    "red" => red = value,
                    "green" => green = value,
                    "blue" => blue = value,
                    _ => continue
                }
            }
            game.add_round(GameRound::new(red, green, blue));
        }
        game
    }

    fn get_valid_games(&self, games: Vec<Game>) -> Vec<Game>
    {
        games.iter().filter(|game| game.get_highest_red_value() <= self.max_red)
                    .filter(|game| game.get_highest_green_value() <= self.max_green)
                    .filter(|game| game.get_highest_blue_value() <= self.max_blue)
                    .map(|game| game.clone())
                    .collect()
    }
}