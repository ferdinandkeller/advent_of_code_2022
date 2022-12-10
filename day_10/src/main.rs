use std::fs;
use regex::Regex;
use thousands::Separable;

/// An instruction for the crt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    NoOp,
    AddX(i64)
}

/// Parses the instructions from the input.
fn parse_instructions(input: &str) -> Vec<Instruction> {
    // create a vector of instructions
    let mut instructions = vec![];

    // create the regexes for parsing the instructions
    let noop_regex = Regex::new(r"noop").unwrap();
    let addx_regex = Regex::new(r"addx (-?\d+)").unwrap();

    // for each line in the input
    for line in input.lines() {
        // if the line matches the noop regex
        if noop_regex.is_match(line) {
            // add the instruction to the vector
            instructions.push(Instruction::NoOp);
        }

        // if the line matches the addx regex
        else if let Some(captures) = addx_regex.captures(line) {
            // parse the value
            let add_x_value: i64 = captures[1].parse().unwrap();
            // add the instruction to the vector
            instructions.push(Instruction::AddX(add_x_value));
        }

        // if the line doesn't match any regex
        else {
            // throw an error
            panic!("Unknown instruction: {}", line);
        }
    }

    instructions
}

/// The goal of this challenge is to find the total signal strength of the signal, and fidn what is dis
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_10.txt").expect("Unable to read the input file");

    // parse the instructions
    let instructions = parse_instructions(&input);

    // create variables to store the state of the crt
    let mut reg_x: i64 = 1; // value of the x register
    let mut cycle: usize = 0; // current cycle
    let mut next_log: usize = 20; // next cycle to log
    let mut total_signal_strength: i64 = 0; // total signal strength
    let mut crt_screen = vec![vec![false; 40]; 6]; // values displayed on the crt

    // for each instruction
    for instruction in instructions {
        match instruction {
            Instruction::NoOp => {
                // find the current x and y coordinates of the pixel
                let pixel_x = (cycle % 40) as i64;
                let pixel_y = (cycle / 40) as i64;

                // if the current pixel is in the sprite, then it is on
                if reg_x - 1 <= pixel_x && pixel_x <= reg_x + 1 {
                    crt_screen[pixel_y as usize][pixel_x as usize] = true;
                }

                // increment the cycle
                cycle += 1;

                // if needed, log the signal strength
                if cycle >= next_log {
                    total_signal_strength += reg_x * next_log as i64;
                    next_log += 40;
                }
            },
            Instruction::AddX(x) => {
                // find the current x and y coordinates of the pixel
                let pixel_x = (cycle % 40) as i64;
                let pixel_y = (cycle / 40) as i64;

                // if the current pixel is in the sprite, then it is on
                if reg_x - 1 <= pixel_x && pixel_x <= reg_x + 1 {
                    crt_screen[pixel_y as usize][pixel_x as usize] = true;
                }

                // increment the cycle
                cycle += 1;
                
                // find the current x and y coordinates of the pixel
                let pixel_x = (cycle % 40) as i64;
                let pixel_y = (cycle / 40) as i64;

                // if the current pixel is in the sprite, then it is on
                if reg_x - 1 <= pixel_x && pixel_x <= reg_x + 1 {
                    crt_screen[pixel_y as usize][pixel_x as usize] = true;
                }

                // increment the cycle
                cycle += 1;
                
                // if needed, log the signal strength
                if cycle >= next_log {
                    total_signal_strength += reg_x * next_log as i64;
                    next_log += 40;
                }

                // increment the x register
                reg_x += x;
            }
        }
    }

    // print the total signal strength of the signal
    println!(
        "Total signal strength: {total_signal_strength}.",
        total_signal_strength = total_signal_strength.separate_with_commas()
    );

    // convert the crt screen to a string and print it
    let mut screen = String::new();
    for (line_index, line) in crt_screen.iter().enumerate() {
        for &pixel in line {
            if pixel {
                screen.push('█');
            } else {
                screen.push(' ');
            }
        }
        if line_index != crt_screen.len() - 1 {
            screen.push('\n');
        }
    }
    println!("\nCRT screen:\n{screen}");
}
