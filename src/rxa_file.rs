use std::{fs::FileType, path::PathBuf};

use console::Style;
use faccess::PathExt;

pub trait RxaItemDisplay {
    fn display(&self) -> String;
    fn display_detailed(&self) -> String;
}

// TODO : Properly put custom file type struct
// Also add identifier for dotfiles and file extension
#[derive(Debug)]
pub struct RFile {
    pub path: PathBuf,
    pub file_type: FileType,
}

impl RxaItemDisplay for RFile {
    fn display(&self) -> String {
        let mut style = Style::new();
        let filename = self.path.file_name().unwrap().to_string_lossy().to_string();

        // TODO :  Extract this into different function
        if self.path.executable() && self.path.is_file() {
            style = style.green();
        }

        if !self.path.readable() {
            style = style.red();
        }

        format!("{}", style.apply_to(filename))
    }

    fn display_detailed(&self) -> String {
        todo!()
    }
}

// TODO : Propley put custom file type struct
// Also add identifier for dotfiles and file extension
#[derive(Debug)]
pub struct RDirectory {
    pub path: PathBuf,
    pub dir_type: FileType,
    pub child: Vec<RxaFile>,
}

impl RxaItemDisplay for RDirectory {
    fn display(&self) -> String {
        let mut style = Style::new().blue();
        let filename = self.path.file_name().unwrap().to_string_lossy().to_string();

        format!("{}", style.apply_to(filename))
    }

    fn display_detailed(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum RxaFile {
    Directory(RDirectory),
    File(RFile),
}
