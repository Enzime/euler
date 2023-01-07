fn main() {
    const TARGET: f64 = 3.0 / 7.0;

    let mut best_found = (TARGET, 0, 1);

    for d in 2..=1_000_000 {
        let n = (TARGET * d as f64).floor() as usize;
        let distance = TARGET - (n as f64 / d as f64);

        if distance > std::f64::EPSILON && distance < best_found.0 {
            best_found = (distance, n, d);
        }
    }

    println!("{}", best_found.1);
}
