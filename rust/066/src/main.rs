use std::cmp;
use std::collections::HashMap;
use std::mem;

use rug::{Assign, Integer};

fn main() {
    let mut best_found = (Integer::new(), 0);

    // D from 2 -> 1000
    // ignoring perfect squares
    for D in 2u32..=1000 {
        let sqrt_D = (D as f64).sqrt();

        if sqrt_D.fract() <= std::f64::EPSILON {
            continue;
        }

        // determine continued fraction representation for sqrt(D)
        let mut fractions_seen = HashMap::new();

        let mut n_k = 1;
        let mut s_k = sqrt_D as u32;

        fractions_seen.insert((n_k, s_k), 0);
        let mut a = vec![s_k];
        let mut period = 0;

        for k in 1.. {
            let next_a_k = (n_k as f64 / (sqrt_D - s_k as f64)) as u32;
            let next_n_k = (D - s_k * s_k) / n_k;
            let next_s_k = next_a_k * next_n_k - s_k;

            let key = (next_n_k, next_s_k);

            a.push(next_a_k);
            period += 1;

            if fractions_seen.contains_key(&key) {
                break;
            }

            fractions_seen.insert(key, k);

            n_k = next_n_k;
            s_k = next_s_k;
        }

        // compute convergents to find the first convergent that fits
        // n^2 - Dd^2 = 1
        for i in 1.. {
            let mut n = Integer::new();
            let mut d = Integer::new();

            for k in (0..i).rev() {
                let mut relative_k = k % period;

                if k != 0 && relative_k == 0 {
                    relative_k = period;
                }

                if k == i - 1 {
                    n.assign(a[relative_k]);
                    d.assign(1);
                    continue;
                }

                mem::swap(&mut n, &mut d);
                n += a[relative_k] * &d;
            }

            let mut pells = Integer::from(&n * &n);
            pells -= D * Integer::from(&d * &d);

            if pells == 1 {
                // println!("{}^2 - {}*{}^2 = {}", n, D, d, pells);
                best_found = cmp::max((n, D), best_found);
                break;
            }
        }
    }

    println!("{}", best_found.1);
}
