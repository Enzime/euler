use std::collections::HashSet;

fn main() {
    let mut products = HashSet::new();

    for a in 1..1000 {
        let a_str = a.to_string();

        for b in a.. {
            let b_str = b.to_string();

            let c = a * b;
            let c_str = c.to_string();

            if a_str.len() + b_str.len() + c_str.len() > 9 {
                break;
            }

            if a_str.len() + b_str.len() + c_str.len() < 9 {
                continue;
            }

            let digits = a_str.chars().chain(b_str.chars()).chain(c_str.chars()).collect::<HashSet<_>>();

            if digits.len() == 9 && !digits.contains(&'0') {
                products.insert(c);
            }
        }
    }

    println!("{}", products.iter().sum::<usize>());
}
