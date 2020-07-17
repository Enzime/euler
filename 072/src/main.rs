fn main() {
    const MAX_D: usize = 1_000_000;
    let mut is_prime = vec![true; MAX_D + 1];
    let mut phi = vec![0; MAX_D + 1];

    is_prime[0] = false;
    is_prime[1] = false;

    let mut set_size = 0;

    for d in 2..=MAX_D {
        if is_prime[d] {
            phi[d] = d - 1;
        }

        set_size += phi[d];

        if !is_prime[d] {
            continue;
        }

        for k in (2*d..=MAX_D).step_by(d) {
            if phi[k] == 0 {
                phi[k] = k;
            }

            is_prime[k] = false;

            phi[k] *= d - 1;
            phi[k] /= d;
        }
    }

    println!("{}", set_size);
}
