use std::collections::HashSet;

use crate::{
    point::Point2D,
    solvable::{Solution, Solvable},
};

pub struct Day04;

impl Solvable for Day04 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let paper = Paper::new(input);
        let solution = paper
            .0
            .iter()
            .filter(|&&roll| {
                roll.neighbors()
                    .iter()
                    .filter(|&n| paper.0.contains(n))
                    .count()
                    < 4
            })
            .count() as i128;

        Solution::new(solution)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        let paper = Paper::new(input);
        let mut rolls = paper.0.clone();

        loop {
            let next: HashSet<Point2D> = rolls
                .iter()
                .copied()
                .filter(|roll| {
                    roll.neighbors()
                        .iter()
                        .filter(|&n| rolls.contains(n))
                        .count()
                        >= 4
                })
                .collect();

            if next.len() == rolls.len() {
                break;
            }

            rolls = next;
        }

        let solution = (paper.0.len() - rolls.len()) as i128;

        Solution::new(solution)
    }
}

struct Paper(HashSet<Point2D>);

impl Paper {
    fn new(input: &str) -> Self {
        let set = input
            .lines()
            .enumerate()
            .flat_map(|(x, row)| {
                row.bytes().enumerate().filter_map(move |(y, b)| {
                    if b == b'@' {
                        Some(Point2D {
                            x: i32::try_from(x).unwrap(),
                            y: i32::try_from(y).unwrap(),
                        })
                    } else {
                        None
                    }
                })
            })
            .collect();
        Self(set)
    }
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_day_04_first_task() {
        let day_04 = Day04;
        let solution = day_04.first(INPUT);
        assert_eq!(Solution::new(13), solution);
    }

    #[test]
    fn test_day_04_second_task() {
        let day_04 = Day04;
        let solution = day_04.second(INPUT);
        assert_eq!(Solution::new(43), solution);
    }
}
