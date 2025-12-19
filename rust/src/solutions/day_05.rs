use crate::solvable::{Solution, Solvable};

pub struct Day05;

impl Solvable for Day05 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let (ranges, available) = input.split_once("\n\n").unwrap();
        let ranges = Ranges::new_from_lines(ranges).merged();

        let solution = available
            .lines()
            .filter_map(|s| s.parse::<usize>().ok())
            .filter(|&id| ranges.contains(id))
            .count() as i128;

        Solution::new(solution)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        let (ranges, _) = input.split_once("\n\n").unwrap();
        let ranges = Ranges::new_from_lines(ranges).merged();

        let solution = ranges.count() as i128;

        Solution::new(solution)
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn count(&self) -> usize {
        self.to - self.from + 1
    }
}

#[derive(Debug, Clone)]
struct Ranges(Vec<Range>);

impl Ranges {
    fn new_from_lines(input: &str) -> Self {
        let v = input
            .lines()
            .map(|line| {
                let (from_s, to_s) = line.split_once('-').unwrap();

                let from = from_s.parse::<usize>().unwrap();
                let to = to_s.parse::<usize>().unwrap();

                Range { from, to }
            })
            .collect();

        Self(v)
    }

    fn merged(mut self) -> Self {
        self.0.sort_by_key(|r| r.from);

        let mut merged: Vec<Range> = vec![];

        for r in self.0 {
            if let Some(last) = merged.last_mut()
                && r.from <= last.to + 1
            {
                last.to = last.to.max(r.to);
                continue;
            }
            merged.push(r);
        }

        Self(merged)
    }

    fn contains(&self, x: usize) -> bool {
        let idx = self.0.partition_point(|r| r.from <= x);

        if idx == 0 {
            return false;
        }

        let r = self.0[idx - 1];
        x >= r.from && x <= r.to
    }

    fn count(&self) -> usize {
        self.0.iter().map(Range::count).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_day_05_first_task() {
        let day_05 = Day05;
        let solution = day_05.first(INPUT);
        assert_eq!(Solution::new(3), solution);
    }

    #[test]
    fn test_day_05_second_task() {
        let day_05 = Day05;
        let solution = day_05.second(INPUT);
        assert_eq!(Solution::new(14), solution);
    }
}
