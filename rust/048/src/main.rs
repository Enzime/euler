fn main() {
    println!("{}", (1..=1_000).map(|x| (0..x).fold(1, |acc, _| (acc * x) % 10usize.pow(10)))
                              .fold(0, |acc, x_x| (acc + x_x) % 10usize.pow(10)));
}
