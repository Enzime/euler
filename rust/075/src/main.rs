use euler_utils::{gcd, Counter};

fn main() {
    const MAX_L: usize = 1_500_000;

    let mut counter = Counter::new();

    'next_m: for m in (3..).step_by(2) {
        for n in (1..m).step_by(2) {
            if gcd(m, n) != 1 {
                continue;
            }

            for k in 1.. {
                let a = k*m*n;
                let b = k*(m*m - n*n) / 2;
                let c = k*(m*m + n*n) / 2;

                let L = a + b + c;

                if L > MAX_L {
                    if k > 1 {
                        break;
                    }

                    if n > 1 {
                        continue 'next_m;
                    }

                    break 'next_m;
                }

                counter[L] += 1;
            }
        }
    }

    println!("{}", counter.into_iter().filter(|(_, &count)| count == 1).count());
}
