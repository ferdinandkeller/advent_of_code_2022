use crate::point::Point;

/// A line in a 2D space.
/// It is composed of a list of points, connected together.
#[derive(Debug, Clone)]
pub struct Line {
    pub points: Vec<Point>,
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // split the string when a " -> " is found
        // the line is a list of connected points, to each element should be a point
        // if any of the elements is not a point, then the string is not a valid line
        let points: Vec<Point> = value
            .split(" -> ")
            .map(|s| Point::try_from(s))
            .collect::<Result<Vec<Point>, ()>>()?;

        // return the line
        Ok(Line { points })
    }
}

impl TryFrom<String> for Line {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Line::try_from(value.as_ref())
    }
}
