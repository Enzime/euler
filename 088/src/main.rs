use std::collections::HashSet;

fn factorizations(prime_factorization: Vec<usize>, factorizations_of: &mut Vec<Vec<Vec<usize>>>) {
    if prime_factorization.len() == 2 {
        factorizations_of.push(vec![prime_factorization.clone()]);
        return;
    }

    let n = prime_factorization.iter().product::<usize>();
    let sqrt_n = (n as f64).sqrt();
    let mut factorizations_of_n = vec![];
    let mut factors_seen = HashSet::new();

    for mut k in 1..2usize.pow(prime_factorization.len() as u32) {
        let mut i = prime_factorization.len();
        let mut factor = 1;

        while k > 0 {
            i -= 1;

            if k % 2 == 1 {
                factor *= prime_factorization[i];
            }

            k /= 2;
        }

        if factor as f64 > sqrt_n || factors_seen.contains(&factor) {
            continue;
        }

        factors_seen.insert(factor);

        factorizations_of_n.push(vec![factor, n / factor]);

        for factorization in &factorizations_of[n / factor] {
            let mut new_factorization = factorization.iter().cloned().collect::<Vec<_>>();

            if factor > new_factorization[0] {
                continue;
            }

            new_factorization.insert(0, factor);
            factorizations_of_n.push(new_factorization);
        }
    }

    factorizations_of.push(factorizations_of_n);
}

fn main() {
    const MAX_K: usize = 12_000;

    let mut k_seen = vec![false; MAX_K + 1];
    let mut k_seen_count = 1;

    let mut minimal_product_sum_sum = 0;

    let mut primes = vec![];

    let mut factorizations_of = vec![vec![]; 2];

    for n in 2.. {
        if k_seen_count == MAX_K {
            break;
        }

        let mut is_prime = true;
        let mut cur = n;
        let mut prime_factorization = vec![];

        for &p in &primes {
            if cur % p == 0 {
                is_prime = false;

                while cur % p == 0 {
                    cur /= p;
                    prime_factorization.push(p);
                }
            }
        }

        if is_prime {
            primes.push(n);
            factorizations_of.push(vec![]);
            continue;
        }

        factorizations(prime_factorization, &mut factorizations_of);

        let mut new_n = true;

        for factorization in &factorizations_of[n] {
            let ones_to_add = n - factorization.iter().sum::<usize>();
            let k = factorization.len() + ones_to_add;

            if k > MAX_K {
                continue;
            }

            if !k_seen[k] {
                if new_n {
                    minimal_product_sum_sum += n;
                    new_n = false;
                }

                k_seen[k] = true;
                k_seen_count += 1;
            }
        }
    }

    assert_eq!(factorizations_of[4], vec![vec![2, 2]]);
    assert_eq!(factorizations_of[6], vec![vec![2, 3]]);
    assert_eq!(factorizations_of[8], vec![vec![2, 4], vec![2, 2, 2]]);

    println!("{}", minimal_product_sum_sum);
}
