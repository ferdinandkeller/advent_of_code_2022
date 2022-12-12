use std::fs;

/// Represents a place on the map by its x and y coordinates.
/// 
/// # Fields
/// 
/// * `x` - The x coordinate of the position.
/// * `y` - The y coordinate of the position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    /// Creates a new position with the given x and y coordinates.
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns the position north of this position.
    fn north(self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    /// Returns the position south of this position.
    fn south(self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    /// Returns the position west of this position.
    fn west(self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    /// Returns the position east of this position.
    fn east(self) -> Self {
        Self::new(self.x + 1, self.y)
    }
}

/// Represents an agent on the map.
/// An agent is a position on the map and the number of steps it took to get there. When moved, the agent will have a new position and the number of steps will be incremented by one.
/// 
/// # Fields
/// 
/// * `position` - The position of the agent.
/// * `steps` - The number of steps it took to get to the current position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Agent {
    position: Position,
    steps: usize,
}

impl Agent {
    /// Creates a new agent with the given position and number of steps.
    fn new(position: Position, steps: usize) -> Self {
        Self { position, steps }
    }

    /// Creates a new agent with the given position and zero steps.
    fn from_position(position: Position) -> Self {
        Self::new(position, 0)
    }
}

/// Represents the elevation map.
/// 
/// # Fields
/// 
/// * `elevations` - The elevations of the map. Is a number between 0 and 25.
/// * `map_width` - The width of the map.
/// * `map_height` - The height of the map.
/// * `start_position` - The start position of the map.
/// * `end_position` - The end position of the map.
struct ElevationMap {
    elevations: Vec<Vec<i64>>,
    map_width: usize,
    map_height: usize,
    start_position: Position,
    end_position: Position,
}

impl ElevationMap {
    /// Creates a new elevation map from the given input.
    fn parse_from_string(input: &str) -> Self {
        // variable to store the elevation of each point on the map
        let mut elevations = vec![];
        // variable to store the start position
        let mut start_position: Option<Position> = None;
        // variable to store the end position
        let mut end_position: Option<Position> = None;
    
        // iterate over each line in the input
        for (line_index, line) in input.lines().enumerate() {
            // variable to store the elevation of each point on the current line
            let mut row = vec![];
    
            // iterate over each character in the line
            for (column_index, char) in line.chars().enumerate() {
                // check if the character is a start position
                // the start position is represented by an S, and has an elevation of 0
                if char == 'S' {
                    row.push(0);
                    start_position = Some(Position::new(column_index, line_index));
                }
                // check if the character is an end position
                // the end position is represented by an E, and has an elevation of 25
                else if char == 'E' {
                    row.push(25);
                    end_position = Some(Position::new(column_index, line_index));
                }
                // otherwise, the character is a letter, with a to z corresponding to 0 to 25
                else {
                    row.push((char as i64) - 97);
                }
            }
    
            // add the row to the elevations
            elevations.push(row);
        }
    
        // compute the dimensions of the map
        let map_width = elevations[0].len();
        let map_height = elevations.len();
    
        // return the elevation map
        return ElevationMap {
            elevations,
            map_width,
            map_height,
            start_position: start_position.expect("No start position found."),
            end_position: end_position.expect("No end position found."),
        };
    }
}

/// Simulates the agents moving on the map.
fn simulate(elevation_map: &ElevationMap, mut agents: Vec<Agent>) -> Option<usize> {
    // store the locations that have been visited
    let mut visited_locations: Vec<Vec<bool>> = vec![vec![false; elevation_map.map_width]; elevation_map.map_height];

    // mark the starting positions of the agents as visited
    for agent in &agents {
        visited_locations[agent.position.y][agent.position.x] = true;
    }

    // simulate the agents moving on the map until they found the end position or there are no more move options
    loop {
        // create a variable to store the next wave of agents
        let mut next_agents: Vec<Agent> = vec![];

        // iterate over each agent
        for agent in agents {
            // check if the agent has reached the end position
            if agent.position == elevation_map.end_position {
                return Some(agent.steps);
            }

            // get the elevation of the current agent
            let agent_elevation = elevation_map.elevations[agent.position.y][agent.position.x];

            // go north if possible
            if agent.position.y > 0 {
                let north = agent.position.north();
                let elevation = elevation_map.elevations[north.y][north.x];

                if elevation - agent_elevation <= 1 && !visited_locations[north.y][north.x] {
                    visited_locations[north.y][north.x] = true;
                    next_agents.push(Agent::new(north, agent.steps + 1));
                }
            }
            // go south if possible
            if agent.position.y < elevation_map.map_height - 1 {
                let south = agent.position.south();
                let elevation = elevation_map.elevations[south.y][south.x];

                if elevation - agent_elevation <= 1 && !visited_locations[south.y][south.x] {
                    visited_locations[south.y][south.x] = true;
                    next_agents.push(Agent::new(south, agent.steps + 1));
                }
            }
            // go west if possible
            if agent.position.x > 0 {
                let west = agent.position.west();
                let elevation = elevation_map.elevations[west.y][west.x];

                if elevation - agent_elevation <= 1 && !visited_locations[west.y][west.x] {
                    visited_locations[west.y][west.x] = true;
                    next_agents.push(Agent::new(west, agent.steps + 1));
                }
            }
            // go east if possible
            if agent.position.x < elevation_map.map_width - 1 {
                let east = agent.position.east();
                let elevation = elevation_map.elevations[east.y][east.x];

                if elevation - agent_elevation <= 1 && !visited_locations[east.y][east.x] {
                    visited_locations[east.y][east.x] = true;
                    next_agents.push(Agent::new(east, agent.steps + 1));
                }
            }
        }

        // if the next wave of agents is empty, then there are no more move options
        // in that case there are no paths to the end position
        if next_agents.is_empty() {
            return None;
        }
        
        // update the agents to the next wave
        agents = next_agents;
    }
}

/// The goal of this challenge is to find the shortest path from the start position to the end position.
fn main() {
    // read the input file
    let input = fs::read_to_string("inputs/day_12.txt").expect("Unable to read the input file");

    // parse the input into an elevation map
    let elevation_map = ElevationMap::parse_from_string(&input);

    // create the first version of the agents
    let mut agents_v1: Vec<Agent> = vec![];
    agents_v1.push(Agent::from_position(elevation_map.start_position));

    // run the first version of the simulation
    let results_v1 = simulate(&elevation_map, agents_v1).unwrap();

    // create the second version of the agents
    let mut agents_v2: Vec<Agent> = vec![];
    for y in 0..elevation_map.map_height {
        for x in 0..elevation_map.map_width {
            if elevation_map.elevations[y][x] == 0 {
                agents_v2.push(Agent::from_position(Position::new(x, y)));
            }
        }
    }

    // run the second version of the simulation
    let results_v2 = simulate(&elevation_map, agents_v2).unwrap();

    // print the results
    println!("The shortest path when starting from S takes {} steps.", results_v1);
    println!("The shortest path when starting from any 0 elevation positions takes {} steps.", results_v2);
}
