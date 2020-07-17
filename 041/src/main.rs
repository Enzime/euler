use euler_utils::nth_permutation;

fn main() {
    let mut primes = vec![2, 3];
    let mut n = 3;

    // sqrt(987654321) = 31_426.968...
    'next_n: while primes[primes.len() - 1] < 31_427 {
        n += 2;

        for p in &primes {
            if n % p == 0 {
                continue 'next_n;
            }
        }

        primes.push(n);
    }

    'n_loop: for n in (1..=9).rev() {
        // generate all permutations of 123456789
        'next_i: for i in (0..((1..=n).product())).rev() {
            let num = nth_permutation((1..=n).collect(), i).iter()
                                                           .map(|d| d.to_string())
                                                           .collect::<String>()
                                                           .parse::<usize>()
                                                           .unwrap();

            for p in &primes {
                if num % p == 0 {
                    continue 'next_i;
                }
            }

            println!("{}", num);
            break 'n_loop;
        }
    }
}
