use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let _ = lines.next();
    let a = parse_line(lines.next().unwrap().unwrap());
    let b = parse_line(lines.next().unwrap().unwrap());
    println!("{}", solve(a, b));
}

fn parse_line(line: String) -> Vec<usize> {
    line.split(' ')
        .map(|token| token.parse().unwrap())
        .collect()
}

fn solve(mut a: Vec<usize>, mut b: Vec<usize>) -> usize {
    let mut beautiful_count: usize = 0;

    a.sort();
    b.sort();

    let mut a_iter = a.iter();
    let mut b_iter = b.iter();

    let mut ox = a_iter.next();
    let mut oy = b_iter.next();

    while let (Some(x), Some(y)) = (ox, oy) {
        if x == y {
            ox = a_iter.next();
            oy = b_iter.next();
            beautiful_count += 1;
        } else if x > y {
            oy = b_iter.next();
        } else if x < y {
            ox = a_iter.next();
        }
    }

    if beautiful_count == a.len() {
        beautiful_count - 1
    } else {
        beautiful_count + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let a = vec![3, 5, 7, 11, 5, 8];
        let b = vec![5, 7, 11, 10, 5, 8];
        assert_eq!(6, solve(a, b));
    }
}
