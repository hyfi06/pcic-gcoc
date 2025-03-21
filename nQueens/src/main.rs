use std::time::Instant;

use nqueens::NQueens;

mod board;
mod nqueens;

fn main() {
    let mut nq = NQueens::new();

    let start = Instant::now();
    nq.backtrack(0);
    let elapse = Instant::elapsed(&start);
    println!(
        "{} reinas: {} soluciones en {:.2?}",
        nqueens::N,
        nq.seen_leaves,
        elapse
    );

    let start = Instant::now();
    let seen_leaves = NQueens::run_parallel(20);
    let elapse = Instant::elapsed(&start);
    println!(
        "{} reinas: {} soluciones en {:.2?}",
        nqueens::N,
        seen_leaves,
        elapse
    );
}
