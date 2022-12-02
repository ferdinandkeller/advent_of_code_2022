use std::fs;
use std::str::FromStr;

/// Represents the hand a player is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

/// Implement useful methods for the Hand enum.
impl Hand {
    /// Returns the hand that beats the current one.
    fn get_better_hand(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    /// Returns the hand that loses to the current one.
    fn get_worse_hand(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

/// Parses a string into a hand.
/// 
/// A & X are rocks
/// B & Y are papers
/// C & Z are scissors
impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("Invalid hand".to_string());
        }

        match s.chars().next().unwrap() {
            'A' => Ok(Hand::Rock),
            'B' => Ok(Hand::Paper),
            'C' => Ok(Hand::Scissors),
            'X' => Ok(Hand::Rock),
            'Y' => Ok(Hand::Paper),
            'Z' => Ok(Hand::Scissors),
            _ => Err("Invalid hand".to_string()),
        }
    }
}

/// Compares two hands to find who the winner is.
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        } else if self.get_better_hand() == *other {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Greater);
        }
    }
}

/// A struct to store the data about the result of a round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoundResult {
    Loose,
    Draw,
    Win,
}

/// Parses a string into the result of a round.
/// 
/// X is a loose
/// Y is a draw
/// Z is a win
impl FromStr for RoundResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("Invalid ordering".to_string());
        }

        match s.chars().next().unwrap() {
            'X' => Ok(RoundResult::Loose),
            'Y' => Ok(RoundResult::Draw),
            'Z' => Ok(RoundResult::Win),
            _ => Err("Invalid ordering".to_string()),
        }
    }
}

/// Our goal is to compute how many points we would make using the given strategy.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_2.txt").expect("Unable to read the input file");

    // create a variable to store the result of the tournament depending on the method used
    let mut score_method_1 = 0;
    let mut score_method_2 = 0;

    // for each round in the tournament
    for round in input.lines() {
        // parse the hand of each player for the current round
        let elf_hand = Hand::from_str(&round[0..1]).unwrap();
        let human_hand = Hand::from_str(&round[2..3]).unwrap();
        let ordering = RoundResult::from_str(&round[2..3]).unwrap();

        // Method 1: compare the hands
        // attribute points based on the hand
        match human_hand {
            Hand::Rock => score_method_1 += 1,
            Hand::Paper => score_method_1 += 2,
            Hand::Scissors => score_method_1 += 3,
        }

        // attribute points based on the winner
        if human_hand == elf_hand {
            score_method_1 += 3;
        } else if human_hand > elf_hand {
            score_method_1 += 6;
        }

        // Method 2: compare the ordering
        // attribute points based on the hand
        let computed_human_hand = match ordering {
            RoundResult::Loose => elf_hand.get_worse_hand(),
            RoundResult::Draw => elf_hand,
            RoundResult::Win => elf_hand.get_better_hand(),
        };
        match computed_human_hand {
            Hand::Rock => score_method_2 += 1,
            Hand::Paper => score_method_2 += 2,
            Hand::Scissors => score_method_2 += 3,
        }
        
        // attribute points based on the winner
        if computed_human_hand == elf_hand {
            score_method_2 += 3;
        } else if computed_human_hand > elf_hand {
            score_method_2 += 6;
        }
    }

    // display the final score
    println!("By following the strategy guide, you will score :)");
    println!("{score_method_1} points using the first method.");
    println!("{score_method_2} points using the second method.");
}
