use std::{fs, collections::VecDeque};
use thousands::Separable;
use regex::Regex;

/// Represents how the worry-level changes after a monkey has inspected an item.
#[derive(Debug, Clone, Copy)]
enum Operation {
    /// The worry-level is increased by a fixed amount.
    Add(u64),
    /// The worry-level is multiplied by a fixed amount.
    Multiply(u64),
    /// The worry-level is squared.
    Square,
}

/// Represents a monkey.
/// 
/// # Attributes
/// 
/// * `items_worry_levels` - The worry-levels of the items that the monkey possesses.
/// * `operation` - The operation to perform on the worry-levels of the items when the monkey inspects them.
/// * `divisible_test` - If the worry-level of an item is divisible by this number, the monkey will throw the item to another monkey.
/// * `test_results` - The indices of the monkeys that the item will be thrown to if the test succeeds or fails.
/// * `inspected_items_count` - The number of items that the monkey has inspected.
#[derive(Debug, Clone)]
struct Monkey {
    items_worry_levels: VecDeque<u64>,
    operation: Operation,
    divisible_test: u64,
    test_results: (usize, usize),
    inspected_items_count: u64
}

/// Parses the monkeys from the input string.
fn parse_monkeys(input: &str) -> Vec<Monkey> {
    // create a vector to store the monkeys in
    let mut monkeys = vec![];

    // create a regex to match a monkey string
    let monkey_regex = Regex::new(r"Monkey \d+:\n  Starting items: ?(\d+(, )?)*\n  Operation: new = old [+*] (\d+|old)\n  Test: divisible by \d+\n    If true: throw to monkey \d+\n    If false: throw to monkey \d+").unwrap();

    // iterate over all the matches in the input string
    for regex_match in monkey_regex.find_iter(input) {
        // get the monkey string
        let monkey_str = &input[regex_match.start()..regex_match.end()];

        // split the string into lines
        let lines: Vec<&str> = monkey_str.lines().collect();

        // parse the starting items
        let items_worry_levels: VecDeque<u64> = lines[1]
            .split("Starting items: ").collect::<Vec<_>>()[1]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        // parse the operation
        let operations_split: Vec<&str> = lines[2]
            .split("Operation: new = old ").collect::<Vec<_>>()[1]
            .split(" ").collect();
        let operations = match operations_split[0] {
            "+" => Operation::Add(operations_split[1].parse().unwrap()),
            "*" => {
                if operations_split[1] == "old" {
                    Operation::Square
                } else {
                    Operation::Multiply(operations_split[1].parse().unwrap())
                }
            },
            _ => panic!("Unknown operation"),
        };
        // parse the divisible test
        let divisible: u64 = lines[3]
            .split("Test: divisible by ").collect::<Vec<_>>()[1]
            .parse().unwrap();
        // parse the test results
        let test_result_success: usize = lines[4]
            .split("If true: throw to monkey ").collect::<Vec<_>>()[1]
            .parse().unwrap();
        let test_result_fail: usize = lines[5]
            .split("If false: throw to monkey ").collect::<Vec<_>>()[1]
            .parse().unwrap();

        // create the monkey
        let monkey = Monkey {
            items_worry_levels,
            operation: operations,
            divisible_test: divisible,
            test_results: (test_result_success, test_result_fail),
            inspected_items_count: 0,
        };

        // add the monkey to the vector
        monkeys.push(monkey);
    }

    // return the vector of monkeys
    monkeys
}

/// Runs a simulation of the monkeys.
/// 
/// # Arguments
/// 
/// * `monkeys` - The  monkeys to run the simulation on.
/// * `number_of_rounds` - The number of rounds to run the simulation for.
/// * `relief_mode_activated` - Whether or not relief mode is activated. When relief mode is activated, the worry-levels of the items are divided by 3 before being tested.
fn simulation(monkeys: &mut Vec<Monkey>, number_of_rounds: u64, relief_mode_activated: bool) {
    // get the number of monkeys
    let monkey_count = monkeys.len();

    // get the lowest common multiple of the monkeys' divisible tests
    // normally when computing the LCM, you would use the prime factorization of each number
    // and then multiply the highest power of each prime together
    // but since we know that the numbers are all distinct primes in our input, we can just multiply them all together
    let lowest_common_multiple: u64 = monkeys.iter().map(|m| m.divisible_test).product();

    // run the simulation for the specified number of rounds
    for _ in 0..number_of_rounds {

        // iterate over all the monkeys
        for current_monkey_index in 0..monkey_count {
            // get the current monkey out of the vector
            let mut current_monkey = monkeys[current_monkey_index].clone();
    
            // while the current monkey has items to inspect
            while !current_monkey.items_worry_levels.is_empty() {
                // get the worry-level of the item the monkey is inspecting
                let mut item_worry_level = current_monkey.items_worry_levels.pop_front().unwrap();
    
                // apply the operation to the item worry-level
                item_worry_level = match current_monkey.operation {
                    Operation::Add(n) => item_worry_level + n,
                    Operation::Multiply(n) => item_worry_level * n,
                    Operation::Square => item_worry_level.pow(2),
                };

                // apply relief mode if it is activated
                if relief_mode_activated {
                    item_worry_level /= 3;
                }

                // mod the worry-level by the LCM, so that it is in the range [0, LCM[
                // else we might overflow the u64
                item_worry_level %= lowest_common_multiple;
    
                // apply the divisible test
                if item_worry_level % current_monkey.divisible_test == 0 {
                    // if it is divisible, throw the item to the first monkey
                    let target_monkey = current_monkey.test_results.0;

                    // if the target monkey is the current monkey, push the item to the current monkey's queue
                    if target_monkey == current_monkey_index {
                        current_monkey.items_worry_levels.push_back(item_worry_level);
                    }
                    // else push the item to the target monkey's queue
                    else {
                        monkeys[target_monkey].items_worry_levels.push_back(item_worry_level);
                    }
                } else {
                    // if it is not divisible, throw the item to the second monkey
                    let target_monkey = current_monkey.test_results.1;

                    // if the target monkey is the current monkey, push the item to the current monkey's queue
                    if target_monkey == current_monkey_index {
                        current_monkey.items_worry_levels.push_back(item_worry_level);
                    }
                    // else push the item to the target monkey's queue
                    else {
                        monkeys[target_monkey].items_worry_levels.push_back(item_worry_level);
                    }
                }

                // increment the number of items inspected by the current monkey
                current_monkey.inspected_items_count += 1;
            }
    
            // put the current monkey back into the vector
            monkeys[current_monkey_index] = current_monkey;
        }
    }
}

/// The goal of this challenge is to find the monkey-business-level.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_11.txt").expect("Unable to read the input file");


    // parse the input into a vector of monkeys
    let monkeys = parse_monkeys(&input);
    let monkey_count = monkeys.len();


    // run the two simulations
    let mut monkeys_v1 = monkeys.clone();
    simulation(&mut monkeys_v1, 20, true);

    let mut monkeys_v2 = monkeys.clone();
    simulation(&mut monkeys_v2, 10_000, false);


    // compute the monkey business level
    let mut inspected_items_values = vec![];
    for monkey in &monkeys_v1 {
        inspected_items_values.push(monkey.inspected_items_count);
    }
    inspected_items_values.sort();
    let monkey_business_level_v1 = inspected_items_values[monkey_count - 1] * inspected_items_values[monkey_count - 2];
    
    let mut inspected_items_values = vec![];
    for monkey in &monkeys_v2 {
        inspected_items_values.push(monkey.inspected_items_count);
    }
    inspected_items_values.sort();
    let monkey_business_level_v2 = inspected_items_values[monkey_count - 1] * inspected_items_values[monkey_count - 2];


    // print the results
    println!(
        "The monkey business level is {monkey_business_level} in the first version.",
        monkey_business_level = monkey_business_level_v1.separate_with_commas()
    );
    println!(
        "The monkey business level is {monkey_business_level} in the first version.",
        monkey_business_level = monkey_business_level_v2.separate_with_commas()
    );
}
