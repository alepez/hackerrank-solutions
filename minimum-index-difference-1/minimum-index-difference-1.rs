fn parse_line(line: &str) -> Vec<i32> {
    line.trim().split(' ').map(|x| x.parse().unwrap()).collect()
}

fn enumerate_and_sort(vec: Vec<i32>) -> Vec<(i32, i32)> {
    let mut vec: Vec<(usize, &i32)> = vec.iter().enumerate().collect();
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    vec.iter().map(|x| (x.0 as i32, *x.1)).collect()
}

fn minimum_index_differnce(fst: Vec<i32>, snd: Vec<i32>) -> i32 {
    let fst = enumerate_and_sort(fst);
    let snd = enumerate_and_sort(snd);

    *fst.iter()
        .zip(snd.iter())
        .map(|((i, x), (j, _))| (x, (i - j).abs()))
        .min_by(|d1, d2| d1.1.cmp(&d2.1))
        .unwrap()
        .0
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap).skip(1);

    let fst = parse_line(lines.next().unwrap().as_str());
    let snd = parse_line(lines.next().unwrap().as_str());

    println!("{:?}", minimum_index_differnce(fst, snd));
}
