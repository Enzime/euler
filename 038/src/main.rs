use std::collections::HashSet;

fn main() {
    let mut pandigitals = vec![];

    'next_i: for i in 1..10_000 {
        let mut digits = HashSet::new();
        let mut concatenated_product = vec![];
        let mut old_length = 0;

        for n in 1.. {
            let product = (n * i).to_string();

            digits.extend(product.chars());

            // if repeated digits are found
            if digits.contains(&'0') || product.len() + old_length != digits.len() {
                continue 'next_i;
            }

            concatenated_product.extend(product.chars());

            if digits.len() == 9 {
                pandigitals.push(concatenated_product.into_iter().collect::<String>().parse::<usize>().unwrap());
                break;
            }

            old_length = digits.len();
        }
    }

    println!("{}", pandigitals.iter().max().unwrap());
}
