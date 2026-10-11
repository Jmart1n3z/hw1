use std::io;
use rand::RngExt;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
//struct Menu {
	
fn main() {
	let mut gold : u64 = 100;
	let food_price : u64 = 5;
	let max_seed : u64 = 9999;
	let mut food : u64 = 0;
	let mut day : u64 = 1;
	let mut roll_count : u64 = 0;
	let mut quit : bool = false;
	
	fn print_menu() -> String {
		let menu = format!("1) Buy food\n2) Hunt\n3) Trade with caravan\n4) View status\n5) Quit");
		menu
		}

	fn read_number(min: u64, max: u64) -> u64 {
		loop {
			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("Failed to read line");
			let number : u64 = match input.trim().parse() {
				Ok(n) => n,
				Err(_) => {
					println!("Invalid input, Try again");
					continue
					}
				};
			if number < min {
				println!("Invalid input. Try again");
				continue
				}
			else if number > max {
				println!("Invalid input. Try again");
				continue
				}
			else {
				return number;
				}
			}
		}

	fn roll_die(seed: u64) -> u64 {
		let mut rand = StdRng::seed_from_u64(seed);
		let num : u64 = rand.random_range(1..=6);
		num
		};
	
//	fn roll_many(seed: u64, count: u64) -> u64 {
//		let mut sum = 0;
//		for i in 0.. count {
//			sum = seed + seed + 1 + 
//			}
//		};

	fn hunt_food(roll: u64) -> u64 {
		let mut fd_gain : u64 = 0;
		if roll == 1 {
			fd_gain = 0
		} else if roll == 2 || roll == 3 {
			fd_gain = 10
		} else if roll == 4 || roll == 5 {
			fd_gain = 20
		} else if roll == 6 {
			fd_gain = 40
		}
		fd_gain
		}
//	while quit = False
//		println!("
//	println!("
	println!("{}", print_menu());
	let number = read_number(1,5);
	println!("{}", number);
	let mut rng = rand::rng();
	let roll : i32 = rng.random_range(1..=6);
	food = food + hunt_food(roll.try_into().unwrap());
	println!("{}", food)
}

