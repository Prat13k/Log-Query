use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_logic(file: &[u8], word: &u8) -> io::Result<usize>{
	let mut reader = BufReader::new(file);
	let mut line = String::new();
	
	let len = reader.read_line(&mut line)?;
	Ok(())
}
