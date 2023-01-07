use std::collections::HashSet;

use euler_utils::{digits, factorial};

fn main() {
    let mut max_chains = 0;
    
    let mut terms = Vec::with_capacity(60);

    for n in 1..1_000_000 {
        let mut cur = n;

        terms.clear();
        terms.push(n);

        loop {
            cur = digits(cur).into_iter().map(factorial).sum::<usize>();

            if terms.contains(&cur) {
                break;
            }

            terms.push(cur);
        }

        if terms.len() == 60 {
            max_chains += 1;
        }
    }

    println!("{}", max_chains);
}
