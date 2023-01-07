use std::collections::HashSet;
use std::convert::TryInto;

fn ans(a_max: usize, b_max: usize) -> usize {
    let mut terms = HashSet::new();

    for a in 2..=a_max {
        for b in 2..=b_max {
            terms.insert(a.pow(b.try_into().unwrap()));
        }
    }

    terms.iter().count()
}

fn mine(a_max: usize, b_max: usize) -> usize {
    // find primes from [2, a_max]
    let mut primes = vec![2];

    'next_n: for n in 3.. {
        for p in &primes {
            if n % p == 0 {
                continue 'next_n;
            }
        }

        if n > a_max {
            break;
        }

        primes.push(n);
    }

    let mut prime_factorization_of = vec![vec![0; primes.len()]; a_max + 1];

    // get prime factorization for numbers from [2, a_max]
    for n in 2..=a_max {
        let mut cur = n;

        for (i, p) in primes.iter().enumerate() {
            while cur % p == 0 {
                prime_factorization_of[n][i] += 1;
                cur /= p;
            }
        }
    }

    let mut terms = HashSet::new();

    for a in 2..=a_max {
        for b in 2..=b_max {
            terms.insert(prime_factorization_of[a].iter().map(|p| p * b).collect::<Vec<_>>());
        }
    }

    terms.len()
}

fn main() {
    for a in 3..=8 {
        for b in 4..=8 {
            // println!("{} {} {} {}", a, b, ans(a, b), mine(a, b));
            assert_eq!(ans(a, b), mine(a, b));
        }
    }

    println!("{}", mine(100, 100));
}
