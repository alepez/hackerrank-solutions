use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", 1 + input.chars().filter(|&x| x.is_uppercase()).count());
}
