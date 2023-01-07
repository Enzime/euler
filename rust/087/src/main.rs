fn main() {
    const UPPER_BOUND: usize = 50_000_000;

    let mut is_prime = vec![true; (UPPER_BOUND as f64).sqrt().ceil() as usize];
    let mut primes = vec![];

    is_prime[0] = false;
    is_prime[1] = false;

    for n in 0..is_prime.len() {
        if !is_prime[n] {
            continue;
        }

        primes.push(n);

        for k in (2*n..is_prime.len()).step_by(n) {
            is_prime[k] = false;
        }
    }

    let mut sum_of_prime_square_and_prime_cube = vec![false; UPPER_BOUND];

    for &p1 in &primes {
        let a = p1.pow(2);

        if a >= UPPER_BOUND {
            break;
        }

        for &p2 in &primes {
            let b = p2.pow(3);

            if a + b >= UPPER_BOUND {
                break;
            }

            sum_of_prime_square_and_prime_cube[a + b] = true;
        }
    }

    let mut expressible_numbers = 0; 

    for n in 1..UPPER_BOUND {
        for p in &primes {
            let c = p.pow(4);

            if c >= n {
                break;
            }

            if sum_of_prime_square_and_prime_cube[n - c] {
                expressible_numbers += 1;
                break;
            }
        }
    }

    println!("{}", expressible_numbers);
}
