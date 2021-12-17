// Day 7

use crate::Answer;

pub fn crab_target_alignment(input: Vec<String>) -> Answer {
// I think this can be done with a quicksort-like divide-and-conquer algorithm
// If we sort the input values, we can take a "chunk" of them and calculate
// a static fuel cost to move them all to the upper or lower bound of the chunk.
// Then you can calculate further moves of the chunk with an O(1) multiplication
// instead of an O(n) calculation across each member of the chunk.
// Procedurally, we can split the sorted list down the middle and calculate
// the two chunks as upper-bound and lower-bound in threads, even,
// and then whichever chunk has a higher fuel cost to reach its bound
// gets split in half next.
// Subsequent splits are aggregated with all of the chunks below/above them,
// calculated according to shifting the entire chunk according to its fuel cost.
// Assuming this algorithm is sound, this means it's something like O(log(n))
// complexity instead of quadratic O(n^2) which it would be if we used a naive
// algorithm

    let mut crab_positions: Vec<i64> = input.iter().take(1)
        .flat_map(|x| x.split(','))
        .map(|x| x.parse().unwrap()).collect();

    let mut optimizer = ListValueOptimizer::new(crab_positions);

    let optimal_position = optimizer.min_fuel_position().try_into().unwrap();

    Answer::U32(optimal_position, 0)
}

struct BoundCost {
    bound: usize,
    bound_target: i64,
    target_cost: i64,
}

struct ListValueOptimizer {
    list: Vec<i64>,
    bottom: BoundCost,
    top: BoundCost,
}

struct Target(usize, Option<i64>);

impl ListValueOptimizer {
    fn list_size(&self) -> usize {
        self.list.len()
    }

    fn new(mut list: Vec<i64>) -> Self {
        list.sort_unstable();

        ListValueOptimizer {
            bottom: BoundCost {
                bound: 0,
                bound_target: *list.first().unwrap(),
                target_cost: i64::MAX,
            },
            top: BoundCost {
                bound: list.len(),
                bound_target: *list.last().unwrap(),
                target_cost: i64::MAX,
            },
            list,
        }
    }

    fn get_next_target(&self) -> Target {
        if self.top.bound == self.bottom.bound + 1 {
            // Refactor this to use match guards perhaps?
            let target = if self.top.bound * 2 < self.list_size() {
                self.bottom.bound_target
            } else if self.top.bound * 2 == self.list_size() {
                midpoint(self.bottom.bound_target, self.top.bound_target)
            } else if self.top.bound * 2 > self.list_size() {
                self.top.bound_target
            } else {
                unreachable!()
            };

            Target(self.top.bound, Some(target))
        } else {
            let pivot = midpoint(self.bottom.bound, self.top.bound);
            Target(pivot, None)
        }
    }

    fn min_fuel_position(&mut self) -> i64 {
        // let pivot = state.top.bound;
        //
        // let bottom_bound_val = set[pivot - 1];
        // let top_bound_val = set[pivot];

        let target = self.get_next_target();

        match target {
            Target(pivot, None) => {
                let bottom = &self.list[..pivot];
                let top = &self.list[pivot..];
                let bottom_cost = cost_to_target(*bottom.last().unwrap(), bottom);
                let top_cost = cost_to_target(*top.first().unwrap(), top);
            },
            Target(pivot, Some(target)) => {}
        }

        let split_pos = self.list.len() / 2;
        let bottom = &self.list[..split_pos];
        let top = &self.list[split_pos..];

        let bottom_min = bottom.iter().min().unwrap();
        let bottom_max = bottom.iter().max().unwrap();
        let top_min = top.iter().min().unwrap();
        let top_max = top.iter().max().unwrap();
        let target = bottom_max + (top_min - bottom_max) / 2;

        let bottom_cost = cost_to_target(target, bottom);
        let top_cost = cost_to_target(target, top);

        if bottom_cost > top_cost {
            // split bottom
        } else {
            // split top
        }
        0
    }
}

fn min_fuel_position(mut positions: Vec<i64>) -> i64 {
    struct DivideAndConquerState {
        bottom: BoundCost,
        top: BoundCost,
        // bottom_bound: usize, // index below which we have already calculated cost to target
        // bottom_target: i64, // target for memoed cost. Cost above this target is bound_size * n
        // bottom_cost: i64, // cost for the bound indexes to hit target
        // top_bound: usize, // index and above which we have already calculated cost to target 
        // top_target: i64, // target for memoed cost. Cost below this target is bound_size * n
        // top_cost: i64, // cost for bound indexes to hit target
    }

    positions.sort_unstable();

    // With the vec sorted, we split it in half and calculate
    // the cost of each half to reach the upper or lower bound.
    // When we split in half we actually have to choose a target
    // between the two bounds. This should actually be proportional
    // to the positions we are splitting at.

    let mut state = DivideAndConquerState {
        bottom: BoundCost {
            bound: 0,
            bound_target: *positions.first().unwrap(),
            target_cost: i64::MAX,
        },
        top: BoundCost {
            bound: positions.len(),
            bound_target: *positions.last().unwrap(),
            target_cost: i64::MAX,
        },
    };
    struct Target(usize, Option<i64>);

    fn get_next_target(state: &DivideAndConquerState, positions: &[i64]) -> Target {
        if state.bottom.bound + 1 == state.top.bound {
            let target = if state.top.bound * 2 < positions.len() {
               state.bottom.bound_target
            } else if state.top.bound * 2 == positions.len() {
                midpoint(state.bottom.bound_target, state.top.bound_target)
            } else if state.top.bound * 2 > positions.len() {
               state.top.bound_target
            } else {
                unreachable!()
            };
            let target = midpoint(state.bottom.bound_target, state.top.bound_target);
            Target(state.top.bound, Some(target))
        } else {
            let pivot = midpoint(state.bottom.bound, state.top.bound);
            Target(pivot, None)
        }
    }

    // let pivot = state.top.bound;
    //
    // let bottom_bound_val = set[pivot - 1];
    // let top_bound_val = set[pivot];

    let target = get_next_target(&state, &positions);

    match target {
        Target(pivot, None) => {
            let bottom = &positions[..pivot];
            let top = &positions[pivot..];
            let bottom_cost = cost_to_target(*bottom.last().unwrap(), bottom);
            let top_cost = cost_to_target(*top.first().unwrap(), top);
        },
        Target(pivot, Some(target)) => {}
    }

    let split_pos = positions.len() / 2;
    let bottom = &positions[..split_pos];
    let top = &positions[split_pos..];

    let bottom_min = bottom.iter().min().unwrap();
    let bottom_max = bottom.iter().max().unwrap();
    let top_min = top.iter().min().unwrap();
    let top_max = top.iter().max().unwrap();
    let target = bottom_max + (top_min - bottom_max) / 2;

    let bottom_cost = cost_to_target(target, bottom);
    let top_cost = cost_to_target(target, top);

    if bottom_cost > top_cost {
        // split bottom
    } else {
        // split top
    }
    0
}

use std::ops::{Sub, Div, Add};
fn midpoint<T>(lower: T, upper: T) -> T
where
    T: Copy + Div<Output = T> + Sub<Output = T> + Add<Output = T>
{
    let one = upper / upper;
    lower + (upper - lower) / (one + one)
}

#[cfg(test)]
mod midpoint_tests {
    use super::midpoint;

    #[test]
    fn no_rounding() {
        assert_eq!(5, midpoint(4, 6));
    }

    #[test]
    fn rounding_to_floor() {
        assert_eq!(4, midpoint(4, 5));
    }

    #[test]
    fn identity() {
        assert_eq!(4, midpoint(4, 4));
    }

    #[test]
    fn upside_down() {
        assert_eq!(5, midpoint(5, 4));
    }
}

fn cost_to_target(target: i64, set: &[i64]) -> i64 {
    set.iter().map(|x| (target - *x).abs()).sum()
}

#[cfg(test)]
mod cost_to_target_tests {
    use super::cost_to_target;

    #[test]
    fn interior_target() {
        let set = vec![3, 4, 4, 5, 7, 8, 9];
        assert_eq!(14, cost_to_target(6, &set));
    }

    #[test]
    fn below_target() {
        let set = vec![3, 4, 4, 5, 7, 8, 9];
        assert_eq!(33, cost_to_target(1, &set));
    }

    #[test]
    fn above_target() {
        let set = vec![3, 4, 4, 5, 7, 8, 9];
        assert_eq!(72, cost_to_target(16, &set));
    }
}

#[cfg(test)]
mod answer_tests {
    use super::*;

    use crate::read_input;

    #[test]
    fn minimum_fuel() {
        let input = read_input("../testinputs/07.txt");
        if let Answer::U32(fuel, _) = crab_target_alignment(input) {
            assert_eq!(37, fuel);
        } else { panic!("Answer was not a U32 variant.") }
    }
}
