fn main() {
    let mut primes = vec![];
    let mut is_prime = [true; 1_000_001];

    is_prime[0] = false;
    is_prime[1] = false;

    for n in (2..=2).chain((3..=1_000_000).step_by(2)) {
        if !is_prime[n] {
            continue;
        }

        primes.push(n);

        // wipe out its multiples
        for k in (2*n..=1_000_000).step_by(n) {
            is_prime[k] = false;
        }
    }


    'l_loop: for l in (1..=primes.len()).rev() {
        for i in 0..=primes.len() - l {
            let sum = primes[i..i+l].iter().sum::<usize>();

            if sum > 1_000_000 {
                break;
            }

            if is_prime[sum] {
                println!("{} {}", l, sum);
                break 'l_loop;
            }
        }
    }
}
