use std::collections::HashMap;
use std::str::FromStr;

use crate::days::Solution;

const MAX_CONNECTIONS: usize = 1000;

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|s| s.parse().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Ok(Point { x, y, z })
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    fn sizes(&mut self) -> Vec<usize> {
        let mut size_map = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *size_map.entry(root).or_insert(0) += 1;
        }
        size_map.values().cloned().collect()
    }
}

pub struct Day08 {
    points: Vec<Point>,
}

impl FromStr for Day08 {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let points = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Day08 { points })
    }
}

impl Day08 {
    fn sorted_edges(&self) -> Vec<(usize, usize, i64)> {
        let mut edges = Vec::new();
        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                edges.push((i, j, self.points[i].distance(&self.points[j])));
            }
        }
        edges.sort_by_key(|(_, _, distance)| *distance);
        edges
    }
}

impl Solution for Day08 {
    fn part1(&self) -> String {
        let edges = self.sorted_edges();
        let mut uf = UnionFind::new(self.points.len());

        for (i, j, _) in edges.iter().take(MAX_CONNECTIONS) {
            uf.union(*i, *j);
        }

        let mut sizes = uf.sizes();
        sizes.sort();
        sizes.reverse();
        sizes.iter().take(3).product::<usize>().to_string()
    }

    fn part2(&self) -> String {
        let edges = self.sorted_edges();
        let mut uf = UnionFind::new(self.points.len());
        let mut last_connection = (0, 0);

        for (i, j, _) in &edges {
            if uf.union(*i, *j) {
                last_connection = (*i, *j);

                let root = uf.find(0);
                if uf.size[root] == self.points.len() {
                    break;
                }
            }
        }

        let (i, j) = last_connection;
        (self.points[i].x * self.points[j].x).to_string()
    }
}
