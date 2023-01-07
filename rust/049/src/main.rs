use std::char;
use std::collections::HashSet;

use euler_utils::{Counter, nth_permutation};

fn main() {
    let mut primes = vec![2];
    let mut primes_set = vec![2].into_iter().collect::<HashSet<_>>();

    'next_i: for i in (3..10_000).step_by(2) {
        for p in &primes {
            if i % p == 0 {
                continue 'next_i;
            }
        }

        primes.push(i);
        primes_set.insert(i);

        if i / 1000 < 1 {
            continue;
        }

        let prime_permutations = (0..24).map(|n| nth_permutation(i.to_string()
                                                                  .chars()
                                                                  .map(|d| d.to_digit(10).unwrap())
                                                                  .collect(), n)
                                                   .into_iter()
                                                   .map(|d| char::from_digit(d, 10).unwrap())
                                                   .collect::<String>()
                                                   .parse::<usize>()
                                                   .unwrap())
                                        .filter(|perm| perm >= &1000)
                                        .collect::<HashSet<_>>()
                                        .intersection(&primes_set)
                                        .cloned()
                                        .collect::<HashSet<_>>();

        // we want the other sequence
        if prime_permutations.contains(&1487) {
            continue;
        }

        if prime_permutations.len() > 2 {
            let sorted_prime_permutations = {
                let mut v = prime_permutations.iter().cloned().collect::<Vec<_>>();
                v.sort();
                v
            };

            for i in 0..prime_permutations.len() - 2 {
                for j in i+1..prime_permutations.len() - 1 {
                    let next = sorted_prime_permutations[j] + sorted_prime_permutations[j] - sorted_prime_permutations[i];

                    if prime_permutations.contains(&next) {
                        // println!("{:?} {:?}", prime_permutations, sorted_prime_permutations);
                        println!("{}{}{}", sorted_prime_permutations[i], sorted_prime_permutations[j], next);
                        break 'next_i;
                    }
                }
            }

            // println!("{:?} {:?}", prime_permutations, sorted_prime_permutations);
        }
    }
}
