use std::str::FromStr;

use crate::days::Solution;

const INITIAL_DIAL: i32 = 50;

pub struct Day01 {
    rotations: Vec<Rotation>,
}

enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {s}")),
        }
    }
}

struct Rotation {
    direction: Direction,
    distance: i32,
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, dist_str) = s.split_at(1);
        let direction = dir_str.parse()?;
        let distance = dist_str
            .parse()
            .map_err(|e| format!("Invalid distance: {e}"))?;
        Ok(Rotation {
            direction,
            distance,
        })
    }
}

impl FromStr for Day01 {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rotations = input
            .lines()
            .map(|line| line.parse::<Rotation>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Day01 { rotations })
    }
}

impl Solution for Day01 {
    fn part1(&self) -> String {
        let mut dial = INITIAL_DIAL;
        let mut count = 0;

        for rotation in &self.rotations {
            dial = match rotation.direction {
                Direction::Left => (dial - rotation.distance).rem_euclid(100),
                Direction::Right => (dial + rotation.distance) % 100,
            };

            if dial == 0 {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part2(&self) -> String {
        let mut dial = INITIAL_DIAL;
        let mut count = 0;

        for rotation in &self.rotations {
            for _ in 0..rotation.distance {
                dial = match rotation.direction {
                    Direction::Left => (dial + 99) % 100,
                    Direction::Right => (dial + 1) % 100,
                };

                if dial == 0 {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
}
