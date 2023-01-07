use std::hash::Hash;

use euler_utils::Counter;

fn prime_factorized_factorial<T>(prime_factorizations: &Vec<Counter<T>>, n: usize) -> Counter<T>
where
    T: Eq + Hash + Copy,
{
    (2..=n).map(|i| prime_factorizations[i].clone())
           .fold(Counter::new(), |acc, x| acc + x)
}

fn main() {
    let mut prime_factorizations = vec![];
    let mut primes = vec![];

    // 0 and 1 have no prime factorizations
    prime_factorizations.push(Counter::new());
    prime_factorizations.push(Counter::new());

    'next_n: for n in 2..=100 {
        for p in &primes {
            if n % p == 0 {
                let mut prime_factorization = prime_factorizations[(n / p) as usize].clone();
                prime_factorization[*p] += 1;

                prime_factorizations.push(prime_factorization);
                continue 'next_n;
            }
        }

        primes.push(n);
        prime_factorizations.push(vec![n].into_iter().collect());
    }

    let mut result = 0;

    for n in 1..=100 {
        let numerator = prime_factorized_factorial(&prime_factorizations, n);

        'next_r: for r in 1..=n {
            let numerator = numerator.clone();

            let denominator = prime_factorized_factorial(&prime_factorizations, r)
                            + prime_factorized_factorial(&prime_factorizations, n - r);


            let mut accumulator = 1;

            for (prime, count) in (numerator - denominator).into_iter() {
                for _ in 0..*count {
                    accumulator *= prime;

                    if accumulator > 1_000_000 {
                        result += 1;
                        continue 'next_r;
                    }
                }
            }
        }
    }

    println!("{}", result);
}
