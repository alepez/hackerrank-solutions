use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock().lines();
    let t = stdin
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<isize>()
        .unwrap();

    for _ in 0..t {
        // Skip first line
        let _n = stdin
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<isize>()
            .unwrap();

        let arr: Vec<isize> = stdin
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<isize>().unwrap())
            .collect();

        let result = solve(&arr);

        println!("{} {}", result.0, result.1);
    }
}

fn max_sub_sequence(arr: &[isize]) -> isize {
    let only_neg = arr.iter().all(|&x| x < 0);

    if only_neg {
        *arr.iter().max().unwrap()
    } else {
        arr.iter().filter(|&&x| x > 0).sum()
    }
}

fn max_sub_array(arr: &[isize]) -> isize {
    let mut max = isize::min_value();
    let max_len = arr.len();

    for len in 1..=max_len {
        for offset in 0..=(max_len - len) {
            let sub = arr.iter().skip(offset).take(len);
            max = max.max(sub.sum());
        }
    }

    max
}

fn solve(arr: &[isize]) -> (isize, isize) {
    (max_sub_array(&arr), max_sub_sequence(&arr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![1, 2, 3, 4];
        let output = solve(&input);
        assert_eq!(output, (10, 10));
    }

    #[test]
    fn test_2() {
        let input = vec![2, -1, 2, 3, 4, -5];
        let output = solve(&input);
        assert_eq!(output, (10, 11));
    }
}
