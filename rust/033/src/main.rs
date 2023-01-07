use euler_utils::Counter;

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        return gcd(b, a);
    }

    if b == 0 {
        return a;
    }

    return gcd(a % b, b);
}

fn main() {
    let mut numerator = 1;
    let mut denominator = 1;

    for n in 10..=99 {
        if n % 10 == 0 || n % 11 == 0 {
            continue;
        }

        for d in n + 1..=99 {
            if d % 10 == 0 || d % 11 == 0 {
                continue;
            }

            let n_str = n.to_string();
            let d_str = d.to_string();

            let digit_counts = n_str.chars().chain(d_str.chars()).collect::<Counter<_>>();

            // when there is one or more digits in common
            // we want to cross out those digits!
            if digit_counts.len() < 4 {
                for (digit, count) in &digit_counts {
                    if *count > 1 {
                        let new_n = n_str.chars()
                                         .filter(|c| c != digit)
                                         .collect::<String>()
                                         .parse::<f64>()
                                         .unwrap();

                        let new_d = d_str.chars()
                                         .filter(|c| c != digit)
                                         .collect::<String>()
                                         .parse::<f64>()
                                         .unwrap();

                        if ((new_n / new_d) - ((n as f64) / (d as f64))).abs() < 1e-10 {
                            numerator *= new_n as usize;
                            denominator *= new_d as usize;
                        }
                    }
                }
            }
        }
    }

    denominator /= gcd(numerator, denominator);

    println!("{}", denominator);
}
