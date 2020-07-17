use std::collections::HashMap;

fn main() {
    let mut odd_periods_seen = 0;

    for n in 2..=10_000 {
        let sqrt_n = (n as f64).sqrt();

        if sqrt_n.fract() <= std::f64::EPSILON {
            continue;
        }

        // for k = 0
        let mut n_k = 1;
        let mut s_k = sqrt_n as usize;

        let mut fractions_seen = HashMap::new();

        fractions_seen.insert((n_k, s_k), 0);

        for k in 1.. {
            let next_a_k = (n_k as f64 / (sqrt_n - s_k as f64)) as usize;
            let next_n_k = (n - s_k * s_k) / n_k;
            let next_s_k = next_a_k * next_n_k - s_k;

            let key = (next_n_k, next_s_k);

            // check if n_(k+1) or s_(k+1) is in fractions seen
            if fractions_seen.contains_key(&key) {
                if (k - fractions_seen[&key]) % 2 == 1 {
                    odd_periods_seen += 1;
                }

                break;
            }

            fractions_seen.insert(key, k);

            n_k = next_n_k;
            s_k = next_s_k;
        }
    }

    println!("{}", odd_periods_seen);
}
