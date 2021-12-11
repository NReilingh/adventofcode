// Day 5
use std::cmp::max;

pub fn hydrothermal_vents(input: Vec<String>) -> (u32, u32) {
    // Parse input to line segments
    let vents: Vec<LineSegment> = input.iter()
        .map(|item| item.parse().unwrap())
        // .map(|item| LineSegment::from(item.as_str()))
        .collect();

    // Reduce to max x and y values in grid
    let (x_max, y_max) = vents.iter()
        .fold((0u32, 0u32), |(x_max, y_max), curr| {
            (
                max(max(curr.0.x, curr.1.x), x_max),
                max(max(curr.0.y, curr.1.y), y_max)
            )
        });

    // Instantiate a coordinate grid with the max values
    let mut grid = Grid::new(x_max, y_max);

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
    matrix: Vec<u32>,
    width: u32,
}

impl Grid {
    fn new(x_max: u32, y_max: u32) -> Self {
        let x_size = x_max + 1;
        let y_size = y_max + 1;
        Grid {
            matrix: vec![0; (x_size * y_size).try_into().unwrap()],
            width: x_size,
        }
    }

    fn plot(&mut self, seg: &LineSegment) {
        let xdiff = i32::abs((seg.0.x - seg.1.x).try_into().unwrap());
        let ydiff = i32::abs((seg.0.y - seg.1.y).try_into().unwrap());

        let steps = max(xdiff, ydiff);

        let idx0 = seg.0.x + seg.0.y * self.width;
        let idx1 = seg.1.x + seg.1.y * self.width;

        let step_size = (idx1 - idx0) / steps;
        // This is a little bit confusing. If steps == 1,
        // then just incremend idx0 and idx1.
        // If steps > 1, take (idx1 - idx0) / steps
        // and increment idx1 by that amount steps times.
        // There's probably a range by steps or something?
        // Then I can iter_mut over the matrix and filter
        // to the values to increment.
    }

    fn count_overlaps(&self) -> u32 {
        self.matrix.iter().filter(|&x| *x > 1).count().try_into().unwrap()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Eq, PartialEq, Debug)]
struct LineSegment(Coordinate, Coordinate);

impl LineSegment {
    fn diagonal(&self) -> bool {
        false
    }
}

use std::str::FromStr;
use std::error::Error;
impl FromStr for LineSegment {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut results = s.split(&[' ', ',', '-', '>'][..])
            .filter(|x| !x.is_empty())
            .map(|x| Result::<_, Self::Err>::Ok(x.parse()?));

        let mut next = || {
            Result::<_, Self::Err>::Ok(
                results.next().ok_or_else(|| "Not enough parsed numerics.")??
            )
        };

        Ok(LineSegment(
            Coordinate { x: next()?, y: next()? },
            Coordinate { x: next()?, y: next()? },
        ))
    }
}

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn line_segment_from_string() {
        let input = "1,2 -> 3,4";
        let expected = LineSegment(
            Coordinate {
                x: 1,
                y: 2,
            },
            Coordinate {
                x: 3,
                y: 4,
            }
        );
        assert_eq!(expected, LineSegment::from_str(input).unwrap());
    }

    #[test]
    fn grid_size() {
        let mut grid = Grid::new(4, 4);
        let expected: Vec<u32> = vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        assert_eq!(expected, grid.matrix);
    }

    #[test]
    fn horizontal_plot() {
        let mut grid = Grid::new(4, 4);
        let seg = LineSegment(Coordinate {x: 1, y: 2}, Coordinate {x: 4, y: 2});
        let expected: Vec<u32> = vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 1,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        grid.plot(&seg);
        assert_eq!(expected, grid.matrix);
    }

    #[test]
    fn vertical_plot() {
        let mut grid = Grid::new(4, 4);
        let seg = LineSegment(Coordinate {x: 1, y: 3}, Coordinate {x: 1, y: 0});
        let expected: Vec<u32> = vec![
            0, 1, 0, 0, 0,
            0, 1, 0, 0, 0,
            0, 1, 0, 0, 0,
            0, 1, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        grid.plot(&seg);
        assert_eq!(expected, grid.matrix);
    }

    #[test]
    fn up_diag_plog() {
        let mut grid = Grid::new(4, 4);
        let seg = LineSegment(Coordinate {x: 2, y: 1}, Coordinate {x: 0, y: 3});
        let expected: Vec<u32> = vec![
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 1, 0, 0, 0,
            1, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        grid.plot(&seg);
        assert_eq!(expected, grid.matrix);
    }

    #[test]
    fn down_diag_plot() {
        let mut grid = Grid::new(4, 4);
        let seg = LineSegment(Coordinate {x: 0, y: 0}, Coordinate {x: 4, y: 4});
        let expected: Vec<u32> = vec![
            1, 0, 0, 0, 0,
            0, 1, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 1, 0,
            0, 0, 0, 0, 1,
        ];
        grid.plot(&seg);
        assert_eq!(expected, grid.matrix);
    }

    #[test]
    fn overlap_counting() {
        let grid = Grid {
            matrix: vec![0, 1, 1, 2, 1, 3, 3, 2, 1, 0, 4],
            width: 11,
        };
        assert_eq!(5, grid.count_overlaps());
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn overlapping_isolinears() {
        let input = read_input("../testinputs/05.txt");
        let (points, _) = hydrothermal_vents(input);
        assert_eq!(5, points);
    }
}
