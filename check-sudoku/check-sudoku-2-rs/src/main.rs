fn main() {
    use std::io::{self, Read};
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s).unwrap();
    let items = Items::from(s.as_str());
    let matrix = Matrix::from(items);
    if matrix.is_valid() {
        println!("OK");
    } else {
        println!("WRONG INPUT");
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    row: usize,
    col: usize,
    val: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Items(Vec<Item>);

struct Matrix(Vec<Vec<usize>>);

impl Default for Matrix {
    fn default() -> Self {
        Self(vec![vec![0; 9]; 9])
    }
}

impl From<Items> for Matrix {
    fn from(items: Items) -> Self {
        let mut matrix = Matrix::default();
        for Item { row, col, val } in items.0 {
            matrix.0[row - 1][col - 1] = val;
        }
        matrix
    }
}

fn has_duplicates(mut v: Vec<usize>) -> bool {
    let len = v.iter().filter(|&&x| x != 0).count();
    v.sort();
    v.dedup();
    v.iter().filter(|&&x| x != 0).count() != len
}

impl Matrix {
    fn is_valid(&self) -> bool {
        for i in 1..=9 {
            if has_duplicates(self.row(i))
                || has_duplicates(self.col(i))
                || has_duplicates(self.subgrid(i))
            {
                return false;
            }
        }

        true
    }

    fn row(&self, r: usize) -> Vec<usize> {
        self.0[r - 1].clone()
    }

    fn col(&self, c: usize) -> Vec<usize> {
        self.0.iter().map(|row| row[c - 1]).collect()
    }

    fn subgrid(&self, i: usize) -> Vec<usize> {
        let i = i - 1;
        let r = i / 3;
        let c = i % 3;
        let mut v = vec![];
        for row in r * 3..r * 3 + 3 {
            for col in c * 3..c * 3 + 3 {
                v.push(self.0[row][col]);
            }
        }
        v
    }
}

impl From<&str> for Items {
    fn from(txt: &str) -> Self {
        let v = txt
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split_whitespace();
                let row = parts.next().unwrap().parse().unwrap();
                let col = parts.next().unwrap().parse().unwrap();
                let val = parts.next().unwrap().parse().unwrap();
                Item { row, col, val }
            })
            .collect();
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let items = Items(vec![
            Item {
                row: 2,
                col: 2,
                val: 5,
            },
            Item {
                row: 2,
                col: 5,
                val: 1,
            },
            Item {
                row: 5,
                col: 2,
                val: 3,
            },
            Item {
                row: 2,
                col: 8,
                val: 5,
            },
        ]);

        let txt = r#"4
2 2 5
2 5 1
5 2 3
2 8 5
"#;

        assert_eq!(items, Items::from(txt));
    }

    #[test]
    fn solve_a() {
        let items = Items(vec![
            Item {
                row: 2,
                col: 2,
                val: 5,
            },
            Item {
                row: 2,
                col: 5,
                val: 1,
            },
            Item {
                row: 5,
                col: 2,
                val: 3,
            },
            Item {
                row: 2,
                col: 8,
                val: 5,
            },
        ]);

        assert_eq!(false, Matrix::from(items).is_valid());
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

        assert_eq!(true, Matrix::from(Items::from(txt)).is_valid());
    }

    #[test]
    fn test_row_and_col() {
        let items = Items(vec![Item {
            row: 2,
            col: 9,
            val: 5,
        }]);

        let matrix = Matrix::from(items);

        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.row(1));
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 5], matrix.row(2));
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.row(9));

        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.col(1));
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.col(2));
        assert_eq!(vec![0, 5, 0, 0, 0, 0, 0, 0, 0], matrix.col(9));
    }
}
