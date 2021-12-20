// Day 9

use crate::Answer;

pub fn tube_smoke(input: Vec<String>) -> Answer {
    let heightmap: Vec<Vec<u32>> = input.iter()
        .map(|s| {
            s.chars().into_iter()
                .map(|c| {
                    c.to_digit(10).unwrap()
                })
                .collect()
        })
        .collect();

    let mut low_point_risks: Vec<(u32, (u32, u32))> = Vec::new();

    for (i, row) in heightmap.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if is_low_point(j, i, *height, &heightmap) {
                low_point_risks.push((height + 1, (j as u32, i as u32)));
            }
        }
    }

    let total_risk = low_point_risks.iter()
        .map(|x| x.0)
        .sum();

    let mut basin_sizes: Vec<u32> = low_point_risks.iter()
        .map(|x| get_basin_size(x.1, &heightmap))
        .collect();

    basin_sizes.sort_unstable();

    let largest_basin_product = basin_sizes.into_iter()
        .rev().take(3).reduce(|acc, cur| acc * cur).unwrap();

    Answer::U32(total_risk, largest_basin_product)
}

fn get_basin_size(coords: (u32, u32), map: &Vec<Vec<u32>>) -> u32 {
   2 
}

fn is_low_point(x: usize, y: usize, height: u32, map: &Vec<Vec<u32>>) -> bool {
    let x: i32 = x.try_into().unwrap();
    let y: i32 = y.try_into().unwrap();
    let x_bound: i32 = map[0].len().try_into().unwrap();
    let y_bound: i32 = map.len().try_into().unwrap();
    let check_coords: Vec<(i32, i32)>
        = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)].into_iter()
        .filter(|(x, y)| {
            !(*x == -1 || *x == x_bound || *y == -1 || *y == y_bound)
        })
        .collect();
    let check_heights: Vec<u32> = check_coords.iter()
        .map(|(x, y)| {
            map[*y as usize][*x as usize]
        })
        .collect();
    height < *check_heights.iter().min().unwrap()
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn first_puzzle() {
        let input = read_input("../testinputs/09.txt");
        if let Answer::U32(risk, _) = tube_smoke(input) {
            assert_eq!(15, risk);
        } else {
            panic!("wrong type");
        }
    }

    #[test]
    fn second_puzzle() {
        let input = read_input("../testinputs/09.txt");
        if let Answer::U32(_, second) = tube_smoke(input) {
            assert_eq!(1134, second);
        } else {
            panic!("wrong type");
        }
    }
}
