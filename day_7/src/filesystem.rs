use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::file::File;
use crate::directory::Directory;
use crate::path::Path;
use crate::instruction::Instruction;

/// Represents the filesystem.
/// Holds all the folders, as well as their content
pub struct FileSystem {
    root_directory: Rc<RefCell<Directory>>,
    current_directory: Rc<RefCell<Directory>>,
}

impl FileSystem {
    /// Creates a new empty filesystem, with a root directory.
    pub fn new() -> Self {
        let root_directory = Rc::new(RefCell::new(Directory {
            name: String::from("/"),
            files: HashMap::new(),
            sub_directories: HashMap::new(),
            parent_directory: None,
        }));

        Self {
            root_directory: root_directory.clone(),
            current_directory: root_directory.clone(),
        }
    }

    /// Tries to move in the current filesystem using the given path.
    pub fn move_using_path(&mut self, path: Path) {
        match path {
            Path::Relative(_) => {
                self.move_using_relative_path(path);
            },
            Path::Absolute(_) => {
                self.current_directory = self.root_directory.clone();
                self.move_using_relative_path(path.to_relative());
            },
        }
    }
    
    /// Tries to move in the current filesystem using the given relative path.
    pub fn move_using_relative_path(&mut self, path: Path) {
        match path {
            Path::Absolute(_) => panic!("Cannot move using absolute path."),
            Path::Relative(directories) => {
                // for each directory in the path
                for directory in directories {
                    // if the directory is "..", move up
                    if directory == ".." {
                        // try to get the parent directory
                        let maybe_parent_dir = self.current_directory.borrow()
                            .parent_directory
                            .clone();
                        
                        // if the parent directory exists, move to it
                        if let Some(parent_dir) = maybe_parent_dir {
                            self.current_directory = parent_dir;
                        }
                        // else, we are already at the root directory, so we cannot move up
                        else {
                            panic!("Cannot move up from root directory.");
                        }
                    }
                    // else, move to the sub directory
                    else {
                        // try to get the sub directory
                        let maybe_sub_dir = self.current_directory.borrow()
                            .sub_directories.get(&directory)
                            .map_or(None, |e| Some(e.clone()));

                        // if the sub directory exists, move to it
                        if let Some(sub_dir) = maybe_sub_dir {
                            self.current_directory = sub_dir.clone();
                        }
                        // else, the directory does not exist, so we cannot move to it
                        else {
                            panic!("Cannot move to directory {directory} because it does not exist.");
                        }
                    }
                }
            }
        }
    }

    /// Executes the given instruction.
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // if the instruction is to change directory, move to the given path
            Instruction::ChangeDirectory(path) => {
                self.move_using_path(path);
            },
            // if the instruction is to list the directory, do nothing
            Instruction::ListDirectory => {
                // do nothing
            },
            // if the instruction is to add a file, add it to the current directory
            Instruction::CreateFile(file_size, file_name) => {
                let new_file = File {
                    name: file_name.clone(),
                    size: file_size,
                    parent_directory: self.current_directory.clone(),
                };

                self.current_directory.borrow_mut().files.insert(file_name.clone(), new_file);
            },
            // if the instruction is to add a directory, add it to the current directory
            Instruction::CreateDirectory(directory_name) => {
                let new_directory = Directory {
                    name: directory_name.clone(),
                    files: HashMap::new(),
                    sub_directories: HashMap::new(),
                    parent_directory: Some(self.current_directory.clone()),
                };

                self.current_directory.borrow_mut().sub_directories.insert(
                    directory_name.clone(),
                    Rc::new(RefCell::new(new_directory))
                );
            },
        }
    }

    /// Computes the size of the filesystem.
    pub fn size(&self) -> usize {
        self.root_directory.borrow().size()
    }

    /// Computes the small-size of the filesystem.
    pub fn small_size(&self) -> usize {
        self.root_directory.borrow().small_size()
    }

    /// Finds the smallest folder we can delete to free up the given amount of bytes.
    pub fn big_enough_for_delete(&self, bytes_to_delete: usize) -> Option<usize> {
        self.root_directory.borrow().big_enough_for_delete(bytes_to_delete)
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_directory.borrow())
    }
}
