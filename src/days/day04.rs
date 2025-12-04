use crate::days::Solution;
use crate::matrix::Matrix;

pub struct Day04 {
    matrix: Matrix<char>,
}

fn is_accessible(matrix: &Matrix<char>, row: usize, col: usize) -> bool {
    matrix.get(row, col) == Some(&'@')
        && matrix
            .neighbors(row, col)
            .filter(|&(r, c)| matrix.get(r, c) == Some(&'@'))
            .count()
            < 4
}

impl Solution for Day04 {
    fn from_str(input: &str) -> Self {
        let matrix = Matrix::new(input.lines().map(|line| line.chars().collect()).collect());
        Day04 { matrix }
    }

    fn part1(&self) -> String {
        let mut count = 0;
        for row in 0..self.matrix.rows {
            for col in 0..self.matrix.cols {
                if is_accessible(&self.matrix, row, col) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part2(&self) -> String {
        let mut matrix = self.matrix.clone();
        let mut total = 0;

        loop {
            let mut to_remove = vec![];
            for row in 0..matrix.rows {
                for col in 0..matrix.cols {
                    if is_accessible(&matrix, row, col) {
                        to_remove.push((row, col));
                    }
                }
            }

            if to_remove.is_empty() {
                break;
            }

            for (row, col) in to_remove {
                *matrix.get_mut(row, col).unwrap() = '.';
                total += 1;
            }
        }

        total.to_string()
    }
}
