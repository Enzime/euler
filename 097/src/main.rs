fn main() {
    println!("{}", (0..7830457).fold(28433u64, |acc, _| (acc*2) % 10_000_000_000) + 1);
}