fn main() {
    let mut d = [1usize; 10_000];

    d[0] = 0;
    d[1] = 0;

    let mut amicable_numbers_sum = 0;

    for n in 2..10_000 {
        // calculate d(n) = sum(factors of n excluding itself)
        let sqrt_n = (n as f64).sqrt();

        for f in 2..=(sqrt_n as usize) {
            if n % f == 0 {
                if f as f64 == sqrt_n {
                    d[n] += f;
                } else {
                    d[n] += f + (n / f);
                }
            }
        }

        // if d(n) < n, then check if d[d(n)] == n
        if d[n] < n && d[d[n]] == n {
            amicable_numbers_sum += n + d[n];
        }
    }

    println!("{}", amicable_numbers_sum);
}
