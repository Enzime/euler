fn main() {
    let mut primes = vec![];
    let mut consecutive = 0;

    for i in 2.. {
        // prime factorize the number!
        let mut cur = i;

        let mut distinct_prime_factors = 0;

        for p in &primes {
            if cur % p != 0 {
                continue;
            }

            distinct_prime_factors += 1;

            while cur % p == 0 {
                cur /= p;
            }
        }

        if distinct_prime_factors == 0 {
            primes.push(i);
        }

        if distinct_prime_factors == 4 {
            consecutive += 1;
        } else {
            consecutive = 0;
        }

        if consecutive == 4 {
            println!("{}", i - 3);
            break;
        }
    }
}
