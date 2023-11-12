use anyhow::Result;
use console::Style;
use faccess::PathExt;
use std::{
    fmt::format,
    fs::{DirEntry, FileType, ReadDir},
    path::Path,
    path::PathBuf,
};

use crate::print::DisplayItems;

// Optimize later and put it into seperate file
#[derive(Debug)]
pub struct RxaFile {
    path: PathBuf,
    file_name: String,
    file_type: FileType,
}

impl RxaFile {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn print_detailed(&self) -> String {
        todo!()
    }

    pub fn print_name(&self) -> String {
        let mut style = Style::new();

        if self.file_type.is_dir() {
            style = style.blue().bold();
        }

        if self.path.executable() && self.file_type.is_file() {
            style = style.green();
        }

        if !self.path.readable() {
            style = style.red();
        }

        format!("{}", style.apply_to(self.file_name()))
    }
}

#[derive(Debug)]
pub struct FileWalker {
    directory: ReadDir,
}

impl FileWalker {
    pub fn new(path: &Path) -> Result<Self> {
        let directory = path.read_dir()?;

        Ok(Self { directory })
    }
}

impl Iterator for FileWalker {
    type Item = RxaFile;

    fn next(&mut self) -> Option<Self::Item> {
        let file = self.directory.next();

        match file {
            Some(file) => {
                if file.is_err() {
                    return None;
                }

                let file = file.unwrap();
                Some(RxaFile {
                    path: file.path(),
                    file_type: file.file_type().unwrap(),
                    file_name: file.file_name().into_string().unwrap(), // TODO : Handle this
                                                                        // properly
                })
            }
            None => None,
        }
    }
}

// TODO : Put this seperate file as utility
pub fn parse_list_file(filewalker: FileWalker, level: u32) -> DisplayItems {
    let mut temp = vec![];

    for item in filewalker {
        temp.push((level, item));
    }

    temp
}
