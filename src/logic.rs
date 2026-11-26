use std::io::{self, BufReader, BufRead, Read};
use memchr;

const BUF_CAPACITY: usize = 64 * 1024;  

pub fn read_logic<R : Read>(hay: R, needle: &str) -> io::Result<usize>{
	if needle.is_empty(){
		return Err(io::Error::new (io::ErrorKind::InvalidInput, "Word Not Present"));
	}
	
	let finder = memchr::memmem::Finder::new(needle);
	let mut reader = BufReader::with_capacity(BUF_CAPACITY, hay);
	let mut buf = Vec::with_capacity(BUF_CAPACITY);
	let mut count = 0usize;
	
	loop {
		buf.clear();
		if reader.read_until(b'\n', &mut buf)? == 0{
			break;
		}
		
		count += finder.find_iter(&buf).count();
	}
	
	Ok(count)
}


