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
    assert!(k <= n);
    (1..k + 1).into_iter().fold(1, |acc, curr| {
        ((acc * (n - curr + 1)) as i32 / curr as i32) as usize
    })
}

fn rank(choose: &[usize], k: usize, n: usize, first: usize) -> usize {
    if choose.len() == 1 {
        return choose[0] - first;
    }
    if choose[0] == first {
        return rank(&choose[1..], k - 1, n - 1, first + 1);
    } else {
        return n_in_k(n - 1, k - 1) + rank(&choose, k, n - 1, first + 1);
    }
}

fn unrank(i: usize, buff: &mut [usize], n: usize, k: usize, first: usize) {
    if k == 0 {
        return;
    }
    let bound = n_in_k(n - 1, k - 1);
    // println!("i {} bound {} n {} k {} first {}", i, bound, n, k, first);
    if i < bound {
        buff[0] = first;
        unrank(i, &mut buff[1..], n - 1, k - 1, first + 1);
    } else {
        unrank(i - bound, buff, n - 1, k, first + 1);
    }
}

fn main() {
    (0..6).for_each(|n_1| {
        print!("n = {} ", n_1);
        (0..n_1 + 1).for_each(|k_1| {
            print!(" {}", n_in_k(n_1, k_1));
        });
        println!("")
    });
    println!("{}", "=".repeat(20));

    let n: usize = 5;
    let k: usize = 3;
    let mut buff: Vec<usize> = vec![0; k];
    let set: Vec<usize> = (0..n).collect();
    choosek(&set, k, &mut buff, |sel| {
        println!("{} {:?}", rank(sel, 3, 5, 0), sel);
        State::Continue
    });

    println!("{}", "=".repeat(20));

    (0..n_in_k(n, k)).for_each(|i| {
        let mut buff: Vec<usize> = vec![0; k];
        unrank(i, &mut buff, n, k, 0);
        println!("{} {:?}", i, buff);
    });
}
