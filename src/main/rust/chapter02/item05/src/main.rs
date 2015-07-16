use std::io::{self, Write};

fn main() {
    let mut sum = 0u64;
    for i in (0..std::i32::MAX) {
        sum += i as u64;
    }

    writeln!(io::stdout(), "{}", sum).ok();
}