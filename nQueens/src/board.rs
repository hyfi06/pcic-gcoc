// #![allow(dead_code)]

use bitmaps::{Bitmap, Bits, BitsImpl};

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

#[derive(Clone, Debug)]
pub struct Board<const N: usize, const D: usize>
where
    BitsImpl<N>: Bits,
    BitsImpl<D>: Bits,
{
    cols: Vec<usize>,
    pub available_rows: Bitmap<N>,
    available_up_diags: Bitmap<D>,
    available_down_diags: Bitmap<D>,
}

impl<const N: usize, const D: usize> Board<N, D>
where
    BitsImpl<N>: Bits,
    BitsImpl<D>: Bits,
{
    const SIZE: usize = N;

    pub fn new() -> Self {
        let size: usize = N;
        let diag_size = size * 2 - 1;
        Self {
            cols: vec![0; size],
            available_rows: Bitmap::mask(size),
            available_up_diags: Bitmap::mask(diag_size),
            available_down_diags: Bitmap::mask(diag_size),
        }
    }
    pub fn get_size(&self) -> usize {
        Self::SIZE
    }
    pub fn set(&mut self, col: usize, row: usize) {
        self.cols[col] = row;
        self.available_rows.set(row, false);
        self.available_up_diags.set(col + row, false);
        self.available_down_diags
            .set(self.get_size() - row + col - 1, false);
    }
    pub fn unset(&mut self, col: usize, row: usize) {
        self.cols[col] = 0;
        self.available_rows.set(row, true);
        self.available_up_diags.set(col + row, true);
        self.available_down_diags
            .set(self.get_size() + col - row - 1, true);
    }
    pub fn is_available(&self, col: usize, row: usize) -> bool {
        self.available_rows.get(row)
            && self.available_up_diags.get(col + row)
            && self
                .available_down_diags
                .get(self.get_size() + col - row - 1)
    }
}

impl<const N: usize, const D: usize> std::fmt::Display for Board<N, D>
where
    BitsImpl<N>: Bits,
    BitsImpl<D>: Bits,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = N;
        for i in 0..n {
            for j in 0..n {
                if self.cols[i] == j {
                    write!(f, "[♛]")?; // Reina en la columna especificada
                } else {
                    write!(f, "[ ]")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
