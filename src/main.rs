use std::io;

fn main() {
	println!("Guess the number!");
	println!("Input: ");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("you guessed: {}", guess);
}
