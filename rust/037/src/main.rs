fn generate_more_primes(primes: &mut Vec<usize>, is_prime: &mut Vec<bool>, amount: usize) -> usize {
    let original = primes.len();

    'next_n: for n in primes[primes.len() - 1] + 1.. {
        for p in primes.iter() {
            if n % p == 0 {
                is_prime.push(false);
                continue 'next_n;
            }
        }

        is_prime.push(true);
        primes.push(n);

        if original + amount == primes.len() {
            break;
        }
    }
    
    original
}

fn main() {
    let mut primes = vec![2, 3];
    let mut is_prime = vec![false; 4];

    is_prime[2] = true;
    is_prime[3] = true;

    let mut truncatable_primes = vec![];

    while truncatable_primes.len() < 11 {
        let next = generate_more_primes(&mut primes, &mut is_prime, 1000);

        'next_prime: for p in &primes[next..] {
            if *p < 10 {
                continue;
            }

            let p_str = p.to_string();

            for n in 1..p_str.len() {
                if !is_prime[p_str[n..].parse::<usize>().unwrap()] || !is_prime[p_str[..n].parse::<usize>().unwrap()] {
                    continue 'next_prime;
                }
            }

            truncatable_primes.push(*p);
        }
    }

    println!("{:?}", truncatable_primes);
}
