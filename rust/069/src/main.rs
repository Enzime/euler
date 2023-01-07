fn main() {
    let mut is_prime = [true; 1_000_001];

    is_prime[0] = false;
    is_prime[1] = false;

    let mut prime_factors_of = vec![vec![]; 1_000_001];
    let mut best_ratio = (0.0, 0);

    for n in 2..=1_000_000 {
        let mut phi_n = (n - 1) as i32;

        if !is_prime[n] {
            for k in 1..2u32.pow(prime_factors_of[n].len() as u32) {
                let mut primes_included = 0;
                let mut non_coprime_base = 1;

                let mut i = 0;
                let mut cur = k;
                
                while cur > 0 {
                    if cur % 2 == 1 {
                        primes_included += 1;
                        non_coprime_base *= prime_factors_of[n][i];
                    }

                    cur /= 2;
                    i += 1;
                }

                if primes_included % 2 == 0 {
                    phi_n += ((n - 1) / non_coprime_base) as i32;
                } else {
                    phi_n -= ((n - 1) / non_coprime_base) as i32;
                }
            }
        } else {
            for k in (2*n..=1_000_000).step_by(n) {
                prime_factors_of[k].push(n);
                is_prime[k] = false;
            }
        }

        let candidate_ratio = (n as f64 / phi_n as f64, n);

        if candidate_ratio > best_ratio {
            best_ratio = candidate_ratio;
        }
    }

    println!("{}", best_ratio.1);
}
