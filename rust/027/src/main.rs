use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut primes: HashSet<_> = HashSet::from_iter(2..=3);

    'outer: for i in (5..).step_by(2) {
        if primes.len() == 10_000 {
            break;
        }

        for p in &primes {
            if i % p == 0 {
                continue 'outer;
            }
        }

        primes.insert(i);
    }

    // println!("{:?}", primes);

    println!("{:?}", (-999..1000).flat_map(|a| (-1000..=1000).map(|b| ((0..).map(|n| n*n + a*n + b)
                                                                            .take_while(|y| primes.contains(y))
                                                                            .count(), a * b))
                                                             .collect::<Vec<_>>())
                                 .max().unwrap().1);
}
