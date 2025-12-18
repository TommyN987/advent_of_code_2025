use std::fmt::Display;

pub struct Solution(i128);

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "The solution is {}", self.0)
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
