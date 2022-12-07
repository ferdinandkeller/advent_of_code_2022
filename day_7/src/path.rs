use std::{convert::TryFrom, fmt::Display};

/// Representation of a path.
/// A path can be either absolute or relative. Absolute paths start with a `/` and are relative to the root directory.
/// The array of [`String`] contains the names of the directories in the path.
#[derive(Debug, Clone)]
pub enum Path {
    Relative(Vec<String>),
    Absolute(Vec<String>),
}

impl Path {
    /// Converts an absolute path to a relative path.
    pub fn to_relative(self) -> Path {
        match self {
            Path::Relative(_) => self,
            Path::Absolute(directories) => Path::Relative(directories),
        }
    }
}

impl TryFrom<&str> for Path {
    type Error = String;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let mut path_parts: Vec<&str> = path.split("/").collect();

        path_parts.retain(|p| !p.is_empty());
    
        if path.starts_with("/") {
            Ok(Path::Absolute(path_parts.iter().map(|p| p.to_string()).collect()))
        } else {
            Ok(Path::Relative(path_parts.iter().map(|p| p.to_string()).collect()))
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // if the path is relative, we don't want to print the leading `/`
            Path::Relative(directories) => {
                // create a string to hold the path
                let mut path = String::new();

                // iterate over the directories in the path, and add them to the path string
                for directory in directories {
                    path.push_str(&format!("{}/", directory));
                }

                // write the path string to the formatter
                write!(f, "{}", path)
            },

            // if the path is absolute, we want to print the leading `/`
            Path::Absolute(directories) => {
                // create a string to hold the path
                let mut path = String::new();
                
                // iterate over the directories in the path, and add them to the path string
                for directory in directories {
                    path.push_str(&format!("{}/", directory));
                }

                // write the path string to the formatter
                write!(f, "/{}", path)
            },
        }
    }
}
