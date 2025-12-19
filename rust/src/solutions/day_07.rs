use std::collections::{HashSet, VecDeque};

use crate::solvable::{Solution, Solvable};

pub struct Day07;

impl Solvable for Day07 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let mut simulation = Simulation::parse(input);

        for r in (simulation.start_row + 1)..simulation.h {
            simulation.resolve_row_splitters(r);

            if simulation.active_beams.is_empty() {
                break;
            }
        }

        Solution::new(simulation.split_count)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        let mut finished = 0i128;
        let mut simulation = Simulation::parse(input);

        for r in (simulation.start_row + 1)..simulation.h {
            simulation.resolve_row_splitters_quantum(r, &mut finished);

            if simulation.active_timelines.iter().all(|&x| x == 0) {
                break;
            }
        }

        Solution::new(finished + simulation.active_timelines.iter().copied().sum::<i128>())
    }
}

#[derive(Debug)]
struct Simulation {
    cells: Vec<Vec<u8>>,
    h: usize,
    w: usize,
    start_row: usize,
    active_beams: HashSet<usize>,
    active_timelines: Vec<i128>,
    split_count: i128,
}

impl Simulation {
    fn parse(input: &str) -> Self {
        let cells: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

        let h = cells.len();
        let w = cells.first().map_or(0, Vec::len);

        let mut start_row = None;
        let mut start_col = None;

        for (r, row) in cells.iter().enumerate() {
            assert!(row.len() == w);

            if let Some(c) = row.iter().position(|&b| b == b'S') {
                start_row = Some(r);
                start_col = Some(c);
                break;
            }
        }

        let (start_row, start_col) = (
            start_row.expect("No 'S' found"),
            start_col.expect("No 'S' found"),
        );

        let active_beams = HashSet::from_iter([start_col]);
        let mut active_timelines = vec![0i128; w];
        active_timelines[start_col] = 1;

        Self {
            cells,
            h,
            w,
            start_row,
            active_beams,
            active_timelines,
            split_count: 0,
        }
    }

    fn at(&self, r: usize, c: usize) -> u8 {
        self.cells[r][c]
    }

    fn in_bounds_col(&self, c: isize) -> bool {
        c >= 0 && (c as usize) < self.w
    }

    fn resolve_row_splitters(&mut self, r: usize) {
        loop {
            let splitter_col = self
                .active_beams
                .iter()
                .copied()
                .find(|&c| self.at(r, c) == b'^');

            let Some(c) = splitter_col else { break };

            self.active_beams.remove(&c);

            self.split_count += 1;

            let left = c as isize - 1;
            let right = c as isize + 1;

            if self.in_bounds_col(left) {
                self.active_beams.insert(left as usize);
            }
            if self.in_bounds_col(right) {
                self.active_beams.insert(right as usize);
            }
        }
    }

    fn resolve_row_splitters_quantum(&mut self, r: usize, finished: &mut i128) {
        let mut queue: VecDeque<usize> = VecDeque::new();

        for c in 0..self.w {
            if self.active_timelines[c] > 0 && self.at(r, c) == b'^' {
                queue.push_back(c);
            }
        }

        while let Some(c) = queue.pop_front() {
            let t = self.active_timelines[c];
            if t == 0 {
                continue;
            }

            if self.at(r, c) != b'^' {
                continue;
            }

            self.active_timelines[c] = 0;

            let left = c as isize - 1;
            let right = c as isize + 1;

            if self.in_bounds_col(left) {
                let lc = left as usize;
                self.active_timelines[lc] = self.active_timelines[lc]
                    .checked_add(t)
                    .expect("Timeline count overflow. Use BigUint");

                if self.at(r, lc) == b'^' {
                    queue.push_back(lc);
                }
            } else {
                *finished = finished
                    .checked_add(t)
                    .expect("Timeline count overflow. Use BigUint");
            }

            if self.in_bounds_col(right) {
                let rc = right as usize;
                self.active_timelines[rc] = self.active_timelines[rc]
                    .checked_add(t)
                    .expect("Timeline count overflow. Use BigUint");

                if self.at(r, rc) == b'^' {
                    queue.push_back(rc);
                }
            } else {
                *finished = finished
                    .checked_add(t)
                    .expect("Timeline count overflow. Use BigUint");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_day_07_first_task() {
        let day_07 = Day07;
        let solution = day_07.first(INPUT);
        assert_eq!(Solution::new(21), solution);
    }

    #[test]
    fn test_day_07_second_task() {
        let day_07 = Day07;
        let solution = day_07.second(INPUT);
        assert_eq!(Solution::new(40), solution);
    }
}
