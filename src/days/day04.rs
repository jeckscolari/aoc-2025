use crate::days::Solution;
use crate::matrix::Matrix;

const ROLL: char = '@';
const EMPTY: char = '.';

pub struct Day04 {
    matrix: Matrix<char>,
}

fn is_accessible(matrix: &Matrix<char>, row: usize, col: usize) -> bool {
    matrix.get(row, col) == Some(&ROLL)
        && matrix
            .neighbors(row, col)
            .filter(|&(r, c)| matrix.get(r, c) == Some(&ROLL))
            .count()
            < 4
}

impl Solution for Day04 {
    fn from_str(input: &str) -> Self {
        let matrix = Matrix::new(input.lines().map(|line| line.chars().collect()).collect());
        Day04 { matrix }
    }

    fn part1(&self) -> String {
        self.matrix
            .iter_coords()
            .filter(|&(row, col)| is_accessible(&self.matrix, row, col))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut matrix = self.matrix.clone();
        let mut count = 0;

        loop {
            let to_remove: Vec<_> = matrix
                .iter_coords()
                .filter(|&(row, col)| is_accessible(&matrix, row, col))
                .collect();

            if to_remove.is_empty() {
                break;
            }

            for (row, col) in to_remove {
                *matrix.get_mut(row, col).unwrap() = EMPTY;
                count += 1;
            }
        }

        count.to_string()
    }
}
