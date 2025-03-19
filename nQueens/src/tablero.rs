#![allow(dead_code)]
use bitmaps::Bitmap;

const MY_SIZE: usize = std::mem::size_of::<usize>();

/*
   0  1  2  3
0 [ ][ ][ ][ ]
1 [ ][ ][ ][ ]
2 [ ][ ][ ][ ]
3 [ ][ ][ ][ ]

upward diagonals
0 [0][1][2][3]
1 [1][2][3][4]
2 [2][3][4][5]
3 [3][4][5][6]
   0  1  2  3

downward diagonals
0 [3][4][5][6]
1 [2][3][4][5]
2 [1][2][3][4]
3 [0][1][2][3]
   0  1  2  3
*/
pub struct Tablero {
    cols: Vec<Option<usize>>,
    available_cols: Bitmap<MY_SIZE>,
    available_rows: Bitmap<MY_SIZE>,
    available_up_diags: Bitmap<MY_SIZE>,
    available_down_diags: Bitmap<MY_SIZE>,
    size: usize,
}

impl Tablero {
    pub fn new(size: usize) -> Self {
        let diag_size = 2 * size - 1;
        Self {
            cols: vec![None; size],
            available_cols: Bitmap::mask(size),
            available_rows: Bitmap::mask(size),
            available_up_diags: Bitmap::mask(diag_size),
            available_down_diags: Bitmap::mask(diag_size),
            size,
        }
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn set(&mut self, col: usize, row: usize) {
        self.cols[col] = Some(row);
        self.available_cols.set(col, false);
        self.available_rows.set(row, false);
        self.available_up_diags.set(col + row, false);
        self.available_down_diags.set(self.size - row + col, false);
    }
    pub fn unset(&mut self, col: usize, row: usize) {
        self.cols[col] = None;
        self.available_cols.set(col, true);
        self.available_rows.set(row, true);
        self.available_up_diags.set(col + row, true);
        self.available_down_diags.set(self.size + col - row, true);
    }
    pub fn is_available(&self, col: usize, row: usize) -> bool {
        self.available_cols.get(col)
            && self.available_rows.get(row)
            && self.available_up_diags.get(col + row)
            && self.available_down_diags.get(self.size + col - row)
    }
}
