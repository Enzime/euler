fn main() {
    let mut primes = vec![2];

    'outer: for n in (3..).step_by(2) {
        for p in &primes {
            if n % p == 0 {
                continue 'outer;
            }
        }

        primes.push(n);

        if primes.len() == 10001 {
            break;
        }
    }
    
    println!("{:?}", primes);
}
