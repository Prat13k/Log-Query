use rfd::FileDialog;
use std::io;
use std::fs::File;

pub fn choose_file() -> io::Result<Option<File>>{
	let Some(path) = FileDialog::new().pick_file() else {
		return Ok(None)
	};
	
	File::open(path).map(Some)
}

