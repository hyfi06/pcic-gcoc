use std::{ops::Not, time::Instant};

fn main() {
    let n = 9;
    let mut buff = vec![0; n];
    let mut solutions: i32 = 0;
    let start = Instant::now();
    nq(&mut buff, 0, &mut solutions);
    let elapse = Instant::elapsed(&start);
    println!("{n} reinas: {} soluciones en {:?}", solutions, elapse);

    let start = Instant::now();
    solutions = nq_iter(n);
    let elapse = Instant::elapsed(&start);
    println!("{n} reinas: {} soluciones en {:?}", solutions, elapse);
}

fn nq(buff: &mut [usize], k: usize, solutions: &mut i32) {
    let n = buff.len();
    if k == n {
        *solutions += 1;
        return;
    }
    for i in 0..n {
        if !buff[0..k].contains(&i) {
            buff[k] = i;
            if is_valid(buff, k) {
                nq(buff, k + 1, solutions);
            }
        }
    }
    buff[k] = 0;
}

fn nq_iter(n: usize) -> i32 {
    let mut buff = vec![0; n];
    let mut solutions = 0;
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((0, 0));
    while let Some((col, row)) = stack.pop() {
        if col == n - 1 {
            solutions += 1;
            continue;
        }
        for i in row..n {
            if !buff[0..col].contains(&i) {
                buff[col] = i;
                if is_valid(&buff, col) {
                    stack.push((col, i));
                    break;
                }
            }
        }
        buff[col] = row;
        for i in 0..n {
            if !buff[0..col + 1].contains(&i) {
                buff[col + 1] = i;
                if is_valid(&buff, col + 1) {
                    stack.push((col + 1, i));
                    break;
                }
            }
        }
    }
    solutions
}

fn is_valid(buff: &[usize], k: usize) -> bool {
    let mut valid = true;
    for i in 0..k {
        if buff[i] == buff[k] || (buff[i] as isize - buff[k] as isize).abs() == (k - i) as isize {
            valid = false;
        }
    }
    valid
}

fn tablero(buff: &[usize]) {
    let n = buff.len(); // Tamaño del tablero
    for i in 0..n {
        for j in 0..n {
            if buff[i] == j {
                print!("Q "); // Reina en la columna especificada
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
