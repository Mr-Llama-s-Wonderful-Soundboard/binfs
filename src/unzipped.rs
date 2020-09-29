use std::path::PathBuf;
use std::fs::read_to_string;

pub struct UnzippedFS {
	base_path: PathBuf
}

impl UnzippedFS {
	pub fn new(base_path: PathBuf) -> Self {
		Self {
			base_path
		}
	}
}

impl crate::FsAccess for UnzippedFS {
    fn get_file(&mut self, path: &str) -> Option<String> {
		let mut p = self.base_path.clone();
		p.push(path);
		read_to_string(p).ok()
    }
}