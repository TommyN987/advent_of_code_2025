use std::fmt::Display;

use crate::solutions::{Day01, Day02, Day03, Day04, Day05};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution(i128);

impl Solution {
    pub fn new(num: i128) -> Self {
        Self(num)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The solution is {}", self.0)
    }
}

pub trait Solvable {
    fn first(&self, input: &str) -> Solution;
    fn second(&self, input: &str) -> Solution;
}

pub struct Registry {
    solvers: Vec<Box<dyn Solvable>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut solvers: Vec<Box<dyn Solvable>> = Vec::with_capacity(25);
        solvers.push(Box::new(Day01));
        solvers.push(Box::new(Day02));
        solvers.push(Box::new(Day03));
        solvers.push(Box::new(Day04));
        solvers.push(Box::new(Day05));

        Self { solvers }
    }

    pub fn solve(&self, inputs: &[String]) -> Vec<(Solution, Solution)> {
        self.solvers
            .iter()
            .zip(inputs)
            .map(|(solver, input)| (solver.first(input), solver.second(input)))
            .collect()
    }
}
