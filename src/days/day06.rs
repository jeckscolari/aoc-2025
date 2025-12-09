use std::str::FromStr;

use crate::days::Solution;
use crate::matrix::Matrix;

pub struct Day06 {
    matrix: Matrix<char>,
}

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Multiply),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
        }
    }
}

impl FromStr for Matrix<char> {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = input.lines().collect();
        let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let data: Vec<Vec<char>> = lines
            .iter()
            .map(|l| {
                let mut chars: Vec<char> = l.chars().collect();
                chars.resize(max_len, ' ');
                chars
            })
            .collect();

        Ok(Matrix::new(data))
    }
}

impl FromStr for Day06 {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Day06 {
            matrix: input.parse()?,
        })
    }
}

impl Day06 {
    fn parse_left_to_right(&self) -> Vec<Problem> {
        self.split_problems(0..self.matrix.cols)
            .into_iter()
            .map(|cols| self.build_vertical(&cols))
            .collect()
    }

    fn parse_right_to_left(&self) -> Vec<Problem> {
        self.split_problems((0..self.matrix.cols).rev())
            .into_iter()
            .map(|cols| self.build_horizontal(&cols))
            .collect()
    }

    fn split_problems(&self, col_iter: impl Iterator<Item = usize>) -> Vec<Vec<Vec<char>>> {
        let mut problems = Vec::new();
        let mut current = Vec::new();

        for col in col_iter {
            let column: Vec<char> = (0..self.matrix.rows)
                .map(|row| *self.matrix.get(row, col).unwrap())
                .collect();

            if column.iter().all(|&c| c == ' ') {
                if !current.is_empty() {
                    problems.push(current);
                    current = Vec::new();
                }
            } else {
                current.push(column);
            }
        }

        if !current.is_empty() {
            problems.push(current);
        }

        problems
    }

    fn build_vertical(&self, cols: &[Vec<char>]) -> Problem {
        let operation = cols
            .iter()
            .find_map(|col| Operation::from_char(col[self.matrix.rows - 1]))
            .unwrap();

        let numbers = cols
            .iter()
            .map(|col| {
                col[..self.matrix.rows - 1]
                    .iter()
                    .filter(|&&c| c != ' ')
                    .collect::<String>()
                    .parse()
                    .unwrap()
            })
            .collect();

        Problem { numbers, operation }
    }

    fn build_horizontal(&self, cols: &[Vec<char>]) -> Problem {
        let operation = cols
            .iter()
            .find_map(|col| Operation::from_char(col[self.matrix.rows - 1]))
            .unwrap();

        let numbers = (0..self.matrix.rows - 1)
            .filter_map(|row| {
                let digits: String = cols
                    .iter()
                    .rev()
                    .map(|col| col[row])
                    .filter(|&c| c != ' ')
                    .collect();

                if digits.is_empty() {
                    None
                } else {
                    Some(digits.parse().unwrap())
                }
            })
            .collect();

        Problem { numbers, operation }
    }
}

impl Solution for Day06 {
    fn part1(&self) -> String {
        self.parse_right_to_left()
            .iter()
            .map(|p| p.solve())
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.parse_left_to_right()
            .iter()
            .map(|p| p.solve())
            .sum::<u64>()
            .to_string()
    }
}
