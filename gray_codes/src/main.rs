fn main() {
    // let vec:Vec<usize> = grayCodeIter(3).collect();
    // println!("{:?}",vec);
    let gray_code:usize = 0b010;
    let max_num:usize = 1<<3;
    println!("{:b}", gray_code);
    println!("{:b}", (max_num - gray_code));
    println!("{:b}", gray_code & (max_num - gray_code));
    let lsb:usize = gray_code & (max_num - gray_code);
    println!("{:b}", gray_code ^ lsb);
    println!("{:b}", gray_code ^ (lsb<<1));
}

pub fn grayCodeIter(n: usize) -> impl Iterator<Item = usize> {
    let max_num: usize = 1 << n;
    let mut gray_code: usize = 0;
    return std::iter::from_fn(move || {
        if gray_code >= max_num { return None };
        let result = gray_code.clone();
        let lsb: usize = gray_code & (max_num - 1 - gray_code);
        println!("{:b}",lsb);
        if !gray_code % 2 == 0 {
            gray_code ^=  lsb;
        } else {
            gray_code ^= lsb << 1
        }

        return Some(result);
    });
}
