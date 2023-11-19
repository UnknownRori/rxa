use std::path::Path;

use crate::fs::file_walker::{parse_list_file, FileWalker};
use crate::print::{print_grid, print_oneline};

use super::display_options::DisplayOptionEnum;
use super::Args;

use anyhow::Result;

pub struct App {
    args: Args,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn run(&self) -> Result<()> {
        let display_option = DisplayOptionEnum::from(&self.args.display_options);
        let current_dir = Path::new(&self.args.dir);

        let filewalker = FileWalker::new(&current_dir)?;
        let current_dir_data = parse_list_file(filewalker);

        match display_option {
            DisplayOptionEnum::Long => todo!(),
            DisplayOptionEnum::OneLine => print_oneline(current_dir_data),
            DisplayOptionEnum::Grid => print_grid(current_dir_data),
            DisplayOptionEnum::Tree => todo!(),
        };

        Ok(())
    }
}
