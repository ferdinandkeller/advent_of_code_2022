use std::fs;
use regex::Regex;

// The goal of this challenge is to find the state of the stacks after the instructions have been executed.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_5.txt").expect("Unable to read the input file");

    // split the input into two parts: the representation of the stacks and the instructions
    let split: Vec<&str> = input.split("\n\n").collect();
    let stacks_representation = split[0];
    let instructions = split[1];



    // count the number of stacks present in the input
    let stack_count = (stacks_representation.lines().next().unwrap().len() + 1) / 4;

    // create a variable to store the state of the stacks
    let mut stacks_initial_state: Vec<Vec<char>> = vec![vec![]; stack_count];

    // parse the stacks representation and store the state of the stacks in the variable
    for row in stacks_representation.lines().rev().skip(1) {
        // convert the current row into a vector of chars
        let chars: Vec<char> = row.chars().collect();

        // for each stack, push a new crate if there is one
        for stack_index in 0..stack_count {
            let c = chars[4 * stack_index + 1];
            if c != ' ' {
                stacks_initial_state[stack_index].push(c);
            }
        }
    }



    // create variables to store the changing state of the stacks depending on the CrateMover version
    let mut stacks_cm9k = stacks_initial_state.clone();
    let mut stacks_cm9k1 = stacks_initial_state.clone();

    // create a regex to parse the instructions
    let instruction_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // for each instruction in the input
    for instruction in instructions.lines() {
        // parse the instruction
        let instruction_captures = instruction_regex.captures(instruction).unwrap();
        let quantity: usize = instruction_captures[1].parse().unwrap();
        let origin_stack_index: usize = instruction_captures[2].parse::<usize>().unwrap() - 1;
        let destination_stack_index: usize = instruction_captures[3].parse::<usize>().unwrap() - 1;

        // method 1: CrateMover 9000
        // we take the top n crates from the origin stack, reverse their order, and push them to the destination stack
        let origin_stack_size = stacks_cm9k[origin_stack_index].len();
        let tmp_stack: Vec<char> = stacks_cm9k[origin_stack_index][origin_stack_size - quantity..].iter().rev().copied().collect();
        stacks_cm9k[origin_stack_index].truncate(origin_stack_size - quantity);
        stacks_cm9k[destination_stack_index].extend(tmp_stack);

        // method 2: CrateMover 9001
        // we take the top n crates from the origin stack, keep their order, and push them to the destination stack
        let origin_stack_size = stacks_cm9k1[origin_stack_index].len();
        let tmp_stack: Vec<char> = stacks_cm9k1[origin_stack_index][origin_stack_size - quantity..].iter().copied().collect();
        stacks_cm9k1[origin_stack_index].truncate(origin_stack_size - quantity);
        stacks_cm9k1[destination_stack_index].extend(tmp_stack);
    }



    // find the top crate of each stack for CrateMover 9000
    let mut top_crates_cm9k = vec![];
    for stack in stacks_cm9k {
        top_crates_cm9k.push(stack[stack.len() - 1]);
    }
    let top_crates_cm9k: String = top_crates_cm9k.into_iter().collect();

    // find the top crate of each stack for CrateMover 9001
    let mut top_crates_cm9k1 = vec![];
    for stack in stacks_cm9k1 {
        top_crates_cm9k1.push(stack[stack.len() - 1]);
    }
    let top_crates_cm9k1: String = top_crates_cm9k1.into_iter().collect();



    // print the results
    println!("Using the CrateMove 9000, the top crates are: {top_crates_cm9k}");
    println!("Using the CrateMove 9001, the top crates are: {top_crates_cm9k1}");
}
