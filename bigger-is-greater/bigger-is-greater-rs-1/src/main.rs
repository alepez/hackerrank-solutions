use std::io::BufRead;

fn solve(a: String) -> Option<String> {
    let mut a: Vec<u8> = a.into();

    let mut i = None;

    if a.len() == 1 {
        return None;
    }

    for n in (0..=(a.len() - 2)).rev() {
        if a[n] < a[n + 1] {
            i = Some(n);
            break;
        }
    }

    let i = i?;

    let mut j = None;
    for n in ((i + 1)..=(a.len() - 1)).rev() {
        if a[i] < a[n] {
            j = Some(n);
            break;
        }
    }

    let j = j?;

    a.swap(i, j);

    a[i + 1..].reverse();

    Some(String::from_utf8(a).unwrap())
}

fn main() {
    std::io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map(|line| line.unwrap())
        .map(solve)
        .for_each(|x| {
            if let Some(x) = x {
                println!("{}", x);
            } else {
                println!("no answer");
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve("ab".to_string()), Some("ba".to_string()));
        assert_eq!(solve("bb".to_string()), None);
        assert_eq!(solve("hefg".to_string()), Some("hegf".to_string()));
        assert_eq!(solve("dhck".to_string()), Some("dhkc".to_string()));
        assert_eq!(solve("dkhc".to_string()), Some("hcdk".to_string()));
        assert_eq!(solve("a".to_string()), None);
    }
}
