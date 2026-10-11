use std::io;
use rand::RngExt;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn print_menu() {
		println!("1) Buy food");
		println!("2) Hunt");
		println!("3) Trade with caravan");
		println!("4) View status");
		println!("5) Quit")
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
		}
	
	fn roll_many(seed: u64, count: u64) -> u64 {
		let mut sum = 0;
		for i in 0.. count {
			sum = sum + roll_die(seed + i) 
			}
		sum as u64
		}

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

	fn can_afford(gold: u64, cost: u64) -> bool {
		gold >= cost
		}

	fn print_status(gold: u64, food: u64, day: u64) {
		println!("Gold is {}", gold);
		println!("Food is {}", food);
		println!("Day is {}", day)
		}
	
fn main() {
	let mut gold : u64 = 100;
	let food_price : u64 = 5;
	let mut food : u64 = 0;
	let mut day : u64 = 1;
	let mut roll_count : u64 = 0;
	let mut quit : bool = true;
	let mut sd = rand::rng();
	let seed = sd.random_range(1..=9999);

	println!("Welcome to Trail Trader");
	while quit {
		loop {
			println!("");

			print_menu();

			println!("");
			
			let number = read_number(1,5);
	
			println!("");
					
			if number == 1 {
				if can_afford(gold, food_price) {
					println!("How many rations do you want to buy?");
					let mut input = String::new();
					io::stdin()
						.read_line(&mut input)
						.expect("Failed to read line");
					let number : u64 = match input.trim().parse() {
						Ok(n) => n,
						Err(_) => {
							println!("Invalid input, Try again");
							println!("");
							continue;
							}
						};
					food = food + number;
					gold = gold - food_price*number;
					day = day + 1;
					continue
				} else {
					println!("You can't afford to buy food");
					println!("");
					continue
				}

			} else if number == 2 {
				println!("Hunt food: ");
				let roll = roll_die(seed + roll_count);
				roll_count = roll_count + 1;
				let fd_gain = hunt_food(roll);
				food = food + fd_gain;
				day = day + 1;
				println!("Food gained: {}", fd_gain);
				continue
			
			} else if number == 3 {
				println!("Trade with caravan");
				let sum = roll_many(seed + roll_count, 3);
				roll_count = roll_count + 3;
				let gld_gain = sum * 2;
				gold = gold + gld_gain;
				day = day + 1;
				println!("You gained {} gold", gld_gain);
				continue
			} else if number == 4 {
				print_status(gold, food, day);
				continue
			} else if number == 5 {
				println!("The game has ended!");
				print_status(gold, food, day);
				quit = false;
				break
			}
			}
		}
		println!("Thanks for playing!")
}
