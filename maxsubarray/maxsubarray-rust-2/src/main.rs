use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<isize>().unwrap();

    for _ in 0..t {
        lines.next(); // Skip

        let arr: Vec<isize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let result = solve(&arr);

        println!("{} {}", result.0.unwrap(), result.1.unwrap());
    }
}

fn max_sub_sequence(arr: &[isize]) -> Option<isize> {
    if arr.is_empty() {
        return None;
    }

    let only_neg = arr.iter().all(|&x| x < 0);

    if only_neg {
        arr.iter().max().copied()
    } else {
        Some(arr.iter().filter(|&&x| x > 0).sum())
    }
}

fn max_sub_array(arr: &[isize]) -> Option<isize> {
    let mut e = *arr.get(0)?;
    let mut m = *arr.get(0)?;

    for &x in arr.iter().skip(1) {
        e = (e + x).max(x);
        m = m.max(e);
    }

    Some(m)
}

fn solve(arr: &[isize]) -> (Option<isize>, Option<isize>) {
    (max_sub_array(&arr), max_sub_sequence(&arr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![1, 2, 3, 4];
        let output = solve(&input);
        assert_eq!(output, (Some(10), Some(10)));
    }

    #[test]
    fn test_2() {
        let input = vec![2, -1, 2, 3, 4, -5];
        let output = solve(&input);
        assert_eq!(output, (Some(10), Some(11)));
    }
}
