extern crate rand;

use rand::Rng;
use std::io;

fn main() {
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(0, 100) + 1;
	println!("Number {}", secret_number);
	println!("Input: ");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("you guessed: {}", guess);
}
