fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();

    let lines: Vec<Vec<u32>> = stdin
        .lock()
        .lines()
        .skip(1)
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .flat_map(str::parse)
                .collect()
        })
        .collect();

    let arr1 = &lines[0];
    let arr2 = &lines[1];

    for &i in arr1.iter() {
        let count = arr2.iter().filter(|&&j| j <= i).count();
        println!("{}", count);
    }
}
