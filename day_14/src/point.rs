use std::convert::TryFrom;

/// A point in a 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// Creates a new point using the given coordinates.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The x coordinate of the point.
    /// * `y` - The y coordinate of the point.
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl TryFrom<&str> for Point {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // split the string when a comma is found
        let split: Vec<&str> = value.split(',').collect();

        // if the split is not of length 2, then the string was not a valid point
        if split.len() != 2 {
            return Err(());
        }

        // try to parse the x and y coordinates
        let x = split[0].parse::<usize>().map_err(|_| ())?;
        let y = split[1].parse::<usize>().map_err(|_| ())?;

        // return the point
        Ok(Point::new(x, y))
    }
}

impl TryFrom<String> for Point {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Point::try_from(value.as_ref())
    }
}
