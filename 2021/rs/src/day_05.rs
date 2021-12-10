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
    matrix: Vec<Coordinate>,
}

impl Grid {
    fn new(x_max: u32, y_max: u32) -> Self {
        Grid {
            matrix: Vec::new(),
        }
    }

    fn plot(&mut self, segment: &LineSegment) {
    }

    fn count_overlaps(&self) -> u32 {
        0
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
    magnitude: u32,
}
impl Coordinate {
    fn new(x: u32, y: u32) -> Self {
        Coordinate {
            x,
            y,
            magnitude: 0,
        }
    }
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
            .map(|x| Result::<u32, Self::Err>::Ok(x.parse()?));

        let first = results.next().ok_or_else(|| "Not enough parsed numerics.")??;
        let second = results.next().ok_or_else(|| "Not enough parsed numerics.")??;
        let third = results.next().ok_or_else(|| "Not enough parsed numerics.")??;
        let fourth = results.next().ok_or_else(|| "Not enough parsed numerics.")??;

        Ok(LineSegment(
                Coordinate::new(first, second),
                Coordinate::new(third, fourth)
        ))
    }
}

        // println!("wat {:?}", results.next().ok_or_else(|| "Not enough parsed numerics.")??);
        // let next_result = |results: Map<Filter<Split<&[char]>, |&&str| -> bool>, |&str| -> Result<u32, Box<dyn Error>>>| {
        //     Ok(results.next().ok_or_else(|| "Not enough parsed values.")??)
        // };
     
        // let next_result = |results: &mut dyn Iterator<Item = Result<u32, Self::Err>>| -> Result<u32, Self::Err> {
        //     Ok(results.next().ok_or_else(|| "Not enough parsed values.")??)
        // };
        //
        // println!("wat {:?}", next_result(&results));

        // Ok(LineSegment(
        //         Coordinate::new(0, 0),
        //         Coordinate::new(0, 0)
        // ))

        // Ok(LineSegment(
        //     Coordinate::new(
        //         results.next().ok_or("boop")??,
        //         results.next().ok_or("boop")??
        //     ),
        //     Coordinate::new(
        //         results.next().ok_or("boop")??,
        //         results.next().ok_or("boop")??
        //     )
        // ))
//     }
// }

// impl From<&str> for LineSegment {
//     fn from(s: &str) -> Self {
//     }
// }

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
                magnitude: 0,
            },
            Coordinate {
                x: 3,
                y: 4,
                magnitude: 0,
            }
        );
        assert_eq!(expected, LineSegment::from_str(input).unwrap());
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
