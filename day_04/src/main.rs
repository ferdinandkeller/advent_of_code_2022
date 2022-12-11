use std::fs;
use std::ops::RangeInclusive;

/// The goal of this challenge is to find how many sections of the elfs' sections are contained in the other elfs' sections and how many are overlapping.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_4.txt").expect("Unable to read the input file");

    // create a variable to count how many sections are contained in the other elf's sections
    let mut contained_count: usize = 0;
    // create a variable to count how many sections are overlapping the other elf's sections
    let mut overlapping_count: usize = 0;

    // for each pair of elves
    for elf_pair in input.lines() {
        // split the data between the sections each elf is responsible for
        let split: Vec<&str> = elf_pair.split(",").collect();

        // find the first elf's sections
        let elf_1_split: Vec<&str> = split[0].split("-").collect();
        let elf_1_section_start: usize = elf_1_split[0].parse().unwrap();
        let elf_1_section_end: usize = elf_1_split[1].parse().unwrap();
        let elf_1_sections = elf_1_section_start..=elf_1_section_end;
        
        // find the second elf's sections
        let elf_2_split: Vec<&str> = split[1].split("-").collect();
        let elf_2_section_start: usize = elf_2_split[0].parse().unwrap();
        let elf_2_section_end: usize = elf_2_split[1].parse().unwrap();
        let elf_2_sections = elf_2_section_start..=elf_2_section_end;

        // find if one is contained in the other
        if elf_1_sections.contains_range(&elf_2_sections) || elf_2_sections.contains_range(&elf_1_sections) {
            contained_count += 1;
        }

        // find if one is overlapping the other
        if elf_1_sections.overlapping_range(&elf_2_sections) {
            overlapping_count += 1;
        }
    }

    // print the results
    println!("In total, {contained_count} ranges are contained in the other elf's ranges");
    println!("In total, {overlapping_count} ranges are overlapping the other elf's ranges");
}

/// Trait that adds methods to detect if two ranges are contained in each other or if they are overlapping
trait ContainsRange {
    /// Returns true if the range contains the other range
    fn contains_range(&self, range: &RangeInclusive<usize>) -> bool;

    /// Returns true if the range is overlapping the other range
    fn overlapping_range(&self, range: &RangeInclusive<usize>) -> bool;
}

impl ContainsRange for RangeInclusive<usize> {
    fn contains_range(&self, range: &RangeInclusive<usize>) -> bool {
        self.contains(&range.start()) && self.contains(&range.end())
    }

    fn overlapping_range(&self, range: &RangeInclusive<usize>) -> bool {
        self.start() <= range.end() && range.start() <= self.end()
    }
}
