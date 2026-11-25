pub mod logic;
pub mod file;

use file::choose_file;
use logic::read_logic;

use std::io;

fn main() -> io::Result<()>{
	if let Some(File) = choose_file()? {
		let matched = read_logic(&File, "cat")?;
		println!("Print {matched} words")
	}
	
	Ok(())
}


