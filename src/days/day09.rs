use std::str::FromStr;

use crate::days::Solution;

struct Tile {
    x: i64,
    y: i64,
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Tile {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

pub struct Day09 {
    tiles: Vec<Tile>,
}

impl FromStr for Day09 {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tiles = input
            .lines()
            .map(|line| line.parse::<Tile>().unwrap())
            .collect();
        Ok(Day09 { tiles })
    }
}

impl Tile {
    fn area(&self, other: &Tile) -> i64 {
        let width = (self.x - other.x).abs() + 1;
        let height = (self.y - other.y).abs() + 1;
        width * height
    }
}

impl Solution for Day09 {
    fn part1(&self) -> String {
        let mut max_area = 0;
        for i in 0..self.tiles.len() {
            for j in i + 1..self.tiles.len() {
                max_area = max_area.max(self.tiles[i].area(&self.tiles[j]));
            }
        }
        max_area.to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}
