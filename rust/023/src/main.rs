fn main() {
    // d(n) = n => n is a perfect number
    // d(n) < n => n is deficient
    // d(n) > n => n is abundant

    // find the sum of all positive integers that cannot be written as the sum of 2 abundant numbers
    // [1, 28123]
    
    let mut abundant = [false; 28124];

    let mut non_expressible = vec![];
    let mut sum = 0;

    'outer: for n in 1..28124 {
        let sqrt_n = (n as f64).sqrt();

        // wrong for d[1] but it doesn't matter
        let mut d_n = 1;

        for f in 2..=(sqrt_n as usize) {
            if n % f == 0 {
                if f as f64 == sqrt_n {
                    d_n += f;
                } else {
                    d_n += (n / f) + f;
                }
            }
        }

        abundant[n] = d_n > n;

        // find n = a + b where a and b are abundant numbers
        for a in 1..n {
            let b = n - a;

            if abundant[a] && abundant[b] {
                continue 'outer;
            }
        }

        non_expressible.push(n);
        sum += n;
    }

    println!("{:?}", non_expressible);
    println!("{}", sum);
}
