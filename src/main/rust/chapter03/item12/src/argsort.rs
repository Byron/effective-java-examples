use std::collections::BTreeSet;
use std::env;

fn main() {
    let m: BTreeSet<_> = env::args().collect();
    println!("{:#?}", m);
}