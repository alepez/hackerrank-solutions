fn _solve_cheating(
    _count: usize,
    uphill: usize,
    step: usize,
) -> usize {
    (uphill - 1) * step
}

fn _solve_linear(battleground: Vec<usize>) -> usize {
    let mut it = battleground.iter();
    let mut prev = it.next().unwrap();
    for x in it {
        if x < prev {
            return *prev;
        }

        prev = x;
    }

    *prev
}

fn solve_binary(battleground: Vec<u32>) -> u32 {
    let mut l = 0;
    let mut r = battleground.len() - 1;
    let mut c = (r - l) / 2;

    while c > 0
        && c < battleground.len() - 2
        && c > l
        && c < r
    {
        if battleground[c] > battleground[c - 1] {
            // uphill
            l = c;
            c = (r + l) / 2;
        } else if battleground[c] > battleground[c + 1] {
            // downhill
            r = c;
            c = (r + l) / 2;
        }
    }

    if battleground[c] > battleground[c - 1] {
        battleground[c]
    } else {
        battleground[c - 1]
    }
}

fn battleground(
    count: usize,
    uphill: usize,
    step: usize,
) -> Vec<u32> {
    let mut v = Vec::new();
    v.resize(count, 0u32);

    for i in 0..uphill {
        v[i] = (i * step) as u32;
    }

    for i in uphill..count {
        v[i] = ((count - i - 1) * step) as u32;
    }

    v
}

fn read_number() -> Option<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok()?;
    line.trim().parse().ok()
}

fn main() {
    let result = {
        let a = read_number().unwrap();
        let b = read_number().unwrap();
        let c = read_number().unwrap();

        // solve_cheating(a, b, c)

        // solve_linear(battleground(a, b, c))

        solve_binary(battleground(a, b, c))
    };

    println!("{}", result);
}
