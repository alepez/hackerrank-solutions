fn main() {
    use std::collections::BTreeMap;
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let numbers: Vec<String> = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut coll: BTreeMap<usize, BTreeMap<String, usize>> =
        BTreeMap::new();

    numbers.into_iter().for_each(|number| {
        let len = number.len();
        coll.entry(len)
            .and_modify(|x| {
                x.entry(number.clone())
                    .and_modify(|y| {
                        *y += 1;
                    })
                    .or_insert(1);
            })
            .or_insert_with(|| {
                let mut x = BTreeMap::new();
                x.insert(number.clone(), 1);
                x
            });
    });

    coll.iter().for_each(|(_, x)| {
        x.iter().for_each(|(y, &count)| {
            for _ in 0..count {
                println!("{}", y);
            }
        })
    });
}
