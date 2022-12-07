use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, format};
use thousands::Separable;
use crate::file::File;

/// Represents a directory in the filesystem.
/// A directory is a collection of files and sub-directories.
/// 
/// # Fields
/// 
/// * `name` - The name of the directory.
/// * `files` - A map of the files in the directory.
/// * `sub_directories` - A map of references to the sub-directories in the directory.
/// * `parent_directory` - A reference to the parent directory of the directory.
#[derive(Clone)]
pub struct Directory {
    pub name: String,
    pub files: HashMap<String, File>,
    pub sub_directories: HashMap<String, Rc<RefCell<Directory>>>,
    pub parent_directory: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    /// The size of the directory in bytes.
    pub fn size(&self) -> usize {
        // compute the size of all the files in the directory
        let files_size: usize = self.files.values().map(|f| f.size).sum();

        // compute the size of all the sub-directories in the directory
        let sub_directories_size: usize = self.sub_directories.values().map(|d| d.borrow().size()).sum();

        // the size of the directory is the sum of the size of all the files and sub-directories
        files_size + sub_directories_size
    }

    /// The size of the directory in bytes, but only taking into account the sub-directories that are less than or equal to 100,000 bytes.
    pub fn small_size(&self) -> usize {
        // compute the size of the current directory
        let current_directory_size: usize = self.size();

        // get the small-size of all the sub-directories in the directory
        let sub_directories_small_size: usize = self.sub_directories.values().map(|d| d.borrow().small_size()).sum();

        // if the size of the current directory is less than or equal to 100,000 bytes,
        // then the small-size of the directory is the sum of the small-size of all the sub-directories and the size of the current directory
        if current_directory_size <= 100_000 {
            return sub_directories_small_size + current_directory_size;
        }
        // else it only takes into account the small-size of all the sub-directories
        else {
            return sub_directories_small_size;
        }
    }
    
    // find the size of the smallest directory that is big enough to delete the given number of bytes
    pub fn big_enough_for_delete(&self, bytes_to_delete: usize) -> Option<usize> {
        // get the size of the directory
        let directory_size = self.size();

        // check if the directory is too small to delete the given number of bytes
        if directory_size < bytes_to_delete {
            return None;
        }

        // if the size of the directory is big enough to delete the given number of bytes,
        // verify that there is not a smaller directory that is big enough to delete the given number of bytes
        self.sub_directories
            // for each sub-directory
            .values()
            // compute if it is big enoug to delete the given number of bytes
            .map(|d| d.borrow().big_enough_for_delete(bytes_to_delete))
            // keep the smallest directory that is big enough to delete the given number of bytes
            .fold(Some(directory_size), |acc, dir| {
                match (acc, dir) {
                    (Some(acc_size), Some(dir_size)) => {
                        if dir_size < acc_size {
                            return dir;
                        } else {
                            return acc;
                        }
                    },
                    (Some(_), None) => {
                        return acc;
                    },
                    _ => unreachable!()
                }
            })
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // create a string that represents the displayed directory
        let mut displayed = String::new();

        // write the name of the directory and its size
        displayed.push_str(&format!(
            "- {directory_name} (dir, size={directory_size})\n",
            directory_name=self.name,
            directory_size=self.size().separate_with_commas()
        ));

        // write the name of the files and their size
        for file in self.files.values() {
            displayed.push_str(&format!("  {}\n", file));
        }
        
        // write the name of the sub-directories and their size, as well as their content
        for directory in self.sub_directories.values() {
            // indent the content of the sub-directory
            let indented = directory.borrow()
                .to_string().lines()
                .map(|l| format!("  {}", l)).
                collect::<Vec<String>>()
                .join("\n");
            
            // add the indented sub-directory to the displayed directory
            displayed.push_str(&format!("{indented}\n"));
        }

        // write the displayed directory to the formatter
        write!(f, "{displayed}")
    }
}
