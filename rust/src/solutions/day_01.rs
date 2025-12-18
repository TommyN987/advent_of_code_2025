use core::str;

use crate::solvable::{Solution, Solvable};

pub struct Day01;

struct Rotation {
    dir: u8,
    dist: isize,
}

impl Solvable for Day01 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        let count = parse_rotations(input)
            .scan(50isize, |pos, r| {
                *pos = match r.dir {
                    b'L' => (*pos - r.dist).rem_euclid(LOCK_SIZE),
                    b'R' => (*pos + r.dist).rem_euclid(LOCK_SIZE),
                    _ => unreachable!(),
                };
                Some(*pos)
            })
            .filter(|&pos| pos == 0)
            .count() as i128;

        Solution::new(count)
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        let count = parse_rotations(input)
            .fold((50isize, 0isize), |(pos, total), r| {
                let hits = hits_zero_during_rotation(pos, &r);

                let new_pos = match r.dir {
                    b'R' => (pos + r.dist).rem_euclid(LOCK_SIZE),
                    b'L' => (pos - r.dist).rem_euclid(LOCK_SIZE),
                    _ => unreachable!(),
                };

                (new_pos, total + hits)
            })
            .1 as i128;

        Solution::new(count)
    }
}

fn parse_rotations(input: &str) -> impl Iterator<Item = Rotation> + '_ {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (d, n) = line.split_at(1);
            Rotation {
                dir: d.as_bytes()[0],
                dist: n.parse().unwrap(),
            }
        })
}

fn hits_zero_during_rotation(pos: isize, rotation: &Rotation) -> isize {
    if rotation.dist <= 0 {
        return 0;
    }

    let target = match rotation.dir {
        b'R' => (-pos).rem_euclid(LOCK_SIZE),
        b'L' => pos.rem_euclid(LOCK_SIZE),
        _ => unreachable!(),
    };

    let first = if target == 0 { LOCK_SIZE } else { target };

    if rotation.dist < first {
        0
    } else {
        1 + (rotation.dist - first) / LOCK_SIZE
    }
}

const LOCK_SIZE: isize = 100;

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_day_01_first_task() {
        let day_01 = Day01;
        let solution = day_01.first(INPUT);
        assert_eq!(Solution::new(3), solution);
    }

    #[test]
    fn test_day_01_second_task() {
        let day_01 = Day01;
        let solution = day_01.second(INPUT);
        assert_eq!(Solution::new(6), solution);
    }
}
