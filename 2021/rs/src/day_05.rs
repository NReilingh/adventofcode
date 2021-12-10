// Day 5
use std::cmp::max;

pub fn hydrothermal_vents(input: Vec<String>) -> (u32, u32) {
    // Parse input to line segments
    let vents: Vec<LineSegment> = input.iter()
        .map(|item| LineSegment::from(item))
        .collect();

    // Reduce to max x and y values in grid
    let (x_max, y_max) = vents.iter()
        .fold((0u32, 0u32), |(x_max, y_max), curr| {
            x_max = max(max(curr.0.x, curr.1.x), x_max);
            y_max = max(max(curr.0.y, curr.1.y), y_max);
            (x_max, y_max)
        });

    // Instantiate a coordinate grid with the max values
    let grid = Grid::new(x_max, y_max);

    // Plot each isolinear(?) line segment on the grid
    for vent in vents.iter().filter(|s| !s.diagonal()) {
        grid.plot(&vent);
    }

    // Count overlaps
    let overlaps = grid.count_overlaps();

    // Additively plot diagonal line segments on the grid
    for vent in vents.iter().filter(|s| s.diagonal()) {
        grid.plot(&vent);
    }
    (overlaps, grid.count_overlaps())
}

struct Grid {
    matrix: Vec<Coordinate>,
}

impl Grid {
    fn new(x_max: u32, y_max: u32) -> Self {
    }

    fn plot(&mut self, segment: &LineSegment) {
    }

    fn count_overlaps(&self) -> u32 {
    }
}

struct Coordinate {
    x: u32,
    y: u32,
    magnitude: u32,
}

struct LineSegment(Coordinate, Coordinate);

impl LineSegment {
    fn diagonal(&self) -> bool {
    }
}

impl From<&String> for LineSegment {
    fn from(s: &String) -> Self {
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn overlapping_points() {
        let input = read_input("../testinputs/05.txt");
        let (points, _) = hydrothermal_vents(input);
        assert_eq!(5, points);
    }
}
