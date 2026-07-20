#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    fn place(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn new(rows: usize, cols: usize) -> Self {
        let size = rows * cols;
        let grid = vec![T::default(); size];
        Self { rows, cols, grid }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self { rows, cols, grid: grid.to_vec() }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[self.place(row, col)]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        let idx = self.place(row, col);
        self.grid[idx] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let rd = if row == 0 {0} else {row - 1};
        let cl = if col == 0 {0} else {col - 1};
        let rh = if row == self.rows - 1 {row + 1} else {row + 2};
        let cr = if col == self.cols - 1 {col + 1} else {col + 2};
        let mut res: Vec<(usize, usize)> = Vec::with_capacity((rh - rd) * (cr - cl) - 1);
        for i in rd..rh {
            for j in cl..cr {
                if i != row || j != col {
                    res.push((i, j));
                }
            }
        }
        res
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let grid = &mut self.grid;
        let mut change_alive: Vec<(usize, usize)> = Vec::new();
        let mut change_dead: Vec<(usize, usize)> = Vec::new();
        for i in 0..grid.rows {
            for j in 0..grid.cols {
                let neighbours = grid.neighbours(i, j);
                let mut n_alive = 0u16;
                for (cr, cl) in neighbours {
                    if grid.get(cr, cl) == &Cell::Alive {n_alive += 1;}
                }
                if grid.get(i, j) == &Cell::Alive && (n_alive < 2 || n_alive > 3) {
                    change_alive.push((i, j));
                }
                if grid.get(i, j) == &Cell::Dead && n_alive == 3 {
                    change_dead.push((i, j));
                }
            }
        }
        for (cr, cl) in change_alive {
            grid.set(Cell::Dead, cr, cl);
        }
        for (cr, cl) in change_dead {
            grid.set(Cell::Alive, cr, cl);
        }
    }
}
