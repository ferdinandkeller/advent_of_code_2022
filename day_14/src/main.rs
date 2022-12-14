mod point;
mod line;
mod map;

use std::fs;
use map::{Map, UpdateState};
use thousands::Separable;

fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_14.txt").expect("Unable to read the input file.");

    // parse the input file into a map
    let map = Map::try_from(input).expect("Unable to parse the input file into a map.");

    // execute the first version of the simulation
    let mut map_v1 = map.clone();
    let mut units_added_v1 = 0;
    while let UpdateState::SandComesToRest = map_v1.update() {
        units_added_v1 += 1;
    }
    
    // execute the second version of the simulation (this time with a floor)
    let mut map_v2 = map.clone();
    map_v2.add_floor();
    let mut units_added_v2 = 0;
    while let UpdateState::SandComesToRest = map_v2.update() {
        units_added_v2 += 1;
    }

    // print the results of the simulations
    println!(
        "Units of sands added before they start flowing into the void: {units_added_v1}",
        units_added_v1 = units_added_v1.separate_with_commas()
    );
    println!(
        "Units of sands added before we can't add more (with a floor this time): {units_added_v2}",
        units_added_v2 = units_added_v2.separate_with_commas()
    );
}
