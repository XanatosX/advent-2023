#[derive(Debug, Clone)]
pub struct Scretchcard {
    id: i32,
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
    copies: i32
}

impl Scretchcard {
    pub fn new(id: i32, winning_numbers: Vec<i32>, card_numbers: Vec<i32>) -> Scretchcard {
        Scretchcard{id, winning_numbers, card_numbers, copies: 1}
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_winning_numbers(&self) -> &Vec<i32> {
        &self.winning_numbers
    }

    pub fn get_card_numbers(&self) -> &Vec<i32> {
        &self.card_numbers
    }

    pub fn get_number_of_copies(&self) -> &i32 {
        &self.copies
    }

    pub fn add_copy(&mut self) {
        self.copies += 1;
    }

    pub fn get_valid_winning_numbers(&self) -> Vec<i32> {
        let card_numbers = self.get_card_numbers();
        let winning_numbers = self.get_winning_numbers();
        let mut return_numbers = Vec::new();

        for card_number in card_numbers {
            if winning_numbers.contains(card_number) {
                return_numbers.push(*card_number)
            }
        }

        return_numbers
    }

    pub fn get_winning_number_count(&self) -> i32 {
        self.get_valid_winning_numbers().len() as i32
    }
} 