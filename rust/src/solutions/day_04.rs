use std::{collections::HashSet, ops::Add};

use crate::solvable::{Solution, Solvable};

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    const NORTH: Self = Self { x: 0, y: -1 };
    const SOUTH: Self = Self { x: 0, y: 1 };
    const EAST: Self = Self { x: 1, y: 0 };
    const WEST: Self = Self { x: -1, y: 0 };

    fn neighbors(self) -> [Self; 8] {
        [
            self + Self::NORTH + Self::WEST,
            self + Self::NORTH,
            self + Self::NORTH + Self::EAST,
            self + Self::EAST,
            self + Self::SOUTH + Self::EAST,
            self + Self::SOUTH,
            self + Self::SOUTH + Self::WEST,
            self + Self::WEST,
        ]
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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
