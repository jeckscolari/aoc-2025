const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data.first().map_or(0, |r| r.len());
        Self { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|row| row.get(col))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row).and_then(|row| row.get_mut(col))
    }

    pub fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        DIRS.into_iter().filter_map(move |(dr, dc)| {
            let new_row = row.checked_add_signed(dr)?;
            let new_col = col.checked_add_signed(dc)?;
            (new_row < self.rows && new_col < self.cols).then_some((new_row, new_col))
        })
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> {
        let cols = self.cols;
        (0..self.rows).flat_map(move |row| (0..cols).map(move |col| (row, col)))
    }
}
