pub mod path;
pub mod file;
pub mod directory;
pub mod filesystem;
pub mod instruction;

use std::fs;
use filesystem::FileSystem;
use instruction::parse_instructions;
use thousands::Separable;

/// The goal of this challenge is basically to create a filesystem.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_07.txt").expect("Unable to read the input file");

    // parse the input into a vector of instructions
    let instructions = parse_instructions(&input).unwrap();

    // create a new filesystem
    let mut filesystem = FileSystem::new();

    // execute each instruction
    for instruction in instructions {
        filesystem.execute(instruction);
    }

    // print a representation of the filesystem
    println!("{filesystem}");

    // print the small-size of the filesystem
    println!(
        "Small-size of the filesystem: {small_size} bytes",
        small_size = filesystem.small_size().separate_with_commas()
    );

    // print the size of the smallest directory that can free 30MB of space
    let disk_size: usize = 70_000_000;
    let used_space = filesystem.size();
    let unused_space = disk_size - used_space;
    let space_needed_for_update: usize = 30_000_000;
    let space_to_clear = space_needed_for_update - unused_space;

    println!(
        "The size of the smallest directory we can clear to free a total of 3MB is: {big_enough} bytes",
        big_enough = filesystem.big_enough_for_delete(space_to_clear).unwrap().separate_with_commas()
    );
}
