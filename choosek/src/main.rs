use std::collections::btree_map::RangeMut;

enum State {
    Stop,
    Continue,
}

fn choosek(
    unprocessed: &[usize],
    k: usize,
    buff: &mut [usize],
    callback: fn(&[usize]) -> State,
) -> State {
    if k == 0 {
        return callback(buff);
    }
    if unprocessed.len() < k {
        return State::Continue;
    }
    buff[buff.len() - k] = unprocessed[0];
    match choosek(&unprocessed[1..], k - 1, buff, callback) {
        State::Stop => return State::Stop,
        State::Continue => return choosek(&unprocessed[1..], k, buff, callback),
    }
}

fn n_in_k(n: usize, k: usize) -> usize {
    assert!(k < n);
    (1..n + 1).skip(n - k).reduce(|acc, x| acc * x).unwrap_or(1)
}

fn rank(choose: &[usize], k: usize, n: usize, first: usize) -> usize {
    println!("set {:?} k {} n {} f {}", choose, k, n, first);
    if choose.len() == 1 {
        return choose[0] - first;
    }
    if choose[0] == first {
        return rank(&choose[1..], k - 1, n - 1, first + 1);
    } else {
        return n_in_k(n - 1, k - 1) + rank(&choose, k, n, first + 1);
    }
}

fn main() {
    let n: usize = 5;
    let k: usize = 3;
    let mut buff: &[usize] = &[0; 3];
    choosek(&vec![0..n], 3, &mut buff, |sel| {
        println!("{} {:?}", rank(sel, 3, 5, 0), sel);
        State::Continue
    });
}
