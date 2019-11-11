fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();

    /* Skip first line, parse other lines, splitting by spaces */
    let mut lines: Vec<Vec<u32>> = stdin
        .lock()
        .lines()
        .skip(1)
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .flat_map(str::parse)
                .collect()
        })
        .collect();

    let mut arr2 = lines.pop().unwrap();

    /* Add indexes */
    let mut arr1: Vec<(usize, u32)> = lines.pop().unwrap().iter().cloned().enumerate().collect();

    /* Descending sort by value, keep the index */
    arr1.sort_by(|(_, x), (_, y)| y.cmp(x));
    arr2.sort_by(|x, y| y.cmp(x));

    /* Remember how many elements to skip */
    let mut to_skip = 0;

    let mut result = Vec::new();

    for &(i, x) in arr1.iter() {
        /* Find the first less or equal than x. All following will also
         * be less or equal than x, because they are sorted descending */
        if let Some(m) = arr2.iter().skip(to_skip).position(|&y| x >= y) {
            result.push((i, (arr1.len() - m - to_skip) as u32));
            to_skip = m;
        } else {
            result.push((i, 0));
        }
    }

    result.sort();

    for (_, r) in result {
        println!("{}", r);
    }
}
