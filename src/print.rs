use crate::file_walker::RxaFile;

pub type DisplayItems = Vec<(u32, RxaFile)>;

pub fn print_grid(items: DisplayItems) {
    unimplemented!()
}

pub fn print_oneline(items: DisplayItems) {
    for item in items {
        println!("{}", item.1.print_name());
    }
}
