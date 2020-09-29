use zip::ZipArchive;
use std::io::{Cursor, Read};

pub struct ZipFS {
	bytes: ZipArchive<Cursor<Vec<u8>>>
}

impl ZipFS {
	pub fn new(bytes: Vec<u8>) -> Self {
		Self {
			bytes: ZipArchive::new(Cursor::new(bytes)).expect("Error loading zip")
		}
	}
}

impl crate::FsAccess for ZipFS {
    fn get_file(&mut self, path: &str) -> Option<String> {
        self.bytes.by_name(path).ok().map(|mut x| {let mut s = String::new(); x.read_to_string(&mut s).expect("Error reading file"); s})
    }
}