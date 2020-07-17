use num_bigint::ToBigUint;

fn main() {
    println!("{}", (2..100).flat_map(|a: u64| (2..100).map(move |b| a.to_biguint()
                                                                     .unwrap()
                                                                     .pow(b)
                                                                     .to_string()
                                                                     .chars()
                                                                     .map(|c| c.to_digit(10).unwrap())
                                                                     .sum::<u32>()))
                           .max()
                           .unwrap());
}
