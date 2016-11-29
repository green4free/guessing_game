extern crate rand;

use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(0, 100) + 1;
	println!("Number {}", secret_number);
	println!("Input: ");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	let guess: u32 = guess.trim().parse().expect("Please type a number");
	
	println!("you guessed: {}", guess);
	match guess.cmp(&secret_number) {
	Ordering::Less => println!("Too small!"),
	Ordering::Greater => println!("Too big!"),
	Ordering::Equal => println!("You win!"),
	}
}
