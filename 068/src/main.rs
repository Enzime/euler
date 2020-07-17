use std::cmp;
use std::collections::HashSet;

use euler_utils::{factorial, nth_permutation};

fn main() {
    let mut best_found = "".to_string();

    for n in 0..factorial(10) {
        let mut permutation = nth_permutation((1..=10).into_iter().collect(), n);

        if permutation.iter().position(|&x| x == 10).unwrap() > 4 {
            continue;
        }

        if *permutation[0..5].iter().min().unwrap() != permutation[0] {
            continue;
        }

        permutation.push(permutation[5]);

        let rings = permutation[5..].windows(2)
                                    .enumerate()
                                    .map(|(index, window)|
                                         [&permutation[index..index+1], window].concat())
                                    .collect::<Vec<_>>();

        if rings.iter().map(|ring| ring.iter().sum::<usize>()).collect::<HashSet<_>>().len() == 1 {
            best_found = cmp::max(rings.iter()
                                       .flat_map(|ring| ring.iter()
                                                            .map(|n| n.to_string()))
                                       .collect::<String>(), best_found);
        }
    }

    println!("{}", best_found);
}
