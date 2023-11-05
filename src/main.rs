use clap::Parser;
use rxa::Args;

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
    println!("\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
                "https://google.com", "Test");
}
