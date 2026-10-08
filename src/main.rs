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
//	while quit = False
//		println!("
//	println!("
	println!("{}", print_menu())
}
