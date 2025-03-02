use std::time::Instant;

fn main() {
    let n = 14;
    let mut buff = vec![0; n];
    let mut solutions: i32 = 0;
    let start = Instant::now();
    nq(&mut buff, 0, &mut solutions);
    let elapse = Instant::elapsed(&start);
    // for sol in solutions.iter() {
    //     println!("{:?}", sol);
    //     tablero(sol);
    // }
    println!("{n} reinas: {} soluciones en {:?}", solutions, elapse);
}

fn nq(buff: &mut [usize], k: usize, solutions: &mut i32) {
    let n = buff.len();
    if k == n {
        *solutions += 1;
        return;
    }
    let positions: Vec<usize> = (0..n).filter(|x| !buff[0..k].contains(x)).collect();
    for i in positions {
        buff[k] = i;
        if is_valid(buff, k) {
            nq(buff, k + 1, solutions);
        }
    }
    buff[k] = 0;
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
