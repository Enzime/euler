fn main() {
    let mut primes = vec![2];

    'next_n: for n in (3..=10_000).step_by(2) {
        for p in &primes {
            if n % p == 0 {
                continue 'next_n;
            }
        }

        primes.push(n);
    }

    let mut prime_sums = vec![[0u128; N + 1]; primes.len() + 1];

    const N: usize = 500;

    'outer: for i in 1..=primes.len() {
        let p = primes[i - 1];

        for j in 2..=N {
            prime_sums[i][j] = prime_sums[i-1][j];

            if j == p {
                prime_sums[i][j] += 1;
            }

            if j > p {
                prime_sums[i][j] += prime_sums[i][j - p];
            }

            if prime_sums[i][j] >= 5_000 && p >= j {
                println!("{}", j);
                break 'outer;
            }
        }
    }
}
