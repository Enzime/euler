use euler_utils::grow_sieve;

fn main() {
    let mut is_prime = vec![];
    let mut primes = vec![];

    grow_sieve(&mut is_prime, &mut primes, 1_000);

    let mut legs: Vec<Box<dyn Fn(usize) -> usize>> = vec![];

    // top right
    legs.push(Box::new(|n: usize| -> usize { 4*n*n - 2*n + 1 }));
    // top left
    legs.push(Box::new(|n: usize| -> usize { 4*n*n + 1 }));
    // bottom left
    legs.push(Box::new(|n: usize| -> usize { 4*n*n + 2*n + 1 }));
    // bottom right
    legs.push(Box::new(|_: usize| -> usize { 0 }));

    let mut primes_seen = 0;
    let mut numbers_seen = 1;

    'n_loop: for n in 1.. {
        for leg in &legs {
            let leg_n = leg(n);

            numbers_seen += 1;

            if leg_n < is_prime.len() {
                if is_prime[leg_n] {
                    primes_seen += 1;
                }
            } else {

                while primes[primes.len() - 1] * primes[primes.len() - 1] < leg_n {
                    grow_sieve(&mut is_prime, &mut primes, 1_000);
                }

                let mut leg_n_is_prime = true;

                for p in &primes {
                    if leg_n % p == 0 {
                        leg_n_is_prime = false;
                        break;
                    }
                }

                if leg_n_is_prime {
                    primes_seen += 1;
                }
            }

            if primes_seen * 10 < numbers_seen {
                println!("{}", 2 * n + 1);
                break 'n_loop;
            }
        }
    }
}
