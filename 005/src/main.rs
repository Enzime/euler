fn gcd(a: u64, b: u64) -> u64 {
    if a > b {
        return gcd(b, a);
    }

    if a == 0 {
        return b;
    }

    return gcd(a, b % a);
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    println!("{}", (1..20).fold(1, lcm));
}
