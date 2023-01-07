use std::cmp;

fn f(seen: &mut Vec<usize>, n: usize) -> usize {
    if seen[n] != 0 {
        return seen[n];
    }

    let mut cur = n;
    let mut next_n = 0;

    while cur > 0 {
        next_n += (cur % 10) * (cur % 10);
        cur /= 10;
    }

    let f_n = f(seen, next_n);
    seen[n] = f_n;
    return f_n;
}

fn main() {
    const UPPER_BOUND: usize = 10_000_000;

    let mut seen = vec![0; cmp::max(UPPER_BOUND, 1000) + 1];
    seen[1] = 1;
    seen[89] = 89;

    let mut eighty_niners = 0;

    for i in 1..UPPER_BOUND {
        if f(&mut seen, i) == 89 {
            eighty_niners += 1;
        }
    }

    println!("{}", eighty_niners);
}
