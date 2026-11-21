pub mod logic;
pub mod file;

use file::choose_file;
use logic::read_file;
use std::io;

fn main(){
	let New_File = file::choose_file;
	logic::read_logic(New_File);
}

