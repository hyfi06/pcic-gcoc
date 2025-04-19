/*
000
001
011
010
110
111
101
100
 */

fn main() {
    let max_num = 1 << 4;
    (0..max_num).into_iter().for_each(|num| {
        println!("{:b}", gc_rank(num));
    });
}

pub fn gc_rank(t: usize) -> usize {
    t ^ (t >> 1)
}

pub fn gc_unrank(gray_code: usize) -> usize {
    let mut gray_code: usize = gray_code;
    let mut mask = gray_code.clone();
    while mask > 0 {
        mask >>= 1;
        gray_code ^= mask
    }
    return gray_code;
}
