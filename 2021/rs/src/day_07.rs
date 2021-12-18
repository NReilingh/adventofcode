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
    println!("Initial optimizer state: {:#?}", optimizer);

    let optimal_cost = optimizer.min_fuel_position().target_cost.try_into().unwrap();

    Answer::U32(optimal_cost, 0)
}

#[derive(Debug)]
struct BoundCost {
    bound: usize,
    bound_target: i64,
    target_cost: i64,
}

// bottom_bound: usize, // index below which we have already calculated cost to target
// bottom_target: i64, // target for memoed cost. Cost above this target is bound_size * n
// bottom_cost: i64, // cost for the bound indexes to hit target
// top_bound: usize, // index and above which we have already calculated cost to target 
// top_target: i64, // target for memoed cost. Cost below this target is bound_size * n
// top_cost: i64, // cost for bound indexes to hit target

#[derive(Debug)]
struct ListValueOptimizer {
    list: Vec<i64>,
    bottom: BoundCost,
    top: BoundCost,
}

#[derive(Debug)]
struct Target(usize, Option<i64>);

impl ListValueOptimizer {
    fn list_size(&self) -> usize {
        self.list.len()
    }

    fn new(mut list: Vec<i64>) -> Self {
        list.sort_unstable();

        // With the vec sorted, we split it in half and calculate
        // the cost of each half to reach the upper or lower bound.
        // When we split in half we actually have to choose a target
        // between the two bounds. This should actually be proportional
        // to the positions we are splitting at.

        ListValueOptimizer {
            bottom: BoundCost {
                bound: 0,
                bound_target: *list.first().unwrap(),
                target_cost: 0,
            },
            top: BoundCost {
                bound: list.len(),
                bound_target: *list.last().unwrap(),
                target_cost: 0,
            },
            list,
        }
    }

    fn get_next_target(&self) -> Target {
        if self.top.bound == self.bottom.bound + 1 {
            // Refactor this to use match guards perhaps?
            let target = if self.top.bound * 2 < self.list_size() {
                self.top.bound_target
            } else if self.top.bound * 2 == self.list_size() {
                midpoint(self.bottom.bound_target, self.top.bound_target)
            } else if self.top.bound * 2 > self.list_size() {
                self.bottom.bound_target
            } else {
                unreachable!()
            };

            Target(self.top.bound, Some(target))
        } else {
            let pivot = midpoint(self.bottom.bound, self.top.bound);
            Target(pivot, None)
        }
    }

    fn pivot_cost(&self, pivot: usize) -> (BoundCost, BoundCost) {
        // Start by ensuring the pivot is between our existing bounds.
        assert!(pivot < self.top.bound && pivot > self.bottom.bound);

        // How do we efficiently calculate our costs at this pivot point?
        // Starting with below the pivot, use cost_to_target() to calculate
        // the slice from the bound to the pivot, then add the bound target_cost
        // plus the bound size times target minus bound_target.
        // So, first we need to know what our target is.
        let bottom_delta = &self.list[self.bottom.bound..pivot];
        let top_delta = &self.list[pivot..self.top.bound];
        let bottom_target = *bottom_delta.last().unwrap();
        let top_target = *top_delta.first().unwrap();
        let bottom_delta_cost = cost_to_target(bottom_target, bottom_delta);
        let top_delta_cost = cost_to_target(top_target, top_delta);

        let bottom_bound_size: i64 = self.bottom.bound.try_into().unwrap();
        let bottom_bound_delta = (bottom_target - self.bottom.bound_target) * bottom_bound_size;

        let top_bound_size: i64 = (self.list_size() - self.top.bound).try_into().unwrap();
        let top_bound_delta = (self.top.bound_target - top_target) * top_bound_size;

        let bottom_total_cost = self.bottom.target_cost + bottom_bound_delta + bottom_delta_cost;
        let top_total_cost = self.top.target_cost + top_bound_delta + top_delta_cost;

        (
            BoundCost {
                bound: pivot,
                bound_target: bottom_target,
                target_cost: bottom_total_cost,
            },
            BoundCost {
                bound: pivot,
                bound_target: top_target,
                target_cost: top_total_cost,
            }
        )
    }

    fn min_fuel_position(&mut self) -> BoundCost {
        let target = self.get_next_target();

        println!("Target acquired: {:?}", target);

        match target {
            Target(pivot, None) => {
                // Calculate the total cost for above and below the pivot to reach
                // their respective bound targets.
                let (bottom_bound, top_bound) = self.pivot_cost(pivot);
                println!("Calculated bottom: {:#?}, top: {:#?}", bottom_bound, top_bound);

                if bottom_bound.target_cost > top_bound.target_cost {
                    // move top bound to pivot
                    self.top = top_bound;
                } else {
                    // move bottom bound to pivot
                    self.bottom = bottom_bound;
                }
                // Recurse
                return self.min_fuel_position();
            },
            Target(pivot, Some(target)) => {
                return BoundCost {
                    bound: pivot,
                    bound_target: target,
                    target_cost: cost_to_target(target, &self.list),
                };
            }
        }
    }
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
