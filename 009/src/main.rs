fn main() {
    for a in 1..1000 {
        for b in a.. {
            let a = a as f64;
            let b = b as f64;

            let c = a.hypot(b);

            if a + b + c > 1000.5 {
                break
            }

            if c.fract() < 1e-10 && ((a + b + c - 1000.0).abs() < 1e-10) {
                println!("{}", a * b * c);
            }
        }
    }
}
