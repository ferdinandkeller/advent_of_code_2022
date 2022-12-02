use std::fs;

/// A struct to store how many calories an elf is carrying.
/// 
/// # Fields
/// 
/// * `elf` - The index of the elf (it starts at 1).
/// * `calories` - The number of calories the elf is carrying.
#[derive(Debug, Clone, Copy)]
struct ElfCalories {
    elf: usize,
    calories: usize,
}

/// Our goal is to find which elves are carrying the most calories.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_1.txt").expect("Unable to read file");

    // create a vector of ElfCalories to store the parsed data
    let mut elves_calories = Vec::new();

    // create two variable that will store the data about the current elf
    let mut current_elf = ElfCalories { elf: 1, calories: 0 };

    // parse the data
    for line in input.lines() {
        if !line.is_empty() {
            current_elf.calories += line.parse::<usize>().unwrap();
        } else {
            elves_calories.push(current_elf);
            current_elf.elf += 1;
            current_elf.calories = 0;
        }
    }
    elves_calories.push(current_elf);

    // sort the elves by the number of calories they are carrying
    elves_calories.sort_by(|e1, e2| e2.calories.cmp(&e1.calories));

    // find the three elf with the most calories
    let first_elf = elves_calories[0];
    let second_elf = elves_calories[1];
    let third_elf = elves_calories[2];

    // display the first elf
    println!(
        "The {first}th elf has the most calories: {first_elf_calories}",
        first = first_elf.elf,
        first_elf_calories = first_elf.calories
    );

    // display the total of top three elves
    println!(
        "The top three elves are elf {first}, elf {second}, and elf {third}, with a total of {total} calories.",
        first = first_elf.elf,
        second = second_elf.elf,
        third = third_elf.elf,
        total = first_elf.calories + second_elf.calories + third_elf.calories
    )
}
