mod unzipped;
mod zip;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_export]
macro_rules! fs {
    ($zip:expr, $base_path:expr) => {
        if cfg!(debug_assertions){
            use binfs::load_unzipped;
            load_unzipped($base_path)
        } else {
            use binfs::load_zip;
            load_zip(Vec::from(include_bytes!($zip)))
        }
    };
}

pub fn load_zip(b: Vec<u8>) -> Fs {
    Fs::new(Box::new(zip::ZipFS::new(b)))
}

use std::path::Path;
pub fn load_unzipped<P: AsRef<Path>>(b: P) -> Fs {
    Fs::new(Box::new(unzipped::UnzippedFS::new(b.as_ref().to_path_buf())))
}

pub trait FsAccess {
    fn get_file(&mut self, path: &str) -> Option<String>;
}

pub struct Fs {
    inner: Box<dyn FsAccess>,
}

impl Fs {
    fn new(inner: Box<dyn FsAccess>) -> Self {
        Self { inner }
    }
}


impl FsAccess for Fs {
    fn get_file(&mut self, path: &str) -> Option<String> {
        self.inner.get_file(path)
    }
}