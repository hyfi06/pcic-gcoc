use std::time::Instant;

mod tablero;

fn main() {
    let n = 8;
    let mut buff = vec![0; n];
    let mut solutions: i32 = 0;
    let start = Instant::now();
    let mut nodes = 0;
    nq(&mut buff, 0, &mut solutions, &mut nodes);
    let elapse = Instant::elapsed(&start);
    println!("Visited {nodes}");
    println!("{n} reinas: {} soluciones en {:?}", solutions, elapse);

    let start = Instant::now();
    let (solutions, nodes) = nq_iter(n);
    let elapse = Instant::elapsed(&start);
    println!("Visited {nodes}");
    println!("{n} reinas: {} soluciones en {:?}", solutions, elapse);
}

fn nq(buff: &mut [usize], k: usize, solutions: &mut i32, nodes: &mut i32) {
    *nodes += 1;
    let n = buff.len();
    if k == n {
        *solutions += 1;
        return;
    }
    for i in 0..n {
        buff[k] = i;
        if is_valid(buff, k) {
            nq(buff, k + 1, solutions, nodes);
        }
    }
    buff[k] = 0;
}

fn nq_iter(n: usize) -> (i32, i32) {
    let mut buff = vec![0; n];
    let mut solutions = 0;
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut nodes = 0;
    stack.push((0, 0));
    while let Some((col, row)) = stack.pop() {
        nodes += 1;
        if col == n - 1 {
            solutions += 1;
            continue;
        }
        for i in row + 1..n {
            buff[col] = i;
            if is_valid(&buff, col) {
                stack.push((col, i));
                break;
            }
        }
        buff[col] = row;
        for i in 0..n {
            buff[col + 1] = i;
            if is_valid(&buff, col + 1) {
                stack.push((col + 1, i));
                break;
            }
        }
    }
    (solutions, nodes)
}

fn is_valid(buff: &[usize], k: usize) -> bool {
    let mut valid = true;
    for i in 0..k {
        if buff[i] == buff[k] || (buff[i] as isize - buff[k] as isize).abs() == (k - i) as isize {
            valid = false;
            break;
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
                print!("* ");
            }
        }
        println!();
    }
}
