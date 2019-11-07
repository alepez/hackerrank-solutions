use std::io::BufRead;

fn roman_number(mut n: u32) -> String {
    const TABLE: [(u32, char); 7] = [
        (1000, 'M'),
        (500, 'D'),
        (100, 'C'),
        (50, 'L'),
        (10, 'X'),
        (5, 'V'),
        (1, 'I'),
    ];

    let find_next_sub = |y: u32| TABLE.iter().find(|&&x| x.0 == y / 10).unwrap();

    let mut txt = String::new();

    /* This is the current power of ten to subtract */
    let mut u = find_next_sub(TABLE[0].0);

    for &(mut d, s) in TABLE.iter() {
        /* Subtract the number from table and add to string
         * the relative character until n is big enough.
         */
        while n >= d {
            txt.push(s);
            n -= d;
        }

        /* No more to subtract */
        if d == 1 {
            break;
        }

        if d == u.0 {
            u = find_next_sub(d);
        }

        /* Try with the number on the table minus the
         * power of ten in the same order of magnitude.
         */
        d = d - u.0;

        if n >= d {
            txt.push(u.1);
            txt.push(s);
            n -= d;
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
    println!("{}", roman_number(1999) == "MCMXCIX");
    println!("{}", roman_number(7) == "VII");
}
