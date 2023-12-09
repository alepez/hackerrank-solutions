use std::io::BufRead;

fn product_of_array_except_self(arr: &[usize]) -> Vec<usize> {
    let mut res = vec![1; arr.len()];
    let mut acc = 1;
    for i in 0..(arr.len() - 1) {
        res[i + 1] = acc * arr[i];
        acc *= arr[i];
    }
    let mut acc = 1;
    for i in (1..(arr.len())).rev() {
        res[i - 1] *= acc * arr[i];
        acc *= arr[i];
    }
    res
}

fn main() {
    let stdin = std::io::stdin();
    let arr: Vec<usize> = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();
    product_of_array_except_self(&arr).iter().for_each(|x| {
        print!("{x} ");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            product_of_array_except_self(&[1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            product_of_array_except_self(&[1, 2, 3, 4, 5]),
            vec![120, 60, 40, 30, 24]
        );

        let cases = [
            vec![1],
            vec![1, 10],
            vec![10, 20],
            vec![3, 7, 8, 18],
            vec![3, 7, 7, 3],
        ];

        for case in cases {
            assert_eq!(
                product_of_array_except_self(case.as_slice()),
                product_of_array_except_self_2(case.as_slice())
            );
        }
    }
}
