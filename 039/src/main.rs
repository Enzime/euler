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

fn coprime(a: usize, b: usize) -> bool {
    gcd(a, b) == 1
}

fn main() {
    let p_max = 1_000;

    let mut counter = Counter::new();

    'm_loop: for m in (3..).step_by(2) {
        'n_loop: for n in (1..=m - 2).step_by(2) {
            if !coprime(m, n) {
                continue;
            }

            for k in 1.. {
                let a = k * m * n;
                let b = k * (m.pow(2) - n.pow(2)) / 2;
                let c = k * (m.pow(2) + n.pow(2)) / 2;

                let p = a + b + c;

                if p > p_max {
                    // if k > 1, (m, n + 2) is still a viable candidate
                    if k > 1 {
                        continue 'n_loop;
                    }

                    // if k == 1 and n > 1, (m + 2, 1) is still a viable candidate
                    if n > 1 {
                        continue 'm_loop;
                    }

                    // if k == 1 and n == 1, (m + 2, 1) won't be a viable candidate
                    break 'm_loop;
                }

                counter[p] += 1;
            }
        }
    }

    println!("{}", counter.sorted()[0].0);
}
