fn main() {
    println!("{}", (2u64..10_000_000u64).filter(|n| n.to_string().chars().map(|d| d.to_digit(10).unwrap().pow(5) as u64).sum::<u64>() == *n).sum::<u64>());
}
