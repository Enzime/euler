fn main() {
    let mut primes = vec![2, 3];
    let mut is_prime = [false; 1_000_000];

    'next_n: for n in 4..1_000_000 {
        for p in &primes {
            if n % p == 0 {
                continue 'next_n;
            }
        }

        is_prime[n] = true;
        primes.push(n);
    }

    let mut circular_primes = vec![];

    'next_prime: for p in &primes {
        let digits = p.to_string().chars().collect::<Vec<_>>();

        for n in 1..digits.len() {
            let nth_rotation = [&digits[n..], &digits[0..n]].iter()
                                                            .flat_map(|segment| *segment)
                                                            .collect::<String>()
                                                            .parse::<usize>()
                                                            .unwrap();

            if !is_prime[nth_rotation] {
                continue 'next_prime;
            }
        }

        circular_primes.push(p);
    }

    println!("{}", circular_primes.len());
}
