fn main() {
    println!("{:?}", (100..=999).flat_map(|x| (100..=999).map(move |y| x * y)).filter(|n| *n == n.to_string().chars().rev().collect::<String>().parse::<u32>().unwrap()).max().unwrap());
}
