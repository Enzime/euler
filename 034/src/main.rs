fn main() {
    println!("{}", (3..10_000_000).map(|n| (n, n.to_string().chars().map(|d| (1..=d.to_digit(10).unwrap()).product::<u32>()).sum::<u32>())).filter(|(n, sum)| n == sum).map(|(n, _)| n).sum::<u32>());
}
