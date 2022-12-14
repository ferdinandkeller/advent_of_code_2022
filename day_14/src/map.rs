use std::{convert::TryFrom, fmt::Display};
use crate::line::Line;

/// A cell represents the state of a single point in the map.
/// It can be either air, a wall or sand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Air,
    Wall,
    Sand,
}

/// The UpdateState is the result we obtain after updating the map by adding one unit of sand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateState {
    /// The added sand has come to rest, and the pile is stable.
    SandComesToRest,
    /// The added sand went outside of the map, and fell into the abyss.
    SandFlowsIntoAbyss,
    /// The added sand cannot flow, because its spawning point is blocked by a wall or by more sand.
    SandCannotFlow,
}

/// A map is a 2D space, where sand can be simulated.
#[derive(Debug, Clone)]
pub struct Map {
    state: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    sand_start_x: usize,
}

impl Map {
    /// Updates the map by adding one unit of sand.
    pub fn update(&mut self) -> UpdateState {
        // create the sand
        let mut sand_x = self.sand_start_x;
        let mut sand_y = 0;

        // if the sand is blocked
        if self.state[sand_y][sand_x] != Cell::Air {
            // return that the sand cannot flow
            return UpdateState::SandCannotFlow;
        }

        // while the sand can flow, move it
        loop {
            // if the sand is at the bottom of the map
            if sand_y + 1 == self.height {
                // return that the sand has fallen into the abyss
                return UpdateState::SandFlowsIntoAbyss;
            }
            // else if the sand can flow down
            else if self.state[sand_y+1][sand_x] == Cell::Air {
                // move the sand down
                sand_y += 1;
            }
            // else if the sand is at the left edge of the map
            else if sand_x == 0 {
                // return that the sand has fallen into the abyss
                return UpdateState::SandFlowsIntoAbyss;
            }
            // else if the sand can flow left
            else if self.state[sand_y+1][sand_x-1] == Cell::Air {
                // move the sand left
                sand_x -= 1;
                sand_y += 1;
            }
            // else if the sand is at the right edge of the map
            else if sand_x + 1 == self.width {
                // return that the sand has fallen into the abyss
                return UpdateState::SandFlowsIntoAbyss;
            }
            // else if the sand can flow right
            else if self.state[sand_y+1][sand_x+1] == Cell::Air {
                // move the sand right
                sand_x += 1;
                sand_y += 1;
            }
            // if the sand is not at an edge, but still cannot move neither down, left, or right,
            // then it means the sand has come to a rest, we can stop the loop
            else {
                break;
            }
        }

        // update the map
        self.state[sand_y][sand_x] = Cell::Sand;

        // return that the sand has come to rest
        return UpdateState::SandComesToRest;
    }

    /// Add a floor to the map.
    pub fn add_floor(&mut self) {
        for x in 0..self.width {
            self.state[self.height - 1][x] = Cell::Wall;
        }
    }
}

impl TryFrom<&str> for Map {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // try to parse the input into a vector of lines
        let lines: Vec<Line> = value
            .lines()
            .map(|s| s.try_into())
            .collect::<Result<Vec<Line>, ()>>()?;

        // find the maximum y coordinate present in the input
        let map_max_y = lines.iter().map(|l| l.points.iter().map(|p| p.y).max().unwrap()).max().unwrap() + 2;
        // find the minimum and maximum x coordinates needed to contain the map
        // we are making the map bigger than needed,
        // but it's the smallest map that can simulate until the end without needing a resize
        let map_min_x = 500 - map_max_y;
        let map_max_x = 500 + map_max_y;
        
        // calculate the width and height of the map
        let map_width = map_max_x - map_min_x + 1;
        let map_height = map_max_y + 1;

        // create the map, and fill it with air
        let mut state = vec![vec![Cell::Air; map_width]; map_height];

        // for each line in the input
        for line in lines {
            // iterate points two by two
            for (point_1, point_2) in line.points.iter().zip(line.points.iter().skip(1)) {
                // draw a vertical line
                if point_1.x == point_2.x {
                    // find the start and end of the line (with start <= end)
                    let (line_start, line_end) = if point_1.y < point_2.y {
                        (point_1.y, point_2.y)
                    } else {
                        (point_2.y, point_1.y)
                    };

                    // draw the line
                    for y in line_start..=line_end {
                        state[y][point_1.x - map_min_x] = Cell::Wall;
                    }
                }
                
                // draw a horizontal line
                else if point_1.y == point_2.y {
                    // find the start and end of the line (with start <= end)
                    let (line_start, line_end) = if point_1.x < point_2.x {
                        (point_1.x, point_2.x)
                    } else {
                        (point_2.x, point_1.x)
                    };

                    // draw the line
                    for x in line_start..=line_end {
                        state[point_1.y][x - map_min_x] = Cell::Wall;
                    }
                }
            }
        }

        // create the map
        Ok(Self {
            state,
            width: map_width,
            height: map_height,
            sand_start_x: 500 - map_min_x,
        })
    }
}

impl TryFrom<String> for Map {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // create a string to hold the stringified map
        let mut stringified_map = String::new();

        // iterate over lines of the map
        for line in &self.state {
            // for each cell in the line
            for cell in line {
                // display the cell
                match cell {
                    Cell::Air => stringified_map.push_str(". "),
                    Cell::Wall => stringified_map.push_str("# "),
                    Cell::Sand => stringified_map.push_str("O "),
                }
            }

            // add a new line
            stringified_map.push_str("\n");
        }

        // display the stringified map
        write!(f, "{}", stringified_map)
    }
}
