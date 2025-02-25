fn main() {
    let elements: Vec<usize> = (0..5).collect();
    let mut buff: Vec<usize> = vec![0; elements.len()];
    permutation(&elements, &mut buff, 0);
}

fn permutation(elements: &[usize], buff: &mut [usize], level: usize) {
    if level == elements.len() {
        println!("{:?}", buff);
    }
    for i in elements {
        if !buff[0..level].contains(i) {
            buff[level] = *i;
            permutation(elements, buff, level + 1);
        }
    }
}
