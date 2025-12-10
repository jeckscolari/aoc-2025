use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use crate::days::Solution;
use crate::matrix::Matrix;

const START: char = 'S';
const EMPTY: char = '.';
const SPLITTER: char = '^';
const BEAM: char = '|';

pub struct Day07 {
    matrix: Matrix<char>,
    start: (usize, usize),
}

impl FromStr for Day07 {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let matrix = Matrix::new(input.lines().map(|line| line.chars().collect()).collect());

        let start = matrix
            .iter_coords()
            .find(|&(row, col)| matrix.get(row, col) == Some(&START))
            .unwrap();
        Ok(Day07 { matrix, start })
    }
}

impl Day07 {
    fn simulate(&self) -> usize {
        let mut matrix = self.matrix.clone();
        let mut split_count = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut beams: VecDeque<(usize, usize)> = VecDeque::new();
        beams.push_back(self.start);
        visited.insert(self.start);

        while let Some((mut row, col)) = beams.pop_front() {
            loop {
                if let Some(cell) = matrix.get_mut(row, col)
                    && *cell == EMPTY
                {
                    *cell = BEAM;
                }

                visited.insert((row, col));

                let next_row = row + 1;
                if next_row >= matrix.rows {
                    break;
                }

                match matrix.get(next_row, col) {
                    Some(&SPLITTER) => {
                        split_count += 1;

                        if col > 0 && visited.insert((next_row, col - 1)) {
                            beams.push_back((next_row, col - 1));
                        }

                        if col + 1 < matrix.cols && visited.insert((next_row, col + 1)) {
                            beams.push_back((next_row, col + 1));
                        }

                        break;
                    }
                    Some(&EMPTY) | Some(&BEAM) => {
                        row = next_row;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        split_count
    }

    fn simulate_quantum(&self) -> usize {
        let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
        self.count_timelines(self.start, &mut memo)
    }

    fn count_timelines(
        &self,
        cell: (usize, usize),
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(&count) = memo.get(&cell) {
            return count;
        }

        let mut row = cell.0;
        let col = cell.1;

        loop {
            let next_row = row + 1;
            if next_row >= self.matrix.rows {
                memo.insert(cell, 1);
                return 1;
            }

            match self.matrix.get(next_row, col) {
                Some(&SPLITTER) => {
                    let left = if col > 0 {
                        self.count_timelines((next_row, col - 1), memo)
                    } else {
                        0
                    };
                    let right = if col + 1 < self.matrix.cols {
                        self.count_timelines((next_row, col + 1), memo)
                    } else {
                        0
                    };
                    let total = left + right;
                    memo.insert(cell, total);
                    return total;
                }
                Some(&EMPTY) | Some(&START) => {
                    row = next_row;
                }
                _ => {
                    memo.insert(cell, 1);
                    return 1;
                }
            }
        }
    }
}

impl Solution for Day07 {
    fn part1(&self) -> String {
        self.simulate().to_string()
    }

    fn part2(&self) -> String {
        self.simulate_quantum().to_string()
    }
}
