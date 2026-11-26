pub mod logic;
pub mod file;

use file::choose_file;
use logic::read_logic;

use std::io::{self, Write};

fn main() -> io::Result<()>{
	println!("Enter a string: ");
	let _ = io::stdout().flush();
	
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input" );
	
	let word = input.trim();
	println!("Your word : {}", word);
	
	if let Some(newfile) = choose_file()? {
		let matched = read_logic(&newfile, word)?;
		println!("{matched} words")
	}
	
	Ok(())
}


