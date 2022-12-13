use std::{
    fs,
    cmp::{Ordering, PartialOrd, Ord},
    convert::TryFrom
};
use thousands::Separable;

/// A packet is either a list of other packets, or a number.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(u64),
}

impl Packet {
    /// Wraps the packet into a list-packet.
    fn wrap(&self) -> Packet {
        return Packet::List(vec![self.clone()]);
    }
}

impl TryFrom<&str> for Packet {
    type Error = ();

    fn try_from(packet_str: &str) -> Result<Self, Self::Error> {
        /// Returns true if the string represents a number.
        fn is_number(packet_str: &str) -> bool {
            if packet_str.len() == 0 {
                return false;
            }

            for c in packet_str.chars() {
                if !c.is_digit(10) {
                    return false;
                }
            }

            return true;
        }
        
        /// Returns true if the string represents a list.
        /// A list is a sequence of packets enclosed in square brackets.
        /// No assumptions are made about the contents of the list (The brackets might not be balanced).
        fn is_list(packet_str: &str) -> bool {
            if packet_str.len() < 2 {
                return false;
            }

            packet_str.starts_with('[') && packet_str.ends_with(']')
        }
        
        /// Assuming the string represents a list, returns a vector of strings representing the sub-packets.
        fn get_list_subpackets(packet_str: &str) -> Option<Vec<&str>> {
            // remove the brackets
            let packet_str = &packet_str[1..packet_str.len() - 1];
            
            // create a vector to store the sub-packets
            let mut sub_packets = vec![];

            // create a var to know where the current sub-packet starts
            let mut sub_packet_start = 0;
            // create a var to know the depth level of the current sub-packet
            let mut depth_level: usize = 0;
        
            // iterate over the characters of the string
            for (index, c) in packet_str.chars().enumerate() {
                // if the current character is an open bracket, the depth level increases
                if c == '[' {
                    depth_level += 1;
                }
                // if the current character is a close bracket, the depth level decreases
                else if c == ']' {
                    if depth_level == 0 {
                        return None;
                    }
                    depth_level -= 1;
                }
                // if the current character is a comma and the depth level is 0, that means we are at the end of a sub-packet
                else if c == ',' && depth_level == 0 {
                    sub_packets.push(&packet_str[sub_packet_start..index]);
                    sub_packet_start = index + 1;
                }
            }
        
            // if the list isn't empty
            if packet_str.len() != 0 {
                // add the last sub-packet
                sub_packets.push(&packet_str[sub_packet_start..]);
            }
        
            // return the sub-packets
            Some(sub_packets)
        }
        
        // if the string represents a number, parse it and return a packet containing the number
        if is_number(packet_str) {
            let number = packet_str.parse::<u64>().expect("Invalid number");
            return Ok(Packet::Number(number));
        }
        // if the string represents a list, parse its content and return everything
        else if is_list(packet_str) {
            // create a vector to store the sub-packets
            let mut sub_packets = vec![];

            // iterate over the sub-packets
            // if there is an error while spliting the string, return the error
            for sub_packet_str in get_list_subpackets(packet_str).ok_or_else(|| ())? {
                // parse the sub-packet and add it to the vector
                // if there is an error, return the error
                sub_packets.push(Packet::try_from(sub_packet_str)?);
            }

            // return the list
            return Ok(Packet::List(sub_packets));
        }
        // if the string doesn't represent a number or a list, then the string is invalid
        else {
            return Err(());
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        // match the two packets and compare them accordingly
        match (self, other) {
            // if both packets are numbers, compare them directly
            (Packet::Number(number_1), Packet::Number(number_2)) => {
                return number_1.cmp(number_2);
            },
            // if only one packet is a number, wrap it into a list and compare both lists
            (Packet::Number(_), Packet::List(_)) => {
                return self.wrap().cmp(other);
            },
            (Packet::List(_), Packet::Number(_)) => {
                return self.cmp(&other.wrap());
            },
            // if both packets are lists, compare them element by element
            (Packet::List(list_1), Packet::List(list_2)) => {
                // compare the lists element by element
                for (value_1, value_2) in list_1.into_iter().zip(list_2.into_iter()) {
                    let comparison = value_1.cmp(value_2);
                    if comparison != Ordering::Equal {
                        return comparison;
                    }
                }

                // if the lists are equal up to the length of the shortest list, compare their lengths
                return list_1.len().cmp(&list_2.len());
            }
        }
    }
}


fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_13.txt").expect("Unable to read the input file");

    // create a vector to store the packets
    let mut packets = vec![];
    let mut valid_pairs_index_sum = 0;

    // iterate over the pairs of packets
    for (pair_index, pair) in input.split("\n\n").enumerate() {
        // split the pair
        let split: Vec<&str> = pair.split("\n").collect();

        // if the pair doesn't contain exactly two packets, panic
        if split.len() != 2 {
            panic!("Invalid input");
        }

        // parse the two packets
        let packet_1: Packet = split[0].try_into().expect("Invalid syntax for packet 1");
        let packet_2: Packet = split[1].try_into().expect("Invalid syntax for packet 2");

        // add the packets to the vector of packets
        packets.push(packet_1.clone());
        packets.push(packet_2.clone());

        // if the packets are in the right order, add the pair index to the sum
        if packet_1 <= packet_2 {
            // + 1 because the index starts at 1 and not 0
            valid_pairs_index_sum += pair_index + 1;
        }
    }

    // create the two divider packets
    let packet_two = Packet::Number(2).wrap().wrap();
    let packet_six = Packet::Number(6).wrap().wrap();

    // add the divider packets to the vector of packets
    packets.push(packet_two.clone());
    packets.push(packet_six.clone());

    // sort the packets
    packets.sort();

    // find the indices of the divider packets
    let index_packet_two = packets.binary_search(&packet_two).expect("Packet two not found");
    let index_packet_six = packets.binary_search(&packet_six).expect("Packet six not found");

    // print the results
    println!(
        "The sum of the pair indices is {}.",
        valid_pairs_index_sum.separate_with_commas()
    );
    println!(
        "The product of divisor packets indices is {}.",
        ((index_packet_two + 1) * (index_packet_six + 1)).separate_with_commas()
    );
}
