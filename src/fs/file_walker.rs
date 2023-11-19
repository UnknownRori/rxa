use anyhow::Result;
use std::{fs::ReadDir, path::Path};

use crate::print::DisplayItems;

use super::file::{RDirectory, RFile, RxaFile};

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
                if file.path().is_dir() {
                    return Some(RxaFile::Directory(RDirectory {
                        path: file.path(),
                        dir_type: file.file_type().unwrap(),
                        child: vec![],
                    }));
                }

                return Some(RxaFile::File(RFile {
                    path: file.path(),
                    file_type: file.file_type().unwrap(),
                }));
            }
            None => None,
        }
    }
}

// TODO : Put this seperate file as utility
pub fn parse_list_file(filewalker: FileWalker) -> DisplayItems {
    let mut temp = vec![];

    for item in filewalker {
        temp.push(item);
    }

    temp
}
