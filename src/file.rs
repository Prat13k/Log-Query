use rfd::FileDialog;

pub fn choose_file() -> io::Result<Option<File>>{
	let Some(path) = FileDialog::new().pick_file() else {
		return Ok(None);
	}
	File::open(path).map(Some)
}

