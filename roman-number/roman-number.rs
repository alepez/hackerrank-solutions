use std::io::BufRead;

fn roman_number(mut n: u32) -> String {
    const TABLE: [(u32, &'static str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut txt = String::new();

    for (d, s) in TABLE.iter() {
        while n >= *d {
            n -= d;
            txt.push_str(s);
        }
    }

    txt
}

fn main() {
    let stdin = std::io::stdin();

    let n: u32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    let result = roman_number(n);

    println!("{}", result);
}
