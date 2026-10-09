use std::io;
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


//	while quit = False
//		println!("
//	println!("
	println!("{}", print_menu());
	let number = read_number(1,5);
	println!("{}", number)


}

