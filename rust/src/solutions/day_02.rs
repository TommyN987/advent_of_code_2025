use crate::solvable::{Solution, Solvable};

struct Range {
    min: i64,
    max: i64,
}

pub struct Day02;

impl Solvable for Day02 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let sum = parse_ranges(input)
            .flat_map(|r| r.min..=r.max)
            .filter(|&n| is_double_repeat(n))
            .sum::<i64>();

        Solution::new(sum as i128)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        let sum = parse_ranges(input)
            .flat_map(|r| r.min..=r.max)
            .filter(|&n| is_repeated_at_least_twice(n))
            .sum::<i64>();

        Solution::new(sum as i128)
    }
}

fn parse_ranges(input: &str) -> impl Iterator<Item = Range> + '_ {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range| {
            let (a, b) = range.split_once('-').expect("Each range must contain '-'");
            Range {
                min: a.parse().unwrap(),
                max: b.parse().unwrap(),
            }
        })
}

fn is_double_repeat(n: i64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let mid = s.len() / 2;
    s[..mid] == s[mid..]
}

fn is_repeated_at_least_twice(n: i64) -> bool {
    let s = n.to_string();

    (1..=s.len() / 2)
        .filter(|&pat_len| s.len() % pat_len == 0)
        .any(|pat_len| {
            let reps = s.len() / pat_len;
            let pat = &s[..pat_len];
            pat.repeat(reps) == s
        })
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_day_02_first_task() {
        let day_02 = Day02;
        let solution = day_02.first(INPUT);
        assert_eq!(Solution::new(1227775554), solution);
    }

    #[test]
    fn test_day_02_second_task() {
        let day_02 = Day02;
        let solution = day_02.second(INPUT);
        assert_eq!(Solution::new(4174379265), solution);
    }
}
