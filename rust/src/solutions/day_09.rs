use crate::{
    point::Point2D,
    solvable::{Solution, Solvable},
};

pub struct Day09;

impl Solvable for Day09 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let points = parse_points(input);
        let max_area = points.iter().enumerate().fold(0i128, |best, (i, p1)| {
            let best_with_p1 = points.iter().skip(i + 1).fold(0i128, |local_best, p2| {
                let area = Rect(*p1, *p2).area();
                local_best.max(area)
            });
            best.max(best_with_p1)
        });

        Solution::new(max_area)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        Solution::new(0)
    }
}

struct Rect(Point2D, Point2D);

impl Rect {
    fn area(&self) -> i128 {
        let dx = (self.0.x - self.1.x).abs() as i128;
        let dy = (self.0.y - self.1.y).abs() as i128;
        (dx + 1) * (dy + 1)
    }
}

fn parse_points(input: &str) -> Vec<Point2D> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point2D {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_day_09_first_task() {
        let day_09 = Day09;
        let solution = day_09.first(INPUT);
        assert_eq!(Solution::new(50), solution);
    }

    #[test]
    fn test_day_09_second_task() {
        let day_09 = Day09;
        let solution = day_09.second(INPUT);
        assert_eq!(Solution::new(0), solution);
    }
}
