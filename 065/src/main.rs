use std::mem;

use rug::{Assign, Integer};

fn main() {
    let mut numerator = Integer::new();
    let mut denominator = Integer::new();

    for n in 1..=100 {
        for k in (0..n).rev() {
            let mut a_k = 1;

            if k == 0 {
                a_k = 2;
            } else if k % 3 == 2 {
                a_k = 2 * (k / 3 + 1);
            }

            if k == n - 1 {
                numerator.assign(a_k);
                denominator.assign(1);
                continue;
            }

            mem::swap(&mut numerator, &mut denominator);

            numerator += a_k * &denominator;
        }
    }

    println!("{}", numerator.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
}
