use super::path::Path;
use regex::Regex;

/// Representation of an instruction.
/// An instruction is a command that can be executed on a filesystem.
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Changes the current directory of the filesystem.
    /// 
    /// The element of the tuple is the path to the new current directory.
    ChangeDirectory(Path),

    /// Lists the files and directories in the current directory.
    ListDirectory,
    
    /// Adds a file to the current directory.
    /// 
    /// The first element of the tuple is the size of the file.
    /// The second element of the tuple is the name of the file.
    CreateFile(usize, String),

    /// Adds a directory to the current directory.
    /// 
    /// The element of the tuple is the name of the directory.
    CreateDirectory(String),
}

/// Parses the given input into a list of instructions.
pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    // create the regexes to parse the instructions
    let cd_command_regex = Regex::new(r"\$ cd (.*)").unwrap();
    let ls_command_regex = Regex::new(r"\$ ls").unwrap();
    let ls_file_regex = Regex::new(r"(\d+) (.*)").unwrap();
    let ls_dir_regex = Regex::new(r"dir (.*)").unwrap();

    // create the vector to store the instructions
    let mut instructions = Vec::new();

    // for each line in the input
    for line in input.lines() {
        // if the line is a cd command
        if cd_command_regex.is_match(line) {
            // parse the path from the line
            let captures = cd_command_regex.captures(line).unwrap();
            let path = Path::try_from(&captures[1]).expect("Invalid path.");

            // add the instruction to the list
            instructions.push(Instruction::ChangeDirectory(path));
        }
        
        // if the line is a ls command
        else if ls_command_regex.is_match(line) {
            // add the instruction to the list
            instructions.push(Instruction::ListDirectory);
        }
        
        // if the line is a create file command
        else if ls_file_regex.is_match(line) {
            // parse the file size and name from the line
            let captures = ls_file_regex.captures(line).unwrap();
            let file_size: usize = captures[1].parse().unwrap();
            let file_name: String = captures[2].to_owned();
            
            // add the instruction to the list
            instructions.push(Instruction::CreateFile(file_size, file_name));
        }
        
        // if the line is a create directory command
        else if ls_dir_regex.is_match(line) {
            // parse the directory name from the line
            let captures = ls_dir_regex.captures(line).unwrap();
            let directory_name: String = captures[1].to_owned();

            // add the instruction to the list
            instructions.push(Instruction::CreateDirectory(directory_name));
        }
        
        // if the line is not a valid command
        else {
            // return an error
            return Err("Invalid instruction.".to_owned());
        }
    }

    // return the instructions
    Ok(instructions)
}
