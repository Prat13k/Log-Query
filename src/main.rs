pub mod logic;
pub mod file;

use file::choose_file;
use logic::read_logic;

use std::io::{self, Write};

fn main() -> io::Result<()>{
	let mut input = String::new();
	println!("Enter a string: ");
	
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut input).expect("Failed to read input" );
	
	println!("Your word : {}", input.trim());
	
	if let Some(File) = choose_file()? {
		let matched = read_logic(&File, &input)?;
		println!("{matched} words")
	}
	
	Ok(())
}


