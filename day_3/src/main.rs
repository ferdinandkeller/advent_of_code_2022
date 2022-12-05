use std::{fs, collections::HashSet};

/// Converts a char representing an item to a priority
/// 
/// # Arguments
/// 
/// * `item` - The item to convert
/// 
/// # Priorities
/// 
/// a-z: 1-26
/// A-Z: 27-52
fn item_to_priority(item: char) -> usize {
    match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Invalid item: {}", item),
    }
}

/// The goal of this challenge is to find the items that are in multiple compartments or rucksacks.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_3.txt").expect("Unable to read the input file");

    // create a variable to store the sum of the priorities of the items
    let mut sum_of_priorities_1 = 0;

    // for each rucksack
    for rucksack in input.lines() {
        // create a set to store the items in the rucksack
        let mut items = HashSet::new();

        // the rucksack has two compartments of equal size
        // compute the size of each compartment
        let compartment_size = rucksack.len() / 2;
        
        // add the items of the first compartment to the set
        for item in rucksack[0..compartment_size].chars() {
            items.insert(item);
        }

        // look for the first item of the second compartment that is already in the set
        for item in rucksack[compartment_size..].chars() {
            if items.contains(&item) {
                // compute the priority of the item
                let item_priority = item_to_priority(item);

                // add the priority to the sum
                sum_of_priorities_1 += item_priority;

                // stop looking for items, as we don't want to count doubloons
                break;
            }
        }
    }

    // display the computed sum of the priorities
    println!("Sum of the priorities of items in both compartments: {sum_of_priorities_1}");

    // create a variable to store the sum of the priorities of the items
    let mut sum_of_priorities_2 = 0;

    // for each rucksack in the same group
    for [rucksack_1, rucksack_2, rucksack_3] in input.lines().group() {
        // create a set to store the items in the first rucksack
        let mut rucksack_1_items = HashSet::new();
        // create a set to store the items in the second rucksack
        let mut rucksack_2_items = HashSet::new();

        // add the items of the first rucksack to the first set
        for item in rucksack_1.chars() {
            rucksack_1_items.insert(item);
        }
        // add the items of the second rucksack to the second set if they already exist in the first set
        for item in rucksack_2.chars() {
            if rucksack_1_items.contains(&item) {
                rucksack_2_items.insert(item);
            }
        }
        // find the item in the third rucksack that is already in the previous two sets
        for item in rucksack_3.chars() {
            if rucksack_2_items.contains(&item) {
                // compute the priority of the item
                let item_priority = item_to_priority(item);

                // add the priority to the sum
                sum_of_priorities_2 += item_priority;

                // stop looking for items
                break;
            }
        }
    }

    // display the computed sum of the priorities
    println!("Sum of the priorities of the badges: {sum_of_priorities_2}");
}

/// Special iterator that packs the output of another iterator into groups of 3 elements.
struct GroupIter<I: Iterator> {
    iter: I
}

impl<I: Iterator> Iterator for GroupIter<I> {
    type Item = [I::Item; 3];

    fn next(&mut self) -> Option<Self::Item> {
        Some([self.iter.next()?, self.iter.next()?, self.iter.next()?])
    }
}

trait GroupIterTrait<I: Iterator> {
    fn group(self) -> GroupIter<I>;
}

impl<I: Iterator> GroupIterTrait<I> for I {
    fn group(self) -> GroupIter<Self> {
        GroupIter { iter: self }
    }
}
