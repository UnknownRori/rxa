use console::Style;
use term_grid::{self, Cell, Direction, Filling, Grid, GridOptions};
use terminal_size;

use crate::fs::file::{RItemDisplay, RxaFile};

pub type DisplayItems = Vec<RxaFile>;

pub fn print_grid(items: DisplayItems) {
    // TODO : Make proper grid, (Maybe custom implementation)
    let (terminal_size::Width(width), terminal_size::Height(height)) =
        terminal_size::terminal_size().expect(
            format!(
                "{}",
                Style::new()
                    .red()
                    .apply_to("[!] Cannot get width and height of your current terminal")
            )
            .as_str(),
        );

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(4),
        direction: Direction::LeftToRight,
    });

    for item in items {
        let display = match item {
            RxaFile::File(file) => file.display(),
            RxaFile::Directory(dir) => dir.display(),
        };

        let cell = Cell::from(display);
        grid.add(cell);
    }

    println!("{}:{}", width, height);
    println!("{}", grid.fit_into_width(width.into()).unwrap());
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
