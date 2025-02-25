use std::vec;

fn main() {
    let elements: Vec<usize> = (0..8).collect();
    let mut buff: Vec<usize> = vec![0; elements.len()];
    permutation(&elements, &mut buff, 0);
}

fn permutation(elements: &[usize], buff: &mut [usize], level: usize) {
    if level == elements.len() {
        println!("{:?}", buff);
    }
    let unused: Vec<usize> = elements
        .iter()
        .filter_map(|idx| {
            if !buff[0..level].contains(idx) {
                Some(*idx)
            } else {
                None
            }
        })
        .collect();
    for i in unused {
        buff[level] = i;
        permutation(elements, buff, level + 1);
    }
}
