fn main() {
    use std::io::BufRead;

    let mut row: [[bool; 9]; 9] = Default::default();
    let mut col: [[bool; 9]; 9] = Default::default();
    let mut sub: [[[bool; 9]; 3]; 3] = Default::default();

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    for line in stdin.lines().skip(1) {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let r: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let c: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;

        if row[r][v] || col[c][v] || sub[r / 3][c / 3][v] {
            println!("WRONG INPUT");
            return;
        }

        row[r][v] = true;
        col[c][v] = true;
        sub[r / 3][c / 3][v] = true;
    }

    println!("OK");
}
