use rfd::FileDialog;

fn choose_file() -> io::Result<File> {
	let some(path) = FileDialog::new().pick_file() else {
		return Ok(None);
	}
	File::open(path).map(Some)
}

