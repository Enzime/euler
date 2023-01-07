use euler_utils::gcd;

fn main() {
    const MIN: f64 = 1.0 / 3.0;
    const MAX: f64 = 1.0 / 2.0;

    let mut fractions = 0;

    for d in 2..=12_000 {
        let lower_bound = (MIN * d as f64).ceil() as usize;
        let upper_bound = (MAX * d as f64).floor() as usize;

        for n in lower_bound..=upper_bound {
            let fraction = n as f64 / d as f64;

            if fraction > MIN && fraction < MAX && gcd(n, d) == 1 {
                fractions += 1;
            }
        }
    }

    println!("{}", fractions);
}
