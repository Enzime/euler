use num_bigint::{ToBigUint, BigUint};
use num_traits::One;

fn main() {
    println!("{}", (1..=100).map(|n| n.to_biguint().unwrap()).fold(One::one(), |acc: BigUint, n| acc * n).to_string().chars().map(|d| d.to_digit(10).unwrap()).sum::<u32>());
}
