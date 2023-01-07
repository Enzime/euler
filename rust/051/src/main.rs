use std::collections::HashMap;

use euler_utils::Counter;

fn main() {
    let mut is_prime = [true; 1_000_001];

    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes_for_mask = HashMap::new();

    for i in (2..3).chain((3..=1_000_000).step_by(2)) {
        if !is_prime[i] {
            continue;
        }

        for k in (2*i..=1_000_000).step_by(i) {
            is_prime[k] = false;
        }

        // generate all possible masks
        let i_digits = i.to_string();
        let digit_counts = i.to_string()
                            .chars()
                            .collect::<Counter<_>>()
                            .sorted();

        for (digit, count) in digit_counts {
            // missing the 2 length mask in numbers with 3 of the same digit
            
            for j in 1..2usize.pow(count as u32) {
                let mut segments = i_digits.split(digit)
                                           .map(|d| d.to_string())
                                           .collect::<Vec<String>>();

                let glue = format!("{:0width$b}", j, width = count).replace('1', "*")
                                                                   .replace('0', &digit.to_string());

                for (n, separator) in glue.chars().enumerate() {
                    segments.insert(2 * n + 1, separator.to_string());
                }

                let mask = segments.join("");
                primes_for_mask.entry(mask).or_insert(vec![]).push(i);
            }
        }
    }

    assert_eq!(primes_for_mask["56**3"].len(), 7);

    println!("{}", primes_for_mask.iter()
                                  .filter(|(_, primes)| primes.len() == 8)
                                  .flat_map(|(_, primes)| primes)
                                  .min()
                                  .unwrap());
}
