fn main() {
    let mut sum = 0;

    for n in 1..1_000_000 {
        let n_binary = format!("{:b}", n);
        let n_decimal = n.to_string();

        if n_binary.as_bytes()[n_binary.len() - 1] == b'0' || n_decimal.as_bytes()[n_decimal.len() - 1] == b'0' {
            continue;
        }

        if n_decimal.chars().rev().collect::<String>() != n_decimal {
            continue;
        }

        if n_binary.chars().rev().collect::<String>() == n_binary {
            sum += n;
        }
    }

    println!("{}", sum);
}
