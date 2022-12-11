use std::{fs, collections::HashMap};

/// The goal of this problem is to find the index of the first character after a `start-of-packet` instruction.
/// A `start-of-packet` instruction is a sequence of n different characters.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_06.txt").expect("Unable to read the input file");

    // compute the answer to the first part
    println!(
        "The start-of-packet instruction is detected after {} characters when using a window of size 4.",
        find_first_start_of_packet(input.clone(), 4)
    );
    // compute the answer to the second part
    println!(
        "The start-of-packet instruction is detected after {} characters when using a window of size 14.",
        find_first_start_of_packet(input.clone(), 14)
    );
}

/// Finds the index of the first character after a `start-of-packet` instruction.
fn find_first_start_of_packet(input: String, window_size: usize) -> usize {
    // convert the input to a vector of chars
    let chars: Vec<char> = input.chars().collect();

    // create a hashmap to store the characters in the window
    let mut window_characters: HashMap<char, usize> = HashMap::new();

    // initialize the window by adding the first few characters
    for character_index in 0..window_size-1 {
        // get the character at the current index
        let char = chars[character_index];

        // if the character is already in the window, increment its count
        if window_characters.contains_key(&char) {
            *window_characters.get_mut(&char).unwrap() += 1;
        }
        // otherwise, add it to the window
        else {
            window_characters.insert(char, 1);
        }
    }

    // slide the window over the input
    for character_index in window_size-1..input.len() {
        // get the character at the current index
        let char = chars[character_index];

        // if the character is already in the window, increment its count
        if window_characters.contains_key(&char) {
            *window_characters.get_mut(&char).unwrap() += 1;
        }
        // otherwise, add it to the window
        else {
            window_characters.insert(char, 1);
        }

        // if the window contains only different characters,
        // then the number of different characters is equal to the window size
        if window_characters.len() == window_size {
            // if that's the case, return the index of the character following the last character in the window
            return character_index + 1;
        }
        // if we didn't find a solution, remove the first character from the window
        else {
            // find the character to remove
            let char_to_remove = chars[character_index + 1 - window_size];

            // if the character is only present once in the window, remove it
            if window_characters[&char_to_remove] == 1 {
                window_characters.remove(&char_to_remove);
            }
            // otherwise, decrement its count
            else {
                *window_characters.get_mut(&char_to_remove).unwrap() -= 1;
            }
        }
    }

    // if we didn't find a solution, panic (this should never happen)
    panic!("No solution found");
}
