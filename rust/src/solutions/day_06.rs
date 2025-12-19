use crate::solvable::{Solution, Solvable};

pub struct Day06;

impl Solvable for Day06 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let mut lines: Vec<&str> = input.lines().collect();
        let op_line = lines.pop().unwrap();

        let ops: Vec<Op> = op_line
            .split_whitespace()
            .map(|s| match s {
                "+" => Op::Add,
                "*" => Op::Mul,
                _ => unreachable!(),
            })
            .collect();

        let columns: Vec<Vec<i128>> = (0..)
            .map(|i| {
                lines
                    .iter()
                    .map(|line| line.split_whitespace().nth(i).unwrap().parse().unwrap())
                    .collect()
            })
            .take(ops.len())
            .collect();

        let problems = ops
            .into_iter()
            .zip(columns)
            .map(|(op, values)| Math { op, values })
            .collect::<Vec<_>>();

        let solution = problems
            .into_iter()
            .map(|m| match m.op {
                Op::Add => m.values.into_iter().sum::<i128>(),
                Op::Mul => m.values.into_iter().fold(1, |acc, x| acc * x),
            })
            .sum();

        Solution::new(solution)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        let worksheet = Worksheet::parse(input);

        Solution::new(worksheet.solve())
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct Math {
    op: Op,
    values: Vec<i128>,
}

#[derive(Debug, Clone, Copy)]
struct ColRange {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
struct Problem {
    cols: ColRange,
    op: Op,
}

struct Worksheet {
    grid: Vec<Vec<u8>>,
    op_row: usize,
    problems: Vec<Problem>,
}

impl Worksheet {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let op_row = height - 1;

        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(height);

        for line in &lines {
            let mut row = vec![b' '; width];
            row[..line.len()].copy_from_slice(line.as_bytes());
            grid.push(row);
        }

        let ranges = Self::find_problem_ranges(&grid, width, height);
        let problems = Self::build_problems(&grid, op_row, &ranges);

        Self {
            grid,
            op_row,
            problems,
        }
    }

    fn solve(&self) -> i128 {
        self.problems.iter().map(|p| self.evaluate_problem(p)).sum()
    }

    fn evaluate_problem(&self, p: &Problem) -> i128 {
        let values = self.read_cephalopod_numbers(p.cols);

        match p.op {
            Op::Add => values.into_iter().sum(),
            Op::Mul => values.into_iter().fold(1, |acc, x| acc * x),
        }
    }

    fn is_separator_col(grid: &[Vec<u8>], height: usize, col: usize) -> bool {
        (0..height).all(|r| grid[r][col] == b' ')
    }

    fn find_problem_ranges(grid: &[Vec<u8>], width: usize, height: usize) -> Vec<ColRange> {
        let mut ranges = Vec::new();
        let mut start = 0usize;

        for c in 0..width {
            if Self::is_separator_col(grid, height, c) {
                if start < c {
                    ranges.push(ColRange { start, end: c });
                }
                start = c + 1;
            }
        }

        if start < width {
            ranges.push(ColRange { start, end: width });
        }

        ranges
    }

    fn build_problems(grid: &[Vec<u8>], op_row: usize, ranges: &[ColRange]) -> Vec<Problem> {
        let mut problems = Vec::with_capacity(ranges.len());

        for &cols in ranges {
            let slice = &grid[op_row][cols.start..cols.end];
            let op = if slice.contains(&b'+') {
                Op::Add
            } else if slice.contains(&b'*') {
                Op::Mul
            } else {
                panic!(
                    "No operator found in problem range [{}, {}]",
                    cols.start, cols.end
                );
            };

            problems.push(Problem { cols, op });
        }

        problems
    }

    fn read_cephalopod_numbers(&self, cols: ColRange) -> Vec<i128> {
        let mut numbers = Vec::new();

        for c in (cols.start..cols.end).rev() {
            let n = self.read_number_from_column(c);
            numbers.push(n);
        }

        numbers
    }

    fn read_number_from_column(&self, col: usize) -> i128 {
        let mut digits: Vec<u8> = Vec::new();

        for r in 0..self.op_row {
            let b = self.grid[r][col];
            if b.is_ascii_digit() {
                digits.push(b);
            }
        }

        let num = str::from_utf8(&digits)
            .expect("Digits are ASCII")
            .parse::<i128>()
            .expect("Column digits form a valid integer");

        num
    }
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_day_06_first_task() {
        let day_06 = Day06;
        let solution = day_06.first(INPUT);
        assert_eq!(Solution::new(4277556), solution);
    }

    #[test]
    fn test_day_06_second_task() {
        let day_06 = Day06;
        let solution = day_06.second(INPUT);
        assert_eq!(Solution::new(3263827), solution);
    }
}
