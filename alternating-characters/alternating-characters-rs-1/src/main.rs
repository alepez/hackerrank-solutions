use std::io::BufRead;

fn alternating_characters(input: String) -> usize {
    let mut iter = input.bytes();
    let mut curr_char = iter.next().unwrap();
    let mut count = 0;

    for c in iter {
        if c == curr_char {
            count += 1;
        } else {
            curr_char = c;
        }
    }

    count
}

fn main() {
    std::io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map(|line| line.unwrap())
        .map(alternating_characters)
        .for_each(|x| println!("{}", x));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_alternating_characters() {
        assert_eq!(alternating_characters("AAAA".to_string()), 3);
        assert_eq!(alternating_characters("BBBBB".to_string()), 4);
        assert_eq!(alternating_characters("ABABABAB".to_string()), 0);
        assert_eq!(alternating_characters("BABABA".to_string()), 0);
        assert_eq!(alternating_characters("AAABBB".to_string()), 4);
    }
}
