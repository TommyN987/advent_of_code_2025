use crate::solvable::{Solution, Solvable};

pub struct Day03;

impl Solvable for Day03 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let solution = input.lines().map(max_pair_value).sum();

        Solution::new(solution)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        let solution = input.lines().map(max_12).sum();

        Solution::new(solution)
    }
}

fn max_pair_value(line: &str) -> i128 {
    let (best, _best_right) =
        line.bytes()
            .rev()
            .fold((0i128, None::<i128>), |(best, best_right), b| {
                let d = (b - b'0') as i128;
                let best = match best_right {
                    Some(r) => best.max(10 * d + r),
                    None => best,
                };

                let best_right = Some(best_right.map_or(d, |r| r.max(d)));
                (best, best_right)
            });
    best
}

fn max_12(line: &str) -> i128 {
    let n = line.len();

    let mut remove = n - 12;
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    for b in line.bytes() {
        while remove > 0 && !stack.is_empty() && *stack.last().unwrap() < b {
            stack.pop();
            remove -= 1;
        }
        stack.push(b);
    }

    while remove > 0 {
        stack.pop();
        remove -= 1;
    }

    stack.truncate(12);

    String::from_utf8(stack)
        .expect("only digits")
        .parse::<i128>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_day_03_first_task() {
        let day_03 = Day03;
        let solution = day_03.first(INPUT);
        assert_eq!(Solution::new(357), solution);
    }

    #[test]
    fn test_day_03_second_task() {
        let day_03 = Day03;
        let solution = day_03.second(INPUT);
        assert_eq!(Solution::new(3121910778619), solution);
    }
}
