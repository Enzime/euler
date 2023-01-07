use euler_utils::nth_permutation;

fn main() {
    let mut primes = vec![2];
    let mut n = 1;

    'next_n: while primes[primes.len() - 1] < 20 {
        n += 2;

        for p in &primes {
            if n % p == 0 {
                continue 'next_n;
            }
        }

        primes.push(n);
    }

    let mut substring_divisible = vec![];

    // start at n = 9! to skip all the numbers beginning with 0
    'next_num: for n in (1..=9).product()..(1..=10).product() {
        let num = nth_permutation((0..=9).collect(), n).iter()
                                                       .map(|d| d.to_string())
                                                       .collect::<String>();

        for j in 1..=7 {
            let substring = &num[j..j+3].parse::<usize>().unwrap();

            if substring % primes[j - 1] != 0 {
                continue 'next_num;
            }
        }

        println!("{}", num);
        substring_divisible.push(num.parse::<usize>().unwrap());
    }

    println!("{}", substring_divisible.iter().sum::<usize>());
}
