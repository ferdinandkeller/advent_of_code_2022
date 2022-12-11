use std::{fs, collections::HashSet};
use regex::Regex;
use thousands::Separable;

/// An instruction to move the rope.
/// The number inside each enum represents the distance to move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

/// A position in the grid.
/// The first element represents the x coordinate.
/// The second element represents the y coordinate.
/// The origin is at the center of the grid (0, 0).
/// Each element can be negative.
type Position = (i64, i64);

/// A rope that can be moved.
/// The rope is made of knots.
/// Each knot is a position in the grid.
/// The knots are connected.
/// 
/// # Fields
/// 
/// * `knots` - The list of knots that compose the rope.
/// * `visited_locations` - A set of the locations the tail of the rope has visited.
struct Rope {
    knots: Vec<Position>,
    visited_locations: HashSet<Position>,
}

impl Rope {
    /// Creates a new rope of a given size.
    /// Each knots is placed at the origin (0, 0).
    /// 
    /// # Arguments
    /// 
    /// * `size` - The number of knots in the rope.
    fn new(size: usize) -> Self {
        // create a set of visited locations
        let mut hash_set = HashSet::new();

        // add the origin to the set
        hash_set.insert((0, 0));

        // create the rope
        Self {
            knots: vec![(0, 0); size],
            visited_locations: hash_set,
        }
    }

    /// Moves the rope according to an instruction.
    /// 
    /// # Arguments
    /// 
    /// * `instruction` - The instruction to move the rope.
    fn move_rope(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Up(distance) => {
                for _ in 0..distance {
                    self.knots[0].1 += 1;
                    self.propagate();
                }
            }
            Instruction::Down(distance) => {
                for _ in 0..distance {
                    self.knots[0].1 -= 1;
                    self.propagate();
                }
            }
            Instruction::Left(distance) => {
                for _ in 0..distance {
                    self.knots[0].0 -= 1;
                    self.propagate();
                }
            }
            Instruction::Right(distance) => {
                for _ in 0..distance {
                    self.knots[0].0 += 1;
                    self.propagate();
                }
            }
        }
    }

    /// Propagates the movement of the rope after the first knots was moved.
    fn propagate(&mut self) {
        // The number of knots in the rope.
        let knots_count = self.knots.len();

        // Propagate the movement of the first knot to the other knots. The first knot was already moved manually, so we start at 1.
        for knot in 1..knots_count {
            // find the two currently connected knots
            let previous_knot = self.knots[knot - 1];
            let current_knot = self.knots[knot];

            // find the difference between the two knots
            let delta_x = previous_knot.0 - current_knot.0;
            let delta_y = previous_knot.1 - current_knot.1;

            // if the two knots are not connected, propagate the movement
            if delta_x.abs() > 1 || delta_y.abs() > 1 {
                let floor_delta_x = if delta_x == 2 { 1 } else if delta_x == -2 { -1 } else { delta_x };
                let floor_delta_y = if delta_y == 2 { 1 } else if delta_y == -2 { -1 } else { delta_y };

                self.knots[knot].0 += floor_delta_x;
                self.knots[knot].1 += floor_delta_y;
            }
            // if the two knots are connected, then we can stop propagating the movement
            else {
                break;
            }
        }

        // add the last knot position to the set of visited locations
        self.visited_locations.insert(self.knots[knots_count - 1]);
    }

    /// Returns the number of locations the tail of the rope has visited.
    /// 
    /// # Returns
    /// 
    /// The number of locations the tail of the rope has visited.
    fn visited_locations_count(&self) -> usize {
        self.visited_locations.len()
    }
}

/// Parses the instructions from a string.
/// 
/// # Arguments
/// 
/// * `input` - The string containing the instructions.
/// 
/// # Returns
/// 
/// A vector of instructions.
fn parse_instructions(input: &str) -> Vec<Instruction> {
    // create a vector of instructions
    let mut instructions: Vec<Instruction> = vec![];

    // create a regex to parse the instructions
    let instruction_regex = Regex::new(r"([URDL]) (\d+)").expect("Unable to compile the regex.");
    
    // for each line in the input
    for instruction_str in input.lines() {
        // parse the instruction using the regex
        let captures = instruction_regex
            .captures(instruction_str)
            .expect("Unable to parse the direction string.");

        // get the direction and the distance
        let direction: &str = captures
            .get(1)
            .expect("Unable to get the direction.")
            .as_str();
        let distance: usize = captures
            .get(2)
            .expect("Unable to get the distance.")
            .as_str()
            .parse()
            .expect("Unable to parse the distance.");

        // create the instruction
        let instruction = match direction {
            "U" => Instruction::Up(distance),
            "D" => Instruction::Down(distance),
            "L" => Instruction::Left(distance),
            "R" => Instruction::Right(distance),
            _ => panic!("Unknown direction."),
        };

        // add the instruction to the vector
        instructions.push(instruction);
    }

    // return the vector of instructions
    return instructions;
}

/// The goal of this challenge is to find the number of locations the tail of the rope has visited.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_9.txt").expect("Unable to read the input file.");

    // parse the instructions
    let instructions = parse_instructions(&input);

    // create the ropes
    let mut rope_length_2 = Rope::new(2);
    let mut rope_length_10 = Rope::new(10);

    // move the ropes
    for instruction in instructions {
        rope_length_2.move_rope(instruction);
        rope_length_10.move_rope(instruction);
    }

    // print the results
    println!(
        "The tail of the rope of length 2 visited {visited_locations_count} locations.",
        visited_locations_count = rope_length_2.visited_locations_count().separate_with_commas()
    );

    println!(
        "The tail of the rope of length 10 visited {visited_locations_count} locations.",
        visited_locations_count = rope_length_10.visited_locations_count().separate_with_commas()
    );
}
