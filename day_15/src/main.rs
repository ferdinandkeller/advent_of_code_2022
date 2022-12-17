use std::{fs, collections::HashSet, ops::Range, str::FromStr};
use regex::Regex;
use thousands::Separable;
use z3::{self, ast::Ast};
use anyhow;

fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_15.txt").expect("Unable to read the input file.");

    // parse the input
    let mut sensors: Vec<Sensor> = vec![];
    let mut beacons: HashSet<Location> = HashSet::new();

    for line in input.lines() {
        let sensor: Sensor = line.parse().expect("Unable to parse the sensor.");
        beacons.insert(sensor.beacon_location);
        sensors.push(sensor);
    }
    
    // compute the empty position at index
    let mut total_empty_positions = 0;

    // load the ranges of the scanline
    let ranges = get_scanline_ranges(&sensors, 2_000_000);

    // compute the total number of empty positions
    for range in &ranges {
        total_empty_positions += range.end - range.start;
    }

    // remove the beacons from the empty positions
    'beacons_iter: for beacon in &beacons {
        if beacon.y == 2_000_000 {
            for range in &ranges {
                if range.contains(&beacon.x) {
                    total_empty_positions -= 1;
                    continue 'beacons_iter;
                }
            }
        }
    }

    // print the total number of empty positions
    println!(
        "Number of empty positions: {}.",
        total_empty_positions.separate_with_commas()
    );
    
    // create a z3 solver
    let solver_config = z3::Config::new();
    let solver_context = z3::Context::new(&solver_config);
    let solver = z3::Solver::new(&solver_context);
    
    // create the hidden beacon's location variables
    let hidden_beacon_x = z3::ast::Int::new_const(&solver_context, "hidden_beacon_x");
    let hidden_beacon_y = z3::ast::Int::new_const(&solver_context, "hidden_beacon_y");
    
    // assert that the beacon's location is within a grid of 4,000,000 x 4,000,000
    solver.assert(&hidden_beacon_x.ge(&z3::ast::Int::from_i64(&solver_context, 0)));
    solver.assert(&hidden_beacon_x.le(&z3::ast::Int::from_i64(&solver_context, 4_000_000)));
    solver.assert(&hidden_beacon_y.ge(&z3::ast::Int::from_i64(&solver_context, 0)));
    solver.assert(&hidden_beacon_y.le(&z3::ast::Int::from_i64(&solver_context, 4_000_000)));

    // for each sensor in the dataset
    for sensor in sensors {
        // convert the sensor's location to z3's Int
        let sensor_x = z3::ast::Int::from_i64(&solver_context, sensor.location.x as i64);
        let sensor_y = z3::ast::Int::from_i64(&solver_context, sensor.location.y as i64);
        let sensor_clear_radius = z3::ast::Int::from_i64(&solver_context, sensor.sensor_clear_radius as i64);

        // calculate the distance from the sensor to the beacon
        let distance_from_sensor = z3_manhattan_distance(
            &sensor_x, &sensor_y,
            &hidden_beacon_x, &hidden_beacon_y
        );

        // assert that the distance from the sensor to the beacon is greater than the sensor's clear radius
        solver.assert(&distance_from_sensor.gt(&sensor_clear_radius));
    }

    // check that the problem is satisfiable
    if solver.check() == z3::SatResult::Sat {
        // get the model
        let model = solver.get_model().unwrap();

        // extract the solutions
        let hidden_beacon_x_i64 = model
            .eval(&hidden_beacon_x, true)
            .expect("Could not extract the beacon's x coordinate.")
            .as_i64()
            .expect("Could not convert the beacon's x coordinate to an i64.");
        let hidden_beacon_y_i64 = model
            .eval(&hidden_beacon_y, true)
            .expect("Could not extract the beacon's y coordinate.")
            .as_i64()
            .expect("Could not convert the beacon's y coordinate to an i64.");

        // compute the beacon's tuning frequency
        let beacon_tuning_frequency = hidden_beacon_x_i64 * 4_000_000 + hidden_beacon_y_i64;

        // print the beacon's tuning frequency
        println!(
            "Beacon's tuning frequency : {beacon_tuning_frequency}",
            beacon_tuning_frequency = beacon_tuning_frequency.separate_with_commas()
        );
    } else {
        println!("No solution found");
    }

}

// compute the absolute value of a z3 integer
fn z3_abs<'ctx>(number: &z3::ast::Int<'ctx>) -> z3::ast::Int<'ctx> {
    let ctx = number.get_ctx();
    let zero = z3::ast::Int::from_i64(ctx, 0i64);
    number.ge(&zero).ite(&number, &(-number))
}

// compute the manhattan distance between two z3 integers
fn z3_manhattan_distance<'ctx>(x1: &z3::ast::Int<'ctx>, y1: &z3::ast::Int<'ctx>, x2: &z3::ast::Int<'ctx>, y2: &z3::ast::Int<'ctx>) -> z3::ast::Int<'ctx> {
    z3_abs(&(x1 - x2)) + z3_abs(&(y1 - y2))
}

// compute the scanline ranges for a given scanline
fn get_scanline_ranges(sensors: &Vec<Sensor>, scanline_y: isize) -> Vec<Range<isize>> {
    let mut ranges_start: Vec<isize> = vec![];
    let mut ranges: Vec<Range<isize>> = vec![];

    for sensor in sensors {
        let distance_from_scan_line = (sensor.location.y - scanline_y).abs();
        let sensor_clear_radius = sensor.sensor_clear_radius as isize;

        if sensor_clear_radius < distance_from_scan_line {
            continue;
        }

        let scan_line_radius = sensor_clear_radius - distance_from_scan_line;
        let range_start = sensor.location.x - scan_line_radius;
        let range_end = sensor.location.x + scan_line_radius + 1;
        
        let insertion_index = ranges_start.binary_search(&range_start).unwrap_or_else(|e| e);
        ranges_start.insert(insertion_index, range_start);
        ranges.insert(insertion_index, range_start..range_end);

        let mut new_ranges = vec![];
        let mut current_range = ranges[0].clone();

        for range in ranges.iter().skip(1) {
            if range.start <= current_range.end {
                current_range.end = current_range.end.max(range.end);
            } else {
                new_ranges.push(current_range);
                current_range = range.clone();
            }
        }
        new_ranges.push(current_range);

        ranges_start = Vec::with_capacity(new_ranges.len());
        for range in &new_ranges {
            ranges_start.push(range.start);
        }
        ranges = new_ranges;
    }

    ranges
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sensor {
    location: Location,
    beacon_location: Location,
    sensor_clear_radius: usize,
}

impl Sensor {
    fn new(location: Location, beacon_location: Location, sensor_clear_radius: usize) -> Self {
        Self { location, beacon_location, sensor_clear_radius }
    }
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensor_regex = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").expect("The provided regex for parsing a sensor is invalid.");

        let captures = sensor_regex
            .captures(s)
            .ok_or(anyhow::Error::msg("Couldn't extract captures from regex match."))?;

        let sensor_x = captures
            .name("sensor_x")
            .ok_or(anyhow::Error::msg("Couldn't find the capture 'sensor_x' in the match."))?
            .as_str().parse::<isize>()?;
        let sensor_y = captures
            .name("sensor_y")
            .ok_or(anyhow::Error::msg("Couldn't find the capture 'sensor_y' in the match."))?
            .as_str().parse::<isize>()?;
        let sensor_location = Location::new(sensor_x, sensor_y);

        let beacon_x = captures
            .name("beacon_x")
            .ok_or(anyhow::Error::msg("Couldn't find the capture 'beacon_x' in the match."))?
            .as_str().parse::<isize>()?;
        let beacon_y = captures
            .name("beacon_y")
            .ok_or(anyhow::Error::msg("Couldn't find the capture 'beacon_y' in the match."))?
            .as_str().parse::<isize>()?;
        let beacon_location = Location::new(beacon_x, beacon_y);

        Ok(Sensor::new(
            sensor_location,
            beacon_location,
            sensor_location.manhattan_distance(beacon_location)
        ))
    }
}
