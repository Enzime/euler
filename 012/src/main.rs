fn main() {
    for i in 2.. {
        let n: u64 = (1..=i).sum();
        let sqrt = (n as f64).sqrt();

        let mut divisors = 0;

        if sqrt.fract() < 1e-10 {
            divisors = -1;
        }

        for d in 1..=(sqrt as u64) {
            if n % d == 0 {
                divisors += 2;
            }
        }

        if divisors > 500 {
            println!("{}", n);
            break;
        }
    }
}
