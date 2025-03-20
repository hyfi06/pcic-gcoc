// #![allow(dead_code)]
use crate::board::Board;
use bitmaps::{Bits, BitsImpl};
use rayon::prelude::*;

pub const N: usize = 19;
const D: usize = N + N - 1;

#[derive(Debug)]
pub struct NQueens
where
    BitsImpl<N>: Bits,
    BitsImpl<D>: Bits,
{
    board: Board<N, D>,
    seen_vertices: usize,
    pub seen_leaves: usize,
    max_depth: usize,
    pool_board: Vec<Board<N, D>>,
}

impl NQueens
where
    BitsImpl<N>: Bits,
    BitsImpl<D>: Bits,
{
    pub fn new() -> Self {
        Self {
            board: Board::<N, D>::new(),
            seen_vertices: 0,
            seen_leaves: 0,
            max_depth: N.clone(),
            pool_board: vec![],
        }
    }
    pub fn backtrack(&mut self, curr_col: usize) {
        self.seen_vertices += 1;
        if self.max_depth < N && curr_col == self.max_depth {
            self.proceses_early_cut();
            return;
        }
        if curr_col == N {
            self.proceses_leaf();
            return;
        }
        for row in self.board.available_rows.clone().into_iter() {
            if self.board.is_available(curr_col, row) {
                self.board.set(curr_col, row);
                self.backtrack(curr_col + 1);
                self.board.unset(curr_col, row);
            }
        }
    }

    fn proceses_early_cut(&mut self) {
        self.pool_board.push(self.board.clone());
    }

    fn proceses_leaf(&mut self) {
        self.seen_leaves += 1;
    }
    pub fn run_parallel(multiplicity: usize) -> usize {
        let mut nq = Self::new();
        let mut level: usize = 0;
        let cpus = rayon::current_num_threads();
        println!("rayon::current_num_threads(): {}", cpus);
        for k in 1..N {
            nq = Self::new();
            nq.max_depth = k;
            nq.backtrack(0);
            if nq.pool_board.len() >= multiplicity * cpus {
                level = k;
                println!("level {k}: {}", nq.pool_board.len());
                break;
            }
        }
        nq.pool_board
            .into_par_iter()
            .map(|board| {
                let mut nq = Self::new();
                nq.board = board;
                nq.backtrack(level);
                nq.seen_leaves
            })
            .sum()
    }
}
