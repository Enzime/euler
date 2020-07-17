fn main() {
    // ignore 2 as a prime cause we know 2 can never be the prime
    // that is added to a square to make an odd composite
    let mut primes = vec![];

    'next_n: for n in (3..).step_by(2) {
        let mut is_prime = true;
        
        for p in &primes {
            if n % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(n);
            continue;
        }

        for p in &primes {
            let sqrt = (((n - p) / 2) as f64).sqrt();

            if sqrt.fract() < 1e-10 {
                continue 'next_n;
            }
        }

        println!("{}", n);
        break;
    }
}
