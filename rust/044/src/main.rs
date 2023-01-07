use std::collections::HashSet;

fn main() {
    let mut pentagonal_numbers = vec![];
    let mut pentagonal_numbers_set = HashSet::new();

    for n in 1..=10_000 {
        let p_n = n * (3 * n - 1) / 2;

        pentagonal_numbers.push(p_n);
        pentagonal_numbers_set.insert(p_n);
    }

    'j_loop: for j in 0..pentagonal_numbers.len() {
        for k in j+1..pentagonal_numbers.len() {
            let sum = pentagonal_numbers[j] + pentagonal_numbers[k];
            let diff = pentagonal_numbers[k] - pentagonal_numbers[j];

            if !pentagonal_numbers_set.contains(&sum) || !pentagonal_numbers_set.contains(&diff) {
                continue;
            }

            println!("{}", diff);
            break 'j_loop;
        }
    }
}
