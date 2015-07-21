use std::i32;
use std::cmp;

fn main() {
    let ints = [2i32, 7, 1, 8, 2, 8, 1, 8, 2, 8];

    println!("{:?}", ints.iter().fold(0, |base, item| base + item));
    println!("{:?}", ints.iter().fold(1, |base, item| base * item));
    println!("{:?}", ints.iter().fold(i32::MIN, |base, item| cmp::max(base, *item)));
    println!("{:?}", ints.iter().fold(i32::MAX, |base, item| cmp::min(base, *item)));
}