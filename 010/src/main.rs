fn main() {
    let mut is_prime = [true; 2_000_000];

    is_prime[0] = false;
    is_prime[1] = false;

    // sieve
    for n in 2..is_prime.len() {
        if is_prime[n] {
            for m in (n*n..is_prime.len()).step_by(n) {
                is_prime[m] = false;
            }
        }
    }

    println!("{}", is_prime.iter().enumerate().filter(|(_, y)| **y).map(|(x, _)| x as u64).sum::<u64>());
}
