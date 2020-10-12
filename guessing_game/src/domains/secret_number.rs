use super::comparison_result::ComparisonResult;
use std::cmp::Ordering;

pub struct SecretNumber {
    value: i32,
}

pub fn build(value: i32) -> SecretNumber {
    SecretNumber { value }
}

impl SecretNumber {
    pub fn get_number(&self) -> i32 {
        self.value
    }

    pub fn check(&self, guess: i32) -> ComparisonResult {
        match guess.cmp(&self.value) {
            Ordering::Less => {
                println!("Too small!"); //小さすぎ！
                ComparisonResult::Less
            }
            Ordering::Greater => {
                println!("Too big!"); //大きすぎ！
                ComparisonResult::Over
            }
            Ordering::Equal => {
                println!("You win!"); //やったね！
                ComparisonResult::Equal
            }
        }
    }
}
