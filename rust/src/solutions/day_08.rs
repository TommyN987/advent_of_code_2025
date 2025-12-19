use std::collections::HashMap;

use crate::solvable::{Solution, Solvable};

pub struct Day08;

impl Solvable for Day08 {
    fn first(&self, input: &str) -> crate::solvable::Solution {
        Solution::new(solve_first(input, 1000))
    }

    fn second(&self, input: &str) -> crate::solvable::Solution {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn parse(line: &str) -> Self {
        let mut it = line.split(',');
        let x = it.next().unwrap().trim().parse::<i32>().unwrap();
        let y = it.next().unwrap().trim().parse::<i32>().unwrap();
        let z = it.next().unwrap().trim().parse::<i32>().unwrap();

        Self { x, y, z }
    }

    fn dist_to(self, other: Self) -> u128 {
        let dx = (self.x as i128) - (other.x as i128);
        let dy = (self.y as i128) - (other.y as i128);
        let dz = (self.z as i128) - (other.z as i128);
        (dx * dx + dy * dy + dz * dz) as u128
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
    dist: u128,
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];

        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut map: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let r = self.find(i);
            *map.entry(r).or_insert(0) += 1;
        }
        map.into_values().collect()
    }
}

fn solve_first(input: &str, k: usize) -> i128 {
    let points: Vec<Point3D> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Point3D::parse)
        .collect();

    let n = points.len();

    let mut edges: Vec<Edge> = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                a: i,
                b: j,
                dist: points[i].dist_to(points[j]),
            });
        }
    }

    edges.sort_unstable_by(|e1, e2| (e1.dist, e1.a, e1.b).cmp(&(e2.dist, e2.a, e2.b)));

    let mut uf = UnionFind::new(n);

    for e in edges.into_iter().take(k) {
        uf.union(e.a, e.b);
    }

    let mut sizes = uf.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let top3 = sizes
        .into_iter()
        .take(3)
        .map(|x| x as i128)
        .collect::<Vec<_>>();

    top3.into_iter().product()
}

#[cfg(test)]
mod tests {
    use crate::solvable::Solution;

    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_day_08_first_task() {
        let solution = Solution::new(solve_first(INPUT, 10));
        assert_eq!(Solution::new(40), solution);
    }

    #[test]
    fn test_day_08_second_task() {
        let day_08 = Day08;
        let solution = day_08.second(INPUT);
        assert_eq!(Solution::new(25272), solution);
    }
}
