use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use thousands::Separable;
use crate::directory::Directory;

/// Represents a file in the filesystem.
/// 
/// # Fields
/// 
/// * `name` - The name of the file.
/// * `size` - The size of the file in bytes.
/// * `parent_directory` - A reference to the parent directory of the file.
#[derive(Clone)]
pub struct File {
    pub name: String,
    pub size: usize,
    pub parent_directory: Rc<RefCell<Directory>>,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {file_name} (file, size={file_size})", file_name=self.name, file_size=self.size.separate_with_commas())
    }
}
