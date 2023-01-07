fn main() {
    let mut fibonacci = vec![0, 1];

    while *fibonacci.last().unwrap() < 4_000_000 {
        let n = fibonacci.len();
        fibonacci.push(fibonacci[n - 1] + fibonacci[n - 2]);
    }

    println!("{}", fibonacci.iter().step_by(3).sum::<u32>());
}
