fn main() {
    const MAX_N: usize = 9_999_999;
    let mut is_prime = vec![true; MAX_N + 1];
    let mut phi = (0..=MAX_N).collect::<Vec<_>>();

    is_prime[0] = false;
    is_prime[1] = false;

    let mut best_found = (1000.0, 0);

    for n in 2..=MAX_N {
        if !is_prime[n] {
            let mut n_digits = vec![];
            let mut cur = n;

            while cur > 0 {
                n_digits.push(cur % 10);
                cur /= 10;
            }

            let mut phi_n_digits = vec![];
            cur = phi[n];

            while cur > 0 {
                phi_n_digits.push(cur % 10);
                cur /= 10;
            }

            n_digits.sort();
            phi_n_digits.sort();

            if n_digits == phi_n_digits {
                println!("{} {}", n, phi[n]);

                let current_phi = (n as f64 / phi[n] as f64, n);

                if current_phi < best_found {
                    best_found = current_phi;
                }
            }

            continue;
        }

        phi[n] = n - 1;

        for k in (2*n..=MAX_N).step_by(n) {
            is_prime[k] = false;
            phi[k] *= n - 1;
            phi[k] /= n;
        }
    }

    println!("{:?}", best_found);
}
