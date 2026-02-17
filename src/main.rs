use std::fs; //used to read files
use std::io; //for input output

fn main(){	
	let mut word_find = String::new();
	io::stdin()
		.read_line(&mut word_find)
		.expect("Failed to read lines");
	let word_find = word_find.trim();
	
	read_file("documents/file.txt", &word_find);
}

fn read_file(path:&str, word:&str){
	let bytes = fs::read_to_string(path).expect("Failed to load file");
	let mut all_lines: Vec<String> = Vec::new();
	for values in bytes.split('.'){
		let values = values.trim();
		if values.contains(word){
			all_lines.push(values.to_string());		
		}
	}
	let length = all_lines.len();
	println!("{:#?},\n the total line contains {word} is {}",all_lines, length);
}
