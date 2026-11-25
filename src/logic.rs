use std::io::{self, BufReader, BufRead, Read};

pub fn read_logic<R : Read>(file: R, word: &str) -> io::Result<usize>{
	let mut reader = BufReader::new(file);
	let mut line = String::new();
	let mut wordcount = 0usize;
	
	loop {
		line.clear();
		
		if reader.read_line(&mut line) ? == 0 {
			break;
		}
		
		wordcount += line.matches(word).count()
	}
	
	Ok(wordcount)
}


