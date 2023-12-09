fn main() {
    println!("Hello, world!");
}

fn is_valid(s: &str) -> &'static str {
    let mut freqs: [usize; 26] = [0; 26];

    for x in s.as_bytes() {
        let y = (x - b'a') as usize;
        freqs[y] += 1;
    }

    let max_freq = *freqs.iter().max().unwrap();
    let min_freq = *freqs.iter().filter(|&&x| x > 0).min().unwrap();
    let min_freq_not_one = freqs.iter().filter(|&&x| x > 1).min().copied();
    let max_freq_count = freqs.iter().filter(|&&x| x == max_freq).count();
    let min_freq_count = freqs.iter().filter(|&&x| x == min_freq).count();

    let valid =
        // all the characters occur the same number of times
        (min_freq == max_freq) ||
        // all the characters occur the same number of times except for one which occurs only once
        (min_freq == 1 && min_freq_count == 1 && (Some(max_freq) == min_freq_not_one)) ||
        // all the characters occur the same number of times except for one which occurs once more the others
        ((max_freq - min_freq) == 1) && (max_freq_count == 1);

    if valid {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("NO", is_valid("aabbcd"));
        assert_eq!("YES", is_valid("bbccaaa"));
        assert_eq!("YES", is_valid("aaabbcc"));
        assert_eq!("YES", is_valid("aabbcca"));
        assert_eq!("YES", is_valid("aa"));
        assert_eq!("YES", is_valid("abab"));
        assert_eq!("YES", is_valid("ababc"));
        assert_eq!("YES", is_valid("cabcabc"));
        assert_eq!("YES", is_valid("zxyzxyz"));
        assert_eq!("YES", is_valid("aaabbbccc"));
        assert_eq!("YES", is_valid("aaabbbcccz"));
        assert_eq!("YES", is_valid("a"));
        assert_eq!("YES", is_valid("aab"));
        assert_eq!("YES", is_valid("aabbc"));
        assert_eq!("YES", is_valid("aaabbbc"));
        assert_eq!("YES", is_valid("aaabbbcccx"));
        assert_eq!("NO", is_valid("ccabcabc"));
        assert_eq!("NO", is_valid("zzxyzcyz"));
        assert_eq!("NO", is_valid("aaabbbcc"));
        assert_eq!("NO", is_valid("aaabbbccx"));
        assert_eq!("NO", is_valid("aaabbbccxy"));
        assert_eq!("NO", is_valid("aaabbbddccxy"));
        assert_eq!("NO", is_valid("aaabbbxy"));
        assert_eq!("YES", is_valid("abcc"));
        assert_eq!("YES", is_valid("aabbc"));
    }
}
