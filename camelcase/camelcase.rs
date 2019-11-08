use std::io;
use std::io::Read;

fn main() {
    println!(
        "{}",
        io::stdin()
            .bytes()
            .fold(0, |acc, b| acc + ((!b.unwrap() & 0x20) >> 5))
    );
}
