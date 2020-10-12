extern crate rand;

mod domains;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = domains::secret_number::build(rand::thread_rng().gen_range(1, 101));

    println!("The secret number is: {}", secret_number.get_number()); //秘密の数字は次の通り: {}

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        if secret_number.check(guess).is_equal() {
            break;
        }
    }
}
