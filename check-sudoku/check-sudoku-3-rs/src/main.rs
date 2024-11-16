fn main() {
    let stdin = std::io::stdin();
    let matrix = Matrix::from(stdin.lock());
    if matrix.is_valid() {
        println!("OK");
    } else {
        println!("WRONG INPUT");
    }
}

struct Matrix([usize; 81]);

impl Default for Matrix {
    fn default() -> Self {
        Self([0; 81])
    }
}

impl<T> From<T> for Matrix
where
    T: std::io::Read + std::io::BufRead,
{
    fn from(r: T) -> Self {
        let mut matrix = Matrix::default();
        r.lines().skip(1).for_each(|line| {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            let row: usize = parts.next().unwrap().parse().unwrap();
            let col: usize = parts.next().unwrap().parse().unwrap();
            let val: usize = parts.next().unwrap().parse().unwrap();
            let index = (row - 1) * 9 + (col - 1);
            matrix.0[index] = val;
        });
        matrix
    }
}

fn has_duplicates(v: &[usize]) -> bool {
    let mut table: [usize; 10] = [0; 10];
    for &x in v {
        table[x] += 1;
    }
    // Skip the first, which must be ignored because it contains count of zeroes
    table.iter().skip(1).any(|&f| f > 1)
}

impl Matrix {
    fn is_valid(&self) -> bool {
        for i in 1..=9 {
            if has_duplicates(&self.row(i))
                || has_duplicates(&self.col(i))
                || has_duplicates(&self.subgrid(i))
            {
                return false;
            }
        }

        true
    }

    fn row(&self, r: usize) -> [usize; 9] {
        let r = r - 1;
        let b = r * 9;
        let e = b + 9;
        self.0[b..e].try_into().unwrap()
    }

    fn col(&self, c: usize) -> [usize; 9] {
        let mut arr = [0; 9];
        let c = c - 1;
        let mut j = 0;
        for (i, &v) in self.0.iter().enumerate() {
            if (i % 9) == c {
                arr[j] = v;
                j += 1;
            }
        }
        arr
    }

    fn subgrid(&self, i: usize) -> [usize; 9] {
        let mut arr = [0; 9];
        let i = i - 1;
        let r = i / 3;
        let c = i % 3;
        let mut j = 0;
        for row in r * 3..r * 3 + 3 {
            for col in c * 3..c * 3 + 3 {
                arr[j] = self.0[(row * 9) + col];
                j += 1;
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_a() {
        let txt = r#"4
2 2 5
2 5 1
5 2 3
2 8 5
"#;
        assert_eq!(false, Matrix::from(txt.as_bytes()).is_valid());
    }

    #[test]
    fn solve_b() {
        let txt = r#"9
3 1 3
2 8 3
1 4 3
7 2 3
6 3 3
5 5 3
4 7 3
8 6 3
9 9 3"#;

        assert_eq!(true, Matrix::from(txt.as_bytes()).is_valid());
    }

    #[test]
    fn test_row_and_col() {
        let txt = r#"1
2 9 5
"#;

        let matrix = Matrix::from(txt.as_bytes());

        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.row(1));
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 5], matrix.row(2));
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.row(9));

        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.col(1));
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.col(2));
        assert_eq!(vec![0, 5, 0, 0, 0, 0, 0, 0, 0], matrix.col(9));
    }

    #[test]
    fn test_duplicates() {
        assert!(!has_duplicates(&vec![0, 2, 0]));
        assert!(!has_duplicates(&vec![1, 2, 3]));
        assert!(!has_duplicates(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert!(has_duplicates(&vec![1, 2, 3, 4, 5, 6, 7, 8, 1]));
        assert!(has_duplicates(&vec![1, 0, 0, 0, 0, 6, 7, 8, 1]));
    }
}
