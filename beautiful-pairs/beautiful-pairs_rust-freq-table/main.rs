use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let _ = lines.next();
    let a = parse_line(lines.next().unwrap().unwrap());
    let b = parse_line(lines.next().unwrap().unwrap());
    println!("{}", solve(&a, &b));
}

fn parse_line(line: String) -> Vec<usize> {
    line.split(' ')
        .map(|token| token.parse().unwrap())
        .collect()
}

fn solve(a: &[usize], b: &[usize]) -> usize {
    let mut beautiful_count = 0;
    let mut freq_table = [0; 1001];

    unsafe {
        for xb in b {
            *freq_table.get_unchecked_mut(*xb) += 1;
        }

        for xa in a {
            if *freq_table.get_unchecked(*xa) > 0 {
                *freq_table.get_unchecked_mut(*xa) -= 1;
                beautiful_count += 1;
            }
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
        assert_eq!(6, solve(&a, &b));
    }
}
