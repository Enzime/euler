fn main() {
    let mut n = 600851475143u64;

    'outer: for i in (2..=2).chain((3..(n as f64).sqrt() as u64 + 1).step_by(2)) {
        while n % i == 0 {
            if n == i {
                break 'outer;
            }

            n /= i;
        }
    }

    println!("{}", n);
}
