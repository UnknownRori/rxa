use crate::rxa_file::{RxaFile, RxaItemDisplay};

pub type DisplayItems = Vec<RxaFile>;

pub fn print_grid(items: DisplayItems) {
    unimplemented!()
}

pub fn print_oneline(items: DisplayItems) {
    for item in items {
        let display = match item {
            RxaFile::File(file) => file.display(),
            RxaFile::Directory(dir) => dir.display(),
        };

        println!("{}", display);
    }
}
